use errors::OpenIAPError;
use tracing::{debug, error};

use openiap::{
    flow_service_client::FlowServiceClient, DownloadRequest, DownloadResponse, Envelope,
    QueryRequest, QueryResponse, SigninRequest, SigninResponse, UnWatchRequest, UploadRequest,
    UploadResponse, WatchRequest,
};
use openiap::{BeginStream, EndStream, ErrorResponse, Stream, WatchEvent, WatchResponse};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
// use tonic::transport::{channel, Channel, ClientTlsConfig};
use tonic::transport::{Channel, ClientTlsConfig};

pub mod openiap {
    tonic::include_proto!("openiap");
}
use tokio::sync::{mpsc, oneshot};
use tonic::Request;

pub mod download;
pub mod query;
pub mod queue;
pub mod signin;
pub mod upload;
use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod errors;

type QuerySender = oneshot::Sender<Envelope>;
type StreamSender = mpsc::Sender<Vec<u8>>;
#[derive(Debug, Clone)]
pub struct Client {
    pub inner: Arc<Mutex<ClientInner>>,
}
#[derive(Clone)]
pub struct ClientInner {
    pub client: FlowServiceClient<tonic::transport::Channel>,
    pub signedin: bool,
    pub connected: bool,
    pub stream_tx: mpsc::Sender<Envelope>,
    pub queries: Arc<Mutex<std::collections::HashMap<String, QuerySender>>>,
    pub streams: Arc<Mutex<std::collections::HashMap<String, StreamSender>>>,
    pub watches:
        Arc<Mutex<std::collections::HashMap<String, Box<dyn Fn(WatchEvent) + Send + Sync>>>>,
}
// implement debug for ClientInner
impl std::fmt::Debug for ClientInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientInner")
            .field("client", &self.client)
            .field("signedin", &self.signedin)
            .field("connected", &self.connected)
            .field("stream_tx", &self.stream_tx)
            .field("queries", &self.queries)
            .field("streams", &self.streams)
            .finish()
    }
}

fn generate_unique_filename(base: &str) -> PathBuf {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs();
    let filename = format!("{}_{}.tmp", base, timestamp);
    let dir = env::temp_dir();
    dir.join(filename)
}

fn move_file(from: &str, to: &str) -> std::io::Result<()> {
    // Attempt to rename the file first
    if let Err(_e) = std::fs::rename(from, to) {
        // If renaming fails due to a cross-device link error, fall back to copying
        std::fs::copy(from, to)?;
        std::fs::remove_file(from)?;
    }
    Ok(())
}
impl Client {
    #[tracing::instrument(level = "debug", target = "openiap::client", name = "connect")]
    pub async fn connect(dst: &str) -> Result<Self, OpenIAPError> {
        let mut strurl = dst.to_string();
        if strurl.is_empty() {
            strurl = std::env::var("apiurl").unwrap_or("".to_string());
        }
        if strurl.is_empty() {
            strurl = std::env::var("grpcapiurl").unwrap_or("".to_string());
        }
        if strurl.is_empty() {
            strurl = std::env::var("wsapiurl").unwrap_or("".to_string());
        }
        if strurl.is_empty() {
            return Err(OpenIAPError::ClientError("No URL provided".to_string()));
        }
        let url = url::Url::parse(strurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;
        if url.scheme() != "http" && url.scheme() != "https" && url.scheme() != "grpc" {
            return Err(OpenIAPError::ClientError("Invalid URL scheme".to_string()));
        }
        if url.scheme() == "grpc" {
            if url.port() == Some(443) {
                strurl = format!("https://{}", url.host_str().unwrap());
            } else {
                strurl = format!("http://{}", url.host_str().unwrap());
            }
        }
        let mut url = url::Url::parse(strurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;
        let mut username = "".to_string();
        let mut password = "".to_string();
        if url.username().is_empty() == false && url.password().is_none() == false {
            username = url.username().to_string();
            password = url.password().unwrap().to_string();
        }
        url = url::Url::parse(strurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;

        if url.port().is_none() {
            if url.scheme() == "https" {
                strurl = format!("https://{}", url.host_str().unwrap());
            } else {
                strurl = format!("http://{}", url.host_str().unwrap());
            }
        } else {
            strurl = format!("http://{}:{}", url.host_str().unwrap(), url.port().unwrap());
        }
        println!("Connecting to {}", strurl);

        let innerclient;
        if url.scheme() == "http" {
            let response = FlowServiceClient::connect(strurl).await;
            match response {
                Ok(client) => {
                    innerclient = client;
                }
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to connect: {}",
                        e
                    )));
                }
            }
        } else {
            let tls = ClientTlsConfig::new()
                .with_webpki_roots()
                .domain_name(url.host().unwrap().to_string());
            let uri = tonic::transport::Uri::builder()
                .scheme(url.scheme())
                .authority(url.host().unwrap().to_string())
                .path_and_query("/")
                .build();
            let uri = match uri {
                Ok(uri) => uri,
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to build URI: {}",
                        e
                    )));
                }
            };
            let channel = Channel::builder(uri).tls_config(tls);
            let channel = match channel {
                Ok(channel) => channel,
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to build channel: {}",
                        e
                    )));
                }
            };
            let channel = channel.connect().await;
            let channel = match channel {
                Ok(channel) => channel,
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to connect: {}",
                        e
                    )));
                }
            };
            innerclient = FlowServiceClient::new(channel);
            // let response = FlowServiceClient::connect(strurl).await;
            // match channel {
            //     Ok(client) => {
            //         innerclient = FlowServiceClient::new(channel);
            //     }
            //     Err(e) => {
            //         return Err(OpenIAPError::ClientError(format!("Failed to connect: {}", e)));
            //     }
            // }
        }

        let (stream_tx, stream_rx) = mpsc::channel(4);
        let in_stream = ReceiverStream::new(stream_rx);

        let inner = ClientInner {
            client: innerclient,
            signedin: false,
            connected: false,
            stream_tx,
            queries: Arc::new(Mutex::new(std::collections::HashMap::new())),
            streams: Arc::new(Mutex::new(std::collections::HashMap::new())),
            watches: Arc::new(Mutex::new(std::collections::HashMap::new())),
        };

        let client = Client {
            inner: Arc::new(Mutex::new(inner)),
        };
        client.ping().await;
        client.setup_stream(in_stream).await?;
        if username.is_empty() == true && password.is_empty() == true {
            username = std::env::var("OPENIAP_USERNAME").unwrap_or_default();
            password = std::env::var("OPENIAP_PASSWORD").unwrap_or_default();
        }
        if username.is_empty() == false && password.is_empty() == false {
            println!("Signing in with username: {}", username);
            let signin = SigninRequest::with_userpass(username.as_str(), password.as_str());
            let loginresponse = client.signin(signin).await;
            match loginresponse {
                Ok(response) => {
                    println!("Signed in as {}", response.user.as_ref().unwrap().username);
                }
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to sign in: {}",
                        e
                    )));
                }
            }
        }
        Ok(client)
    }
    #[tracing::instrument(level = "debug", target = "openiap::client", name = "setup_stream")]
    async fn setup_stream(&self, in_stream: ReceiverStream<Envelope>) -> Result<(), OpenIAPError> {
        let mut inner = self.inner.lock().await;
        let response = inner.client.setup_stream(Request::new(in_stream)).await;
        let response = match response {
            Ok(response) => response,
            Err(e) => {
                return Err(OpenIAPError::ClientError(format!(
                    "Failed to setup stream: {}",
                    e
                )));
            }
        };

        inner.connected = true;
        let mut resp_stream = response.into_inner();
        let inner = self.inner.clone(); // Clone the Arc to extend its lifetime
        tokio::spawn(async move {
            while let Some(received) = resp_stream.next().await {
                if let Ok(received) = received {
                    let command = received.command.clone();
                    let rid = received.rid.clone();
                    let inner = inner.lock().await;
                    let mut queries = inner.queries.lock().await;
                    let mut streams = inner.streams.lock().await;
                    let watches = inner.watches.lock().await;
                    debug!("Received #{} #{} {} message", received.id, rid, command);
                    if command == "ping" {
                        let envelope = Envelope {
                            command: "pong".into(),
                            ..Default::default()
                        };
                        match inner.stream_tx.send(envelope).await {
                            Ok(_) => _ = (),
                            Err(e) => error!("Failed to send data: {}", e),
                        }
                    } else if command == "beginstream"
                        || command == "stream"
                        || command == "endstream"
                    {
                        let streamresponse: Stream =
                            prost::Message::decode(received.data.unwrap().value.as_ref()).unwrap();
                        let streamdata = streamresponse.data;

                        if streamdata.len() > 0 {
                            let stream = streams.get(rid.as_str()).unwrap();

                            match stream.send(streamdata).await {
                                Ok(_) => _ = (),
                                Err(e) => error!("Failed to send data: {}", e),
                            }
                        }

                        if command == "endstream" {
                            let _ = streams.remove(rid.as_str());
                        }
                    } else if command == "watchevent" {
                        let watchevent: WatchEvent =
                            prost::Message::decode(received.data.unwrap().value.as_ref()).unwrap();
                        if let Some(callback) = watches.get(watchevent.id.as_str()) {
                            callback(watchevent);
                        }
                    } else if let Some(response_tx) = queries.remove(&rid) {
                        let stream = streams.get(rid.as_str());
                        match stream {
                            Some(stream) => {
                                let streamdata = vec![];
                                match stream.send(streamdata).await {
                                    Ok(_) => _ = (),
                                    Err(e) => error!("Failed to send data: {}", e),
                                }
                            }
                            None => (),
                        }
                        // Send to response to waiting call
                        let _ = response_tx.send(received);
                    } else {
                        println!("Received unhandled {} message: {:?}", command, received);
                    }
                }
            }
        });
        Ok(())
    }
    #[allow(dead_code, unused_variables)]
    pub fn set_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        // self.callback = Some(callback);
    }

    fn get_id(&self) -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
    async fn send(&self, msg: Envelope) -> Result<Envelope, OpenIAPError> {
        let response = self.send_noawait(msg).await;
        match response {
            Ok((response_rx, _)) => {
                let response = response_rx.await;
                match response {
                    Ok(response) => Ok(response),
                    Err(e) => Err(OpenIAPError::CustomError(e.to_string())),
                }
            }
            Err(e) => Err(OpenIAPError::CustomError(e.to_string())),
        }
    }
    #[tracing::instrument(level = "debug", target = "openiap::client", name = "send_noawait")]
    async fn send_noawait(
        &self,
        mut msg: Envelope,
    ) -> Result<(oneshot::Receiver<Envelope>, String), OpenIAPError> {
        {
            let inner = self.inner.lock().await;
            if inner.connected == false {
                return Err(OpenIAPError::ClientError("Not connected".to_string()));
            }
        }
        let (response_tx, response_rx) = oneshot::channel();
        let id = self.get_id().to_string();
        msg.id = id.clone();
        {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.insert(id.clone(), response_tx);
            let res = inner.stream_tx.send(msg).await;
            match res {
                Ok(_) => (),
                Err(e) => return Err(OpenIAPError::ClientError(e.to_string())),
            }
        }
        Ok((response_rx, id))
    }
    async fn sendwithstream(
        &self,
        mut msg: Envelope,
    ) -> Result<(oneshot::Receiver<Envelope>, mpsc::Receiver<Vec<u8>>), OpenIAPError> {
        {
            let inner = self.inner.lock().await;
            if !inner.connected {
                return Err(OpenIAPError::ClientError("Not connected".to_string()));
            }
        }
        let (response_tx, response_rx) = oneshot::channel();
        let (stream_tx, stream_rx) = mpsc::channel(1024 * 1024);
        let id = self.get_id().to_string();
        msg.id = id.clone();
        {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.insert(id.clone(), response_tx);
            inner.streams.lock().await.insert(id.clone(), stream_tx);
            let res = inner.stream_tx.send(msg).await;
            if res.is_err() {
                let e = res.err().unwrap();
                return Err(OpenIAPError::ClientError(e.to_string()));
            }
        }
        Ok((response_rx, stream_rx))
    }
    #[allow(dead_code)]
    async fn ping(&self) {
        let envelope = Envelope {
            command: "ping".into(),
            ..Default::default()
        };
        let inner = self.inner.lock().await;
        if inner.stream_tx.send(envelope).await.is_err() == true {
            error!("Failed to send ping");
        }
    }
    pub async fn signin(&self, mut config: SigninRequest) -> Result<SigninResponse, OpenIAPError> {
        if config.username.is_empty() {
            config.username = std::env::var("OPENIAP_USERNAME").unwrap_or_default();
        }
        if config.password.is_empty() {
            config.password = std::env::var("OPENIAP_PASSWORD").unwrap_or_default();
        }
        if config.jwt.is_empty() {
            config.jwt = std::env::var("OPENIAP_JWT").unwrap_or_default();
        }
        if config.jwt.is_empty() {
            config.jwt = std::env::var("JWT").unwrap_or_default();
        }
        let version = env!("CARGO_PKG_VERSION");
        if !version.is_empty() && config.version.is_empty() {
            config.version = version.to_string();
        }
        if config.agent.is_empty() {
            config.agent = "rust".to_string();
        }

        debug!("Attempting sign-in using {:?}", config);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;

        match &result {
            Ok(m) => {
                debug!("Sign-in reply received");
                let mut inner = self.inner.lock().await;
                if m.command == "error" {
                    let e: ErrorResponse =
                        prost::Message::decode(m.data.as_ref().unwrap().value.as_ref())
                            .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(e.message));
                }
                inner.signedin = true;
                debug!("Sign-in successful");
                let response: SigninResponse =
                    prost::Message::decode(m.data.as_ref().unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => {
                debug!("Sending Sign-in request failed {:?}", result);
                debug!("Sign-in failed: {}", e.to_string());
                Err(OpenIAPError::ClientError(e.to_string()))
            }
        }
    }

    pub async fn query(&self, mut config: QueryRequest) -> Result<QueryResponse, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }

        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: QueryResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[no_mangle]
    pub async fn download(
        &self,
        config: DownloadRequest,
        folder: Option<&str>,
        filename: Option<&str>,
    ) -> Result<DownloadResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        match self.sendwithstream(envelope).await {
            Ok((response_rx, mut stream_rx)) => {
                let temp_file_path = generate_unique_filename("openiap");
                println!("Temp file: {:?}", temp_file_path);
                let mut temp_file = File::create(&temp_file_path).map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to create temp file: {}", e))
                })?;
                while stream_rx.is_closed() == false {
                    match stream_rx.recv().await {
                        Some(received) => {
                            if received.len() == 0 {
                                debug!("Stream closed");
                                break;
                            }
                            debug!("Received {} bytes", received.len());
                            temp_file.write_all(&received).map_err(|e| {
                                OpenIAPError::ClientError(format!(
                                    "Failed to write to temp file: {}",
                                    e
                                ))
                            })?;
                        }
                        None => {
                            debug!("Stream closed");
                            break;
                        }
                    }
                }
                temp_file.sync_all().map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to sync temp file: {}", e))
                })?;

                let response = response_rx.await.map_err(|_| {
                    OpenIAPError::ClientError("Failed to receive response".to_string())
                })?;

                if response.command == "error" {
                    let e: ErrorResponse =
                        prost::Message::decode(response.data.unwrap().value.as_ref()).unwrap();
                    return Err(OpenIAPError::ServerError(e.message));
                }
                let mut downloadresponse: DownloadResponse =
                    prost::Message::decode(response.data.unwrap().value.as_ref()).unwrap();

                let mut final_filename = match &filename {
                    Some(f) => f,
                    None => downloadresponse.filename.as_str(),
                };
                if final_filename.is_empty() {
                    final_filename = downloadresponse.filename.as_str();
                }
                let mut folder = match &folder {
                    Some(f) => f,
                    None => ".",
                };
                if folder.is_empty() {
                    folder = ".";
                }
                let filepath = format!("{}/{}", folder, final_filename);
                println!("Moving file to {}", filepath);
                move_file(temp_file_path.to_str().unwrap(), filepath.as_str()).map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to move file: {}", e))
                })?;
                println!("Downloaded file to {}", filepath);
                downloadresponse.filename = filepath;

                Ok(downloadresponse)
            }
            Err(status) => Err(OpenIAPError::ClientError(status.to_string())),
        }
    }
    pub async fn upload(
        &self,
        config: UploadRequest,
        filepath: &str,
    ) -> Result<UploadResponse, OpenIAPError> {
        println!("Rust::upload: Uploading file: {}", filepath);
        let mut file = File::open(filepath)
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to open file: {}", e)))?;
        let chunk_size = 1024 * 1024; // 1 MB
        let mut buffer = vec![0; chunk_size];

        let envelope = config.to_envelope();
        let (response_rx, rid) = self.send_noawait(envelope).await?;
        {
            let inner = self.inner.lock().await;

            let envelope = BeginStream::from_rid(rid.clone());
            debug!("Sending beginstream to #{}", rid);
            inner
                .stream_tx
                .send(envelope)
                .await
                .map_err(|e| OpenIAPError::ClientError(format!("Failed to send data: {}", e)))?;
            let mut counter = 0;

            loop {
                let bytes_read = file.read(&mut buffer).map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to read from file: {}", e))
                })?;
                counter += 1;

                if bytes_read == 0 {
                    break;
                }

                let chunk = buffer[..bytes_read].to_vec();
                let envelope = Stream::from_rid(chunk, rid.clone());
                debug!("Sending chunk {} stream to #{}", counter, envelope.rid);
                inner.stream_tx.send(envelope).await.map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to send data: {}", e))
                })?;
            }

            let envelope = EndStream::from_rid(rid.clone());
            debug!("Sending endstream to #{}", rid);
            inner
                .stream_tx
                .send(envelope)
                .await
                .map_err(|e| OpenIAPError::ClientError(format!("Failed to send data: {}", e)))?;
        }

        debug!("Wait for upload response for #{}", rid);
        match response_rx.await {
            Ok(response) => {
                if response.command == "error" {
                    let error_response: ErrorResponse = prost::Message::decode(
                        response.data.unwrap().value.as_ref(),
                    )
                    .map_err(|e| {
                        OpenIAPError::ClientError(format!("Failed to decode ErrorResponse: {}", e))
                    })?;
                    return Err(OpenIAPError::ServerError(error_response.message));
                }

                let upload_response: UploadResponse =
                    prost::Message::decode(response.data.unwrap().value.as_ref()).map_err(|e| {
                        OpenIAPError::ClientError(format!("Failed to decode UploadResponse: {}", e))
                    })?;
                Ok(upload_response)
            }
            Err(e) => Err(OpenIAPError::CustomError(e.to_string())),
        }
    }
    pub async fn watch(
        &self,
        mut config: WatchRequest,
        callback: Box<dyn Fn(WatchEvent) + Send + Sync>,
    ) -> Result<String, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }
        if config.paths.is_empty() {
            config.paths = vec!["".to_string()];
        }

        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: WatchResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;

                let inner = self.inner.lock().await;
                inner
                    .watches
                    .lock()
                    .await
                    .insert(response.id.clone(), callback);

                Ok(response.id)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    pub async fn unwatch(&self, id: &str) -> Result<(), OpenIAPError> {
        let config = UnWatchRequest::byid(id);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
}

#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin + Clone>() {}
#[cfg(test)]
mod tests {
    use std::{future::Future, pin::Pin};
    use futures::stream::FuturesUnordered;

    use super::*;
    #[allow(dead_code)]
    // const TEST_URL: &str = "http://localhost:50051";
    // const TEST_URL: &str = "http://grpc.demo.openiap.io";
    const TEST_URL: &str = "";
    #[test]
    fn normal_type() {
        is_normal::<Client>();
        is_normal::<ClientInner>();
        is_normal::<SigninRequest>();
        is_normal::<SigninResponse>();
        is_normal::<QueryRequest>();
        is_normal::<QueryResponse>();
        is_normal::<DownloadRequest>();
        is_normal::<DownloadResponse>();
        is_normal::<UploadRequest>();
        is_normal::<UploadResponse>();
        is_normal::<BeginStream>();
        is_normal::<Stream>();
        is_normal::<EndStream>();
    }

    #[tokio::test()]
    async fn test_query() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let query = QueryRequest {
            query: "{}".to_string(),
            projection: "{\"name\": 1}".to_string(),
            ..Default::default()
        };
        let response = client.query(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_multiple_query() {
        // cargo test test_multiple_query -- --nocapture
        let client = Client::connect(TEST_URL).await.unwrap();
        let tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Result<QueryResponse, OpenIAPError>>>>>::new();
        for _ in 1..101 {
                let query = QueryRequest {
                    query: "{}".to_string(),
                    projection: "{\"name\": 1}".to_string(),
                    ..Default::default()
                };
                tasks.push(Box::pin(client.query(query)));
        }
        // while let Some(result) = tasks.next().await {
        //     println!("{}", result);
        // }
        let result = futures::future::join_all(tasks).await;
        println!("{:?}", result);
    }

    #[tokio::test()]
    async fn test_bad_login() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let response = client
            .signin(SigninRequest::with_userpass("testuser", "badpassword"))
            .await;
        assert!(response.is_err(), "login with bad password, did not fail");
    }
    #[tokio::test()]
    async fn test_upload() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let response = client
            .upload(UploadRequest::filename("rust-test.csv"), "testfile.csv")
            .await;
        assert!(
            !response.is_err(),
            "Upload of testfile.csv failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_upload_as_guest() {
        let client = Client::connect(TEST_URL).await.unwrap();
        client
            .signin(SigninRequest::with_userpass("guest", "password"))
            .await
            .unwrap();
        let response = client
            .upload(UploadRequest::filename("rust-test.csv"), "testfile.csv")
            .await;
        assert!(
            !response.is_err(),
            "Upload of testfile.csv did not fail as guest"
        );
    }
    #[tokio::test()]
    async fn test_download() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let response = client
            .download(DownloadRequest::id("65a3aaf66d52b8c15131aebd"), None, None)
            .await;
        println!("Download response: {:?}", response);
        assert!(
            !response.is_err(),
            "Download of file failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_download_as_guest() {
        let client = Client::connect(TEST_URL).await.unwrap();
        client
            .signin(SigninRequest::with_userpass("guest", "password"))
            .await
            .unwrap();
        let response = client
            .download(DownloadRequest::id("65a3aaf66d52b8c15131aebd"), None, None)
            .await;
        println!("Download response: {:?}", response);
        assert!(
            response.is_err(),
            "Download of file as guest did not failed"
        );
    }
}
