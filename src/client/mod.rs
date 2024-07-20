use tracing::{debug, error};

use openiap::{BeginStream, EndStream, ErrorResponse, Stream};
use openiap::{
    flow_service_client::FlowServiceClient, DownloadRequest, DownloadResponse, Envelope,
    QueryRequest, QueryResponse, SigninRequest, SigninResponse, UploadRequest, UploadResponse,
};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;
use std::fs::{File};
use std::io::{Read, Write};
use tokio::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tonic::transport::{Channel, ClientTlsConfig};


pub mod openiap {
    tonic::include_proto!("openiap");
}
use tokio::sync::{mpsc, oneshot};
use tonic::{Request};

pub mod download;
pub mod query;
pub mod signin;
pub mod upload;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;

type QuerySender = oneshot::Sender<Envelope>;
type StreamSender = mpsc::Sender<Vec<u8>>;
#[derive(Debug, Clone)]
pub struct Client {
    pub inner: Arc<Mutex<ClientInner>>,
}
#[derive(Debug, Clone)]
pub struct ClientInner {
    client: FlowServiceClient<tonic::transport::Channel>,
    signedin: bool,
    connected: bool,
    stream_tx: mpsc::Sender<Envelope>,
    queries: Arc<Mutex<std::collections::HashMap<String, QuerySender>>>,
    streams: Arc<Mutex<std::collections::HashMap<String, StreamSender>>>,
}

fn generate_unique_filename(base: &str) -> PathBuf {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
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
    pub async fn connect(dst: &str) -> Result<Self>
    {
        // tracing_subscriber::fmt::fmt()
        // .with_max_level(tracing::Level::DEBUG)
        // .init();

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
            return Err(Box::new(tonic::Status::cancelled("No URL provided")));
        }
        let url = url::Url::parse(strurl.as_str()).map_err(|e| {
            tonic::Status::cancelled(format!("Failed to parse URL: {}", e))
        })?;
        if url.scheme() != "http" && url.scheme() != "https" && url.scheme() != "grpc" {
            return Err(Box::new(tonic::Status::cancelled("Invalid URL scheme")));
        }
        if url.scheme() == "grpc" {
            if url.port() == Some(443) {
                strurl = format!("https://{}", url.host_str().unwrap());
            } else {
                strurl = format!("http://{}", url.host_str().unwrap());
            }
        }
        let mut url = url::Url::parse(strurl.as_str()).map_err(|e| {
            tonic::Status::cancelled(format!("Failed to parse URL: {}", e))
        })?;
        if url.port().is_none() {
            if url.scheme() == "https" {
                strurl = format!("{}:{}", strurl, 443);
            } else {
                strurl = format!("{}:{}", strurl, 80);
            }
        }
        let mut username = "".to_string();
        let mut password= "".to_string();
        if url.username().is_empty() == false && url.password().is_none() == false {
            username = url.username().to_string();
            password = url.password().unwrap().to_string();
        } 
        url = url::Url::parse(strurl.as_str()).map_err(|e| {
            tonic::Status::cancelled(format!("Failed to parse URL: {}", e))
        })?;

        if url.port().is_none() {
            if url.scheme() == "https" {
                strurl = format!("https://{}:443", url.host_str().unwrap());
            } else {
                strurl = format!("http://{}:80", url.host_str().unwrap());
            }
        } else {
            strurl = format!("http://{}:{}", url.host_str().unwrap(), url.port().unwrap());
        }
        println!("Connecting to {}", strurl);
        
        let innerclient;
        if url.scheme() == "http" {
            innerclient = FlowServiceClient::connect(strurl).await?;
        } else {            
            let tls = ClientTlsConfig::new()
            .with_webpki_roots()
            .domain_name(url.host().unwrap().to_string());
            let uri = tonic::transport::Uri::builder()
                .scheme(url.scheme())
                .authority(url.host().unwrap().to_string() )
                .path_and_query("/")
                .build()?;
            let channel = Channel::builder(uri)
                .tls_config(tls)?
                .connect()
                .await?;
    
            innerclient = FlowServiceClient::new(channel);
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
        };

        let client = Client {
            inner: Arc::new(Mutex::new(inner)),
        };
        client.ping().await;
        client.setup_stream(in_stream).await?;
        if username.is_empty() == false && password.is_empty() == false {
            println!("Signing in with username: {}", username);
            let signin = SigninRequest::with_userpass(username.as_str(), password.as_str());
            let _ = client.signin(signin);
        }                
        Ok(client)
    }
    #[tracing::instrument(level = "debug", target = "openiap::client", name = "setup_stream")]
    async fn setup_stream(&self, in_stream: ReceiverStream<Envelope>) -> Result<()> {
        let mut inner = self.inner.lock().await;
        let response = inner.client.setup_stream(Request::new(in_stream)).await?;
        inner.connected = true;
        let mut resp_stream = response.into_inner();
        let inner = self.inner.clone(); // Clone the Arc to extend its lifetime
        tokio::spawn(async move {
            while let Some(received) = resp_stream.next().await {
                if let Ok(received) = received {
                    let inner = inner.lock().await;
                    let mut queries = inner.queries.lock().await;
                    debug!(
                        "Received #{} #{} {} message",
                        received.id, received.rid, received.command
                    );
                    if received.command == "ping" {
                        let envelope = Envelope {
                            command: "pong".into(),
                            ..Default::default()
                        };
                        // let _ = inner.stream_tx.send(envelope).await;
                        match inner.stream_tx.send(envelope).await {
                            Ok(_) => _ = (),
                            Err(e) => error!("Failed to send data: {}", e),
                        }

                    } else if received.command == "beginstream"
                        || received.command == "stream"
                        || received.command == "endstream"
                    {
                        let streamresponse: Stream =
                            prost::Message::decode(received.data.unwrap().value.as_ref()).unwrap();
                        let streamdata = streamresponse.data;

                        let command = received.command.clone();
                        let rid = received.rid.clone();
                        let mut streams = inner.streams.lock().await;
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
                    } else if let Some(response_tx) = queries.remove(&received.rid) {
                        // Send to response to waiting call
                        let _ = response_tx.send(received);
                    } else {
                        println!("Received unhandled {} message: {:?}", received.command, received);
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
    async fn send(&self, msg: Envelope) -> Result<Envelope, tonic::Status> {
        let (response_rx, _id) = self.send_noawait(msg).await?;
        response_rx
            .await
            .map_err(|_| tonic::Status::cancelled("Failed to receive response"))
    }
    #[tracing::instrument(level = "debug", target = "openiap::client", name = "send_noawait")]
    async fn send_noawait(&self, mut msg: Envelope) -> Result<( oneshot::Receiver<Envelope>, String), tonic::Status> {
        {
            let inner = self.inner.lock().await;
            if inner.connected == false {
                return Err(tonic::Status::cancelled("Not connected"));
            }
        }
        let (response_tx, response_rx) = oneshot::channel();
        let id = self.get_id().to_string();
        msg.id = id.clone();
        {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.insert(id.clone(), response_tx);
            let res = inner.stream_tx.send(msg).await;
            if res.is_err() {
                let e = res.err().unwrap();
                return Err(tonic::Status::cancelled(e.to_string()));
            }
        }
        Ok((response_rx,id))
    }
    async fn sendwithstream(
        &self,
        mut msg: Envelope,
    ) -> Result<(oneshot::Receiver<Envelope>, mpsc::Receiver<Vec<u8>>), tonic::Status> {
        {
            let inner = self.inner.lock().await;
            if !inner.connected {
                return Err(tonic::Status::cancelled("Not connected"));
            }
        }
        let (response_tx, response_rx) = oneshot::channel();
        let (stream_tx, stream_rx) = mpsc::channel(1024*1024);
        let id = self.get_id().to_string();
        msg.id = id.clone();
        {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.insert(id.clone(), response_tx);
            inner.streams.lock().await.insert(id.clone(), stream_tx);
            let res = inner.stream_tx.send(msg).await;
            if res.is_err() {
                let e = res.err().unwrap();
                return Err(tonic::Status::cancelled(e.to_string()));
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
    pub async fn signin(&self, mut config: SigninRequest) -> Result<SigninResponse, tonic::Status> {
        if config.username.is_empty() {
            config.username = std::env::var("OPENIAP_USERNAME").unwrap_or("".to_string());
        }
        if config.password.is_empty() {
            config.password = std::env::var("OPENIAP_PASSWORD").unwrap_or("".to_string());
        }
        if config.jwt.is_empty() {
            config.jwt = std::env::var("OPENIAP_JWT").unwrap_or("".to_string());
        }
        if config.jwt.is_empty() {
            config.jwt = std::env::var("JWT").unwrap_or("".to_string());
        }
        let version = env!("CARGO_PKG_VERSION");
        if version.is_empty() == false && config.version.is_empty() {
            config.version = version.to_string();
        }
        if config.agent.is_empty() {
            config.agent = "rust".to_string();
        }

        debug!("Attempting sign-in using {:?}", config);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        if result.is_ok() {
            debug!("Sign-in reply received");
            let mut inner = self.inner.lock().await;
            let m = result.unwrap();
            if m.command == "error" {
                let e: ErrorResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref()).unwrap();
                return Err(tonic::Status::cancelled(e.message));
            }
            inner.signedin = true;
            debug!("Sign-in successful");
            Ok(prost::Message::decode(m.data.unwrap().value.as_ref()).unwrap())
        } else {
            debug!("Sending Sign-in request failed {:?}", result);
            let e = result.err().unwrap();
            debug!("Sign-in failed: {}", e.to_string());
            Err(tonic::Status::cancelled(e.to_string()))
        }
    }
    pub async fn query(&self, config: QueryRequest) -> Result<QueryResponse, tonic::Status> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        if result.is_ok() {
            let m = result.unwrap();
            if m.command == "error" {
                let e: ErrorResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref()).unwrap();
                return Err(tonic::Status::cancelled(e.message));
            }
            Ok(prost::Message::decode(m.data.unwrap().value.as_ref()).unwrap())
        } else {
            let e = result.err().unwrap();
            Err(tonic::Status::cancelled(e.to_string()))
        }
    }
    pub async fn download(
        &self,
        config: DownloadRequest,
        folder: Option<&str>,
        filename: Option<&str>
    ) -> Result<DownloadResponse, tonic::Status> {
        let envelope = config.to_envelope();
        match self.sendwithstream(envelope).await {
            Ok((response_rx, mut stream_rx)) => {
                let temp_file_path = generate_unique_filename("openiap");
                let mut temp_file = File::create(&temp_file_path).map_err(|e| {
                    tonic::Status::internal(format!("Failed to create temp file: {}", e))
                })?;
                while stream_rx.is_closed() == false {
                    match stream_rx.recv().await {
                        Some(received) => {
                            temp_file.write_all(&received).map_err(|e| {
                                tonic::Status::internal(format!("Failed to write to temp file: {}", e))
                            })?;
                        }
                        None => {
                            break;
                        }
                    }                    
                }
                temp_file.sync_all().map_err(|e| {
                    tonic::Status::internal(format!("Failed to sync temp file: {}", e))
                })?;

                let response = response_rx
                    .await
                    .map_err(|_| tonic::Status::cancelled("Failed to receive response"))?;

                if response.command == "error" {
                    let e: ErrorResponse =
                        prost::Message::decode(response.data.unwrap().value.as_ref()).unwrap();
                    return Err(tonic::Status::cancelled(e.message));
                }
                let mut downloadresponse: DownloadResponse =
                    prost::Message::decode(response.data.unwrap().value.as_ref()).unwrap();


                let final_filename = match &filename {
                    Some(f) => f,
                    None => downloadresponse.filename.as_str()
                };
                let folder = match &folder {
                    Some(f) => f,
                    None => "."
                };
                let filepath = format!("{}/{}", folder, final_filename);
                move_file(temp_file_path.to_str().unwrap(), filepath.as_str()).map_err(|e| {
                    tonic::Status::internal(format!("Failed to rename temp file: {}", e))
                })?;
                downloadresponse.filename = filepath;

                Ok(downloadresponse)
            }
            Err(status) => Err(tonic::Status::cancelled(status.message())),
        }
    }
    pub async fn upload(
        &self,
        config: UploadRequest,
        filepath: &str
    ) -> Result<UploadResponse, tonic::Status> {

        let mut file = File::open(filepath).map_err(|e| {
            tonic::Status::internal(format!("Failed to open file: {}", e))
        })?;
        let chunk_size = 1024 * 1024; // 1 MB
        let mut buffer = vec![0; chunk_size];
    
        let envelope = config.to_envelope();
        let (response_rx, rid) = self.send_noawait(envelope).await?;
        {
            let inner = self.inner.lock().await;
            
            let envelope = BeginStream::from_rid(rid.clone());
            debug!("Sending beginstream to #{}", rid);
            inner.stream_tx.send(envelope).await.map_err(|e| {
                tonic::Status::internal(format!("Failed to send data: {}", e))
            })?;
            let mut counter = 0;

            loop {
                let bytes_read = file.read(&mut buffer).map_err(|e| {
                    tonic::Status::internal(format!("Failed to read file chunk: {}", e))
                })?;
                counter += 1;
                
                if bytes_read == 0 {
                    break;
                }
            
                let chunk = buffer[..bytes_read].to_vec();
                let envelope = Stream::from_rid(chunk, rid.clone());
                debug!("Sending chunk {} stream to #{}", counter, envelope.rid);
                inner.stream_tx.send(envelope).await.map_err(|e| {
                    tonic::Status::internal(format!("Failed to send data: {}", e))
                })?;

            }
    
            let envelope = EndStream::from_rid(rid.clone());
            debug!("Sending endstream to #{}", rid);
            inner.stream_tx.send(envelope).await.map_err(|e| {
                tonic::Status::internal(format!("Failed to send data: {}", e))
            })?;
        }

        debug!("Wait for upload response for #{}", rid);
        match response_rx.await {
            Ok(response) => {
                if response.command == "error" {
                    let error_response: ErrorResponse = prost::Message::decode(response.data.unwrap().value.as_ref())
                        .map_err(|e| tonic::Status::internal(format!("Failed to decode ErrorResponse: {}", e)))?;
                    return Err(tonic::Status::cancelled(error_response.message));
                }
    
                let upload_response: UploadResponse = prost::Message::decode(response.data.unwrap().value.as_ref())
                    .map_err(|e| tonic::Status::internal(format!("Failed to decode UploadResponse: {}", e)))?;
                Ok(upload_response)
            }
            Err(_) => Err(tonic::Status::cancelled("Failed to receive response")),
        }
        // Ok(UploadResponse::default())

    }
}

#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin + Clone>() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normal_type() {
        is_normal::<Client>();
        is_normal::<ClientInner>();
    }
}


