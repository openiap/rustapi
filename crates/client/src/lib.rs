pub use openiap_proto::errors::*;
pub use openiap_proto::protos::*;
pub use openiap_proto::*;
pub use protos::flow_service_client::FlowServiceClient;
pub use prost_types::Timestamp;


// use openiap_proto::errors::OpenIAPError;
// use openiap_proto::openiap::{
//     flow_service_client::FlowServiceClient, DownloadRequest, DownloadResponse, Envelope,
//     QueryRequest, QueryResponse, SigninRequest, SigninResponse, UnWatchRequest, UploadRequest,
//     UploadResponse, WatchRequest, AggregateRequest, AggregateResponse, InsertOneRequest, InsertOneResponse,
//     DistinctRequest, DistinctResponse,
//     CountRequest, CountResponse, InsertManyRequest, InsertManyResponse,
//     BeginStream, EndStream, ErrorResponse, Stream, WatchEvent, WatchResponse,
// };
use tracing::{debug, error, info, trace};

use tokio_stream::{wrappers::ReceiverStream, StreamExt};
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use tokio::sync::{mpsc, oneshot};
use tonic::Request;

use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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
    pub queues:
        Arc<Mutex<std::collections::HashMap<String, Box<dyn Fn(QueueEvent) + Send + Sync>>>>,
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
#[tracing::instrument]
fn move_file(from: &str, to: &str) -> std::io::Result<()> {
    // Attempt to rename the file first
    if let Err(_e) = std::fs::rename(from, to) {
        // If renaming fails due to a cross-device link error, fall back to copying
        std::fs::copy(from, to)?;
        std::fs::remove_file(from)?;
    }
    Ok(())
}

// use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use flate2::write::GzEncoder;
use flate2::Compression;
#[allow(dead_code)]
fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // Create the output file
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);

    // Create a GzEncoder with default compression level
    let mut encoder = GzEncoder::new(writer, Compression::default());

    // Buffer to hold the data
    let mut buffer = Vec::new();

    // Read the entire input file into the buffer
    reader.read_to_end(&mut buffer)?;

    // Write the compressed data to the output file
    encoder.write_all(&buffer)?;

    // Finalize the compression
    encoder.finish()?;

    Ok(())
}
pub fn compress_file_to_vec(input_path: &str) -> io::Result<::prost::alloc::vec::Vec<u8>> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    // Buffer to hold the input data
    let mut buffer = Vec::new();

    // Read the entire input file into the buffer
    reader.read_to_end(&mut buffer)?;

    // Create a GzEncoder to compress data into a Vec<u8>
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    // Write the compressed data to the encoder
    encoder.write_all(&buffer)?;

    // Finalize the compression and get the compressed data
    let compressed_data = encoder.finish()?;

    Ok(compressed_data)
}
impl Client {
    #[tracing::instrument(skip_all)]
    pub async fn connect(dst: &str) -> Result<Self, OpenIAPError> {
        let mut strurl = dst.to_string();
        if strurl.is_empty() {
            strurl = std::env::var("apiurl").unwrap_or("".to_string());
            if strurl.is_empty() {
                strurl = std::env::var("grpcapiurl").unwrap_or("".to_string());
            }
            if strurl.is_empty() {
                strurl = std::env::var("wsapiurl").unwrap_or("".to_string());
            }
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
        info!("Connecting to {}", strurl);

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
            // let channel = Channel::builder(uri).tls_config(tls);
            let channel = Channel::builder(uri)
                .tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots());
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
            queues: Arc::new(Mutex::new(std::collections::HashMap::new())),
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
            debug!("Signing in with username: {}", username);
            let signin = SigninRequest::with_userpass(username.as_str(), password.as_str());
            let loginresponse = client.signin(signin).await;
            match loginresponse {
                Ok(response) => {
                    debug!("Signed in as {}", response.user.as_ref().unwrap().username);
                }
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to sign in: {}",
                        e
                    )));
                }
            }
        } else {
            let mut jwt = std::env::var("OPENIAP_JWT").unwrap_or_default();
            if jwt.is_empty() {
                jwt = std::env::var("jwt").unwrap_or_default();
            }
            if jwt.is_empty() == false {
                debug!("Signing in with JWT");
                let signin = SigninRequest::with_jwt(jwt.as_str());
                let loginresponse = client.signin(signin).await;
                match loginresponse {
                    Ok(response) => {
                        debug!("Signed in as {}", response.user.as_ref().unwrap().username);
                    }
                    Err(e) => {
                        return Err(OpenIAPError::ClientError(format!(
                            "Failed to sign in: {}",
                            e
                        )));
                    }
                }
            } else {
                debug!("Connect, No credentials provided so is running as guest");
            }
        }
        Ok(client)
    }
    #[tracing::instrument(skip_all)]
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
                    let queues = inner.queues.lock().await;

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
                    } else if command == "refreshtoken" {
                        // TODO: store jwt at some point in the future
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
                    } else if command == "queueevent" {
                        let queueevent: QueueEvent =
                            prost::Message::decode(received.data.unwrap().value.as_ref()).unwrap();
                        if let Some(callback) = queues.get(queueevent.queuename.as_str()) {
                            callback(queueevent);
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
                        error!("Received unhandled {} message: {:?}", command, received);
                    }
                }
            }
        });
        Ok(())
    }
    #[allow(dead_code, unused_variables)]
    #[tracing::instrument(skip_all)]
    pub fn set_callback(&mut self, callback: Box<dyn Fn(String) + Send + Sync>) {
        // self.callback = Some(callback);
    }
    #[tracing::instrument(skip_all)]
    fn get_id(&self) -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
    #[tracing::instrument(skip_all)]
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
    #[tracing::instrument(skip_all)]
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
        trace!("Sending #{} {} message", id, msg.command);
        msg.id = id.clone();
        {
            trace!("get inner lock");
            let inner = self.inner.lock().await;
            trace!("get query lock");
            inner.queries.lock().await.insert(id.clone(), response_tx);
            trace!("call send");
            let res = inner.stream_tx.send(msg).await;
            trace!("parse result");
            match res {
                Ok(_) => (),
                Err(e) => return Err(OpenIAPError::ClientError(e.to_string())),
            }
        }
        Ok((response_rx, id))
    }
    #[tracing::instrument(skip_all)]
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
            match res {
                Ok(_) => (),
                Err(e) => return Err(OpenIAPError::ClientError(e.to_string())),
            }
        }
        Ok((response_rx, stream_rx))
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    async fn ping(&self) {
        let envelope = Envelope {
            command: "ping".into(),
            ..Default::default()
        };
        let inner = self.inner.lock().await;
        match inner.stream_tx.send(envelope).await {
            Ok(_) => (),
            Err(e) => error!("Failed to send ping: {}", e),            
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn signin(&self, mut config: SigninRequest) -> Result<SigninResponse, OpenIAPError> {
        // autodetect how to signin using environment variables
        if config.username.is_empty() && config.password.is_empty() && config.jwt.is_empty() {
            if config.jwt.is_empty() {
                config.jwt = std::env::var("OPENIAP_JWT").unwrap_or_default();
            }
            if config.jwt.is_empty() {
                config.jwt = std::env::var("jwt").unwrap_or_default();
            }
            // if no jwt was found, test for username and password
            if config.jwt.is_empty() {
                if config.username.is_empty() {
                    config.username = std::env::var("OPENIAP_USERNAME").unwrap_or_default();
                }
                if config.password.is_empty() {
                    config.password = std::env::var("OPENIAP_PASSWORD").unwrap_or_default();
                }
            }
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
    #[tracing::instrument(skip_all)]
    pub async fn query(&self, mut config: QueryRequest) -> Result<QueryResponse, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }

        let envelope = config.to_envelope();
        debug!("Sending query {:?}", envelope);
        let result = self.send(envelope).await;
        debug!("Get result from send, mathing result");
        match result {
            Ok(m) => {
                debug!("Ok, m.command = {}", m.command);
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: QueryResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                debug!("Return Ok(response)");
                Ok(response)
            }
            Err(e) => {
                debug!("Error !!");
                Err(OpenIAPError::ClientError(e.to_string())) 
            },
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn aggregate(
        &self,
        mut config: AggregateRequest,
    ) -> Result<AggregateResponse, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }
        if config.hint.is_empty() {
            config.hint = "".to_string();
        }
        if config.queryas.is_empty() {
            config.queryas = "".to_string();
        }
        if config.aggregates.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No aggregates provided".to_string(),
            ));
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
                let response: AggregateResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn count(&self, mut config: CountRequest) -> Result<CountResponse, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }
        if config.query.is_empty() {
            config.query = "{}".to_string();
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
                let response: CountResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn distinct(
        &self,
        mut config: DistinctRequest,
    ) -> Result<DistinctResponse, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }
        if config.query.is_empty() {
            config.query = "{}".to_string();
        }
        if config.field.is_empty() {
            return Err(OpenIAPError::ClientError("No field provided".to_string()));
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
                let data = m.data.unwrap();
                let response: DistinctResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn insert_one(
        &self,
        config: InsertOneRequest,
    ) -> Result<InsertOneResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertOneResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn insert_many(
        &self,
        config: InsertManyRequest,
    ) -> Result<InsertManyResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertManyResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn update_one(
        &self,
        config: UpdateOneRequest,
    ) -> Result<UpdateOneResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: UpdateOneResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn insert_or_update_one(
        &self,
        config: InsertOrUpdateOneRequest,
    ) -> Result<String, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertOrUpdateOneResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.result)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn insert_or_update_many(
        &self,
        config: InsertOrUpdateManyRequest,
    ) -> Result<InsertOrUpdateManyResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertOrUpdateManyResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn update_document(
        &self,
        config: UpdateDocumentRequest,
    ) -> Result<UpdateDocumentResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: UpdateDocumentResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn delete_one(
        &self,
        config: DeleteOneRequest,
    ) -> Result<i32, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: DeleteOneResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.affectedrows)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[allow(dead_code)]
    #[tracing::instrument(skip_all)]
    pub async fn delete_many(
        &self,
        config: DeleteManyRequest,
    ) -> Result<i32, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: DeleteManyResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.affectedrows)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
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
                debug!("Temp file: {:?}", temp_file_path);
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
                trace!("Moving file to {}", filepath);
                move_file(temp_file_path.to_str().unwrap(), filepath.as_str()).map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to move file: {}", e))
                })?;
                debug!("Downloaded file to {}", filepath);
                downloadresponse.filename = filepath;

                Ok(downloadresponse)
            }
            Err(status) => Err(OpenIAPError::ClientError(status.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn upload(
        &self,
        config: UploadRequest,
        filepath: &str,
    ) -> Result<UploadResponse, OpenIAPError> {
        debug!("Rust::upload: Uploading file: {}", filepath);
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
    #[tracing::instrument(skip_all)]
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
    #[tracing::instrument(skip_all)]
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
    #[tracing::instrument(skip_all)]
    pub async fn register_queue(
        &self,
        mut config: RegisterQueueRequest,
        callback: Box<dyn Fn(QueueEvent) + Send + Sync>,
    ) -> Result<String, OpenIAPError> {
        if config.queuename.is_empty() {
            config.queuename = "".to_string();
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
                let response: RegisterQueueResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;

                let inner = self.inner.lock().await;
                inner
                    .queues
                    .lock()
                    .await
                    .insert(response.queuename.clone(), callback);

                Ok(response.queuename)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn unregister_queue(&self, queuename: &str) -> Result<(), OpenIAPError> {
        let config = UnRegisterQueueRequest::byqueuename(queuename);
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
    #[tracing::instrument(skip_all)]
    pub async fn register_exchange(
        &self,
        mut config: RegisterExchangeRequest,
        callback: Box<dyn Fn(QueueEvent) + Send + Sync>,
    ) -> Result<String, OpenIAPError> {
        if config.exchangename.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No exchange name provided".to_string(),
            ));
        }
        if config.algorithm.is_empty() {
            config.algorithm = "fanout".to_string();
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
                let response: RegisterExchangeResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                if response.queuename.is_empty() == false {
                    let inner = self.inner.lock().await;
                    inner
                        .queues
                        .lock()
                        .await
                        .insert(response.queuename.clone(), callback);
                }
                Ok(response.queuename)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn queue_message(
        &self,
        config: QueueMessageRequest,
    ) -> Result<QueueMessageResponse, OpenIAPError> {
        if config.queuename.is_empty() && config.exchangename.is_empty() {
            return Err(OpenIAPError::ClientError("No queue or exchange name provided".to_string()));
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
                let response: QueueMessageResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn push_workitem(
        &self,
        mut config: PushWorkitemRequest,
    ) -> Result<PushWorkitemResponse, OpenIAPError> {
        if config.wiq.is_empty() && config.wiqid.is_empty() {
            return Err(OpenIAPError::ClientError("No queue name or id provided".to_string()));
        }
        for f in &mut config.files {
            println!("File len: {:?}", f.file.len());
            if f.filename.is_empty() && f.file.is_empty() {
                debug!("Filename is empty");
            } else if f.filename.is_empty() == false && f.file.is_empty() && f.id.is_empty(){
                // does file exist?
                if !std::path::Path::new(&f.filename).exists() {
                    debug!("File does not exist: {}", f.filename);
                } else {
                    let filesize = std::fs::metadata(&f.filename).unwrap().len();
                    // if filesize is less than 5 meggabytes attach it, else upload
                    if filesize < 5 * 1024 * 1024 {                    
                        debug!("File {} exists so ATTACHING it.", f.filename);
                        let filename = std::path::Path::new(&f.filename).file_name().unwrap().to_str().unwrap();
                        f.file = std::fs::read(&f.filename).unwrap();
                        // f.file = compress_file(&f.filename).unwrap();
                        // f.compressed = false;
                        f.file = compress_file_to_vec(&f.filename).unwrap();
                        f.compressed = true;
                        f.filename = filename.to_string();
                        f.id = "findme".to_string();
                        trace!("File {} was read and assigned to f.file, size: {}", f.filename, f.file.len());
                    } else {
                        debug!("File {} exists so UPLOADING it.", f.filename);
                        let filename = std::path::Path::new(&f.filename).file_name().unwrap().to_str().unwrap();
                        let uploadconfig = UploadRequest {
                            filename: filename.to_string(),
                            collectionname: "fs.files".to_string(),
                            ..Default::default()
                        };
                        let uploadresult = self.upload(uploadconfig, &f.filename).await.unwrap();
                        trace!("File {} was upload as {}", filename, uploadresult.id);
                        // f.filename = "".to_string();
                        f.id = uploadresult.id.clone();
                        f.filename = filename.to_string();
                    }
                }
            } else {
                debug!("File {} is already uploaded", f.filename);
            }
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
                let response: PushWorkitemResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn pop_workitem(
        &self,
        config: PopWorkitemRequest,
        downloadfolder: Option<&str>,
    ) -> Result<PopWorkitemResponse, OpenIAPError> {
        if config.wiq.is_empty() && config.wiqid.is_empty() {
            return Err(OpenIAPError::ClientError("No queue name or id provided".to_string()));
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
                let response: PopWorkitemResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;

                match &response.workitem {
                    Some(wi) => {
                        for f in &wi.files {
                            if f.id.is_empty() == false {
                                let downloadconfig = DownloadRequest {
                                    id: f.id.clone(),
                                    collectionname: "fs.files".to_string(),
                                    ..Default::default()
                                };
                                let downloadresult = match self
                                .download(downloadconfig, downloadfolder, None)
                                .await {
                                    Ok(r) => r,
                                    Err(e) => {
                                        debug!("Failed to download file: {}", e);
                                        continue;
                                    }
                                };                                    
                                debug!("File {} was downloaded as {}", f.filename, downloadresult.filename);
                            }
                        }
                    }
                    None => {
                        debug!("No workitem found");
                    }
                }                
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    #[tracing::instrument(skip_all)]
    pub async fn update_workitem(
        &self,
        mut config: UpdateWorkitemRequest,
    ) -> Result<UpdateWorkitemResponse, OpenIAPError> {
        match &config.workitem {
            Some(wiq) => {
                if wiq.id.is_empty() {
                    return Err(OpenIAPError::ClientError("No workitem id provided".to_string()));
                }
            }
            None => {
                return Err(OpenIAPError::ClientError("No workitem provided".to_string()));
            }
        }
        for f in &mut config.files {
            if f.filename.is_empty() && f.file.is_empty() {
                debug!("Filename is empty");
            } else if f.filename.is_empty() == false && f.file.is_empty() && f.id.is_empty(){
                // does file exist?
                if !std::path::Path::new(&f.filename).exists() {
                    debug!("File does not exist: {}", f.filename);
                } else {
                    debug!("File {} exists so uploading it.", f.filename);
                    let filename = std::path::Path::new(&f.filename).file_name().unwrap().to_str().unwrap();
                    let uploadconfig = UploadRequest {
                        filename: filename.to_string(),
                        collectionname: "fs.files".to_string(),
                        ..Default::default()
                    };
                    let uploadresult = self.upload(uploadconfig, &f.filename).await.unwrap();
                    trace!("File {} was upload as {}", filename, uploadresult.id);
                    // f.filename = "".to_string();
                    f.id = uploadresult.id.clone();

                    // read file content and assign it to f.file as a base64 encoded string
                    // f.file = base64::encode(std::fs::read(&f.filename).unwrap()).into();
                    // f.file = base64::engine::general_purpose::STANDARD.encode(std::fs::read(&f.filename).unwrap()).into();
                    // f.file = "wefewwe".to_string().into();
                    // f.file = base64::engine::general_purpose::STANDARD.encode(std::fs::read(&f.filename).unwrap()).into();
                    //f.file = std::fs::read(&f.filename).unwrap();
                    // f.filename = filename.to_string();
                    // f.id = "";
                    // f.compressed = false;
                    // f.id = "".to_string();
                    // println!("File {} was read and assigned to f.file, size: {}", f.filename, f.file.len());
                    // workitem.files.push( WorkitemFile {
                    //     filename: filename.to_string(),
                    //     id: uploadresult.id,
                    //     compressed: false,
                    //     file: "".to_string().into(),
                    // });
                    f.filename = filename.to_string();
                    // f.filename = "".to_string();
                }
            } else {
                debug!("Skipped file");
            }
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
                let response: UpdateWorkitemResponse =
                    prost::Message::decode(m.data.unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
}

#[allow(dead_code)]
fn is_normal<T: Sized + Send + Sync + Unpin + Clone>() {}
#[cfg(test)]
mod tests {
    use futures::stream::FuturesUnordered;
    use std::{future::Future, pin::Pin};

    use super::*;
    #[allow(dead_code)]
    const TEST_URL: &str = "http://localhost:50051";
    // const TEST_URL: &str = "http://grpc.demo.openiap.io";
    // const TEST_URL: &str = "";
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
        let tasks = FuturesUnordered::<
            Pin<Box<dyn Future<Output = Result<QueryResponse, OpenIAPError>>>>,
        >::new();
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
    async fn test_aggreate() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let query = AggregateRequest {
            collectionname: "entities".to_string(),
            aggregates: "[]".to_string(),
            ..Default::default()
        };
        let response = client.aggregate(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_aggreate_multiple() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let tasks = FuturesUnordered::<
            Pin<Box<dyn Future<Output = Result<AggregateResponse, OpenIAPError>>>>,
        >::new();
        for _ in 1..101 {
            let query = AggregateRequest {
                collectionname: "entities".to_string(),
                aggregates: "[]".to_string(),
                ..Default::default()
            };
            tasks.push(Box::pin(client.aggregate(query)));
        }
        // while let Some(result) = tasks.next().await {
        //     println!("{}", result);
        // }
        let result = futures::future::join_all(tasks).await;
        println!("{:?}", result);
    }
    #[tokio::test()]
    async fn test_count() {
        // cargo test test_count -- --nocapture
        let client = Client::connect(TEST_URL).await.unwrap();
        let query = CountRequest {
            collectionname: "entities".to_string(),
            query: "{}".to_string(),
            ..Default::default()
        };
        let response = client.count(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_distinct() {
        // cargo test test_distinct -- --nocapture
        let client = Client::connect(TEST_URL).await.unwrap();
        let query = DistinctRequest {
            collectionname: "entities".to_string(),
            field: "_type".to_string(),
            ..Default::default()
        };
        let response = client.distinct(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_insert_one() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item: "{\"name\": \"test from rust\", \"_type\": \"test\"}".to_string(),
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_insert_many() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let query = InsertManyRequest {
            collectionname: "entities".to_string(),
            items: "[{\"name\": \"test many from rust 1\", \"_type\": \"test\"}, {\"name\": \"test many from rust 2\", \"_type\": \"test\"}]".to_string(),
            ..Default::default()
        };
        let response = client.insert_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        println!("Response: {:?}", response);
    }
    #[tokio::test()]
    async fn test_update_one() {
        let client = Client::connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"update test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);
        
        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();
        let item =format!("{{\"name\":\"updated from rust \", \"_id\": \"{}\"}}", _id);

        let query = UpdateOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.update_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_insert_or_update_one() {
        let client = Client::connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"insert or update one test from rust\", \"_type\": \"test\", \"age\": \"21\"}".to_string();
        let query = InsertOrUpdateOneRequest {
            collectionname: "entities".to_string(),
            item,
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);
        
        let _obj: serde_json::Value = serde_json::from_str(&response).unwrap();
        let _id = _obj["_id"].as_str().unwrap();
        let age = _obj["age"].as_str().unwrap();
        assert!(age == "21", "Age did not match after first insert or update");

        let item ="{\"name\":\"insert or update one test from rust\", \"age\": \"22\"}".to_string();

        let query = InsertOrUpdateOneRequest {
            collectionname: "entities".to_string(),
            item,
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response2: {:?}", response);
        let _obj: serde_json::Value = serde_json::from_str(&response).unwrap();
        let _id2 = _obj["_id"].as_str().unwrap();
        let age = _obj["age"].as_str().unwrap();
        assert!(age == "22", "Age did not match after first insert or update");

        assert!(_id == _id2, "ID did not match after update");
    }
    #[tokio::test()]
    async fn test_insert_or_update_many() {
        let client = Client::connect(TEST_URL).await.unwrap();

        let item1 = "{\"name\": \"insert or update many test from rust 1\", \"_type\": \"test\", \"age\": \"21\"}".to_string();
        let item2 = "{\"name\": \"insert or update many test from rust 2\", \"_type\": \"test\", \"age\": \"23\"}".to_string();
        let query = InsertOrUpdateManyRequest {
            collectionname: "entities".to_string(),
            items: format!("[{}, {}]", item1, item2),
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_many(query).await;

        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);

        let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
        let _id1 = _obj[0]["_id"].as_str().unwrap();
        let _id2 = _obj[1]["_id"].as_str().unwrap();
        let age1 = _obj[0]["age"].as_str().unwrap();

        let item1 ="{\"name\":\"insert or update many test from rust 1\", \"age\": \"22\"}".to_string();
        let item2 ="{\"name\":\"insert or update many test from rust 2\", \"age\": \"24\"}".to_string();

        let query = InsertOrUpdateManyRequest {
            collectionname: "entities".to_string(),
            items: format!("[{}, {}]", item1, item2),
            uniqeness: "name".to_string(),
            ..Default::default()
        };
        let response = client.insert_or_update_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response2: {:?}", response);
        let _obj: serde_json::Value = serde_json::from_str(&response.results).unwrap();
        let _id1_2 = _obj[0]["_id"].as_str().unwrap();
        let _id2_2 = _obj[1]["_id"].as_str().unwrap();
        let age1_2 = _obj[0]["age"].as_str().unwrap();

        assert!(_id1 == _id1_2, "ID1 did not match after update");
        assert!(_id2 == _id2_2, "ID2 did not match after update");
        assert!(age1 == "21", "Age1 did not match after first insert or update");
        assert!(age1_2 == "22", "Age1 did not match after second insert or update");
   
    }
    #[tokio::test()]
    async fn test_delete_one() {
        let client = Client::connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"delete test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);
        
        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();

        let query = DeleteOneRequest {
            collectionname: "entities".to_string(),
            id: _id.to_string(),
            ..Default::default()
        };
        let response = client.delete_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );        
    }
    #[tokio::test()]
    async fn test_delete_many_query() {
        let client = Client::connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"delete many query test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);
        
        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();

        let query = DeleteManyRequest {
            collectionname: "entities".to_string(),
            query: format!("{{\"_id\": \"{}\"}}", _id),
            ..Default::default()
        };
        let response = client.delete_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    }
    #[tokio::test()]
    async fn test_delete_many_ids() {
        let client = Client::connect(TEST_URL).await.unwrap();

        let item = "{\"name\": \"delete many ids test from rust\", \"_type\": \"test\"}".to_string();
        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item,
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
        let response = response.unwrap();
        println!("Response: {:?}", response);
        
        let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
        let _id = _obj["_id"].as_str().unwrap();

        let query = DeleteManyRequest {
            collectionname: "entities".to_string(),
            ids: vec![_id.to_string()],
            ..Default::default()
        };
        let response = client.delete_many(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );        
    }
    #[tokio::test()]
    async fn test_bad_login() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let response = client
            .signin(SigninRequest::with_userpass("testuser", "badpassword"))
            .await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
                assert!(
                    false,
                    "login with bad password, did not fail"
                );
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    #[tokio::test()]
    async fn test_upload() {
        let client = Client::connect(TEST_URL).await.unwrap();
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());

        let response = client
            .upload(UploadRequest::filename("rust-test.csv"), "../../testfile.csv")
            .await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
            }
            Err(e) => {
                assert!(
                    false,
                    "Upload of testfile.csv failed with {:?}",
                    e
                );
            }
        }
    }
    #[tokio::test()]
    async fn test_upload_as_guest() {
        let client = Client::connect(TEST_URL).await.unwrap();
        client
            .signin(SigninRequest::with_userpass("guest", "password"))
            .await
            .unwrap();
        let response = client
            .upload(UploadRequest::filename("rust-test-user.csv"), "../../testfile.csv")
            .await;
        match response {
            Ok(response) => {
                println!("{:?}", response);
                assert!(
                    false,
                    "Upload of testfile.csv did not fail as guest"
                );
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
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
        let response = client
            .signin(SigninRequest::with_userpass("guest", "password"))
            .await
            .unwrap();
        println!("Signin response: {:?}", response);
        let response = client
            .download(DownloadRequest::id("65a3aaf66d52b8c15131aebd"), None, None)
            .await;
        println!("Download response: {:?}", response);
        assert!(
            response.is_err(),
            "Download of file as guest did not failed"
        );
    }
    #[tokio::test]
    async fn test_watch() {
        let client = Client::connect(TEST_URL).await.unwrap();
    
        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx))); 
    
        let response: std::result::Result<String, OpenIAPError> = client
            .watch(WatchRequest::new("", vec!["".to_string()]), {
                let tx = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Watch event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await;
    
        println!("Watch response: {:?}", response);
    
        assert!(
            response.is_ok(),
            "Watch failed with {:?}",
            response.err().unwrap()
        );
    
        let id = response.unwrap();

        let query = InsertOneRequest {
            collectionname: "entities".to_string(),
            item: "{\"name\": \"testing watch from rust\", \"_type\": \"test\"}".to_string(),
            ..Default::default()
        };
        let response = client.insert_one(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    
        // Await the watch event
        rx.await.unwrap();
        println!("Watch event received");
    
        client.unwatch(&id).await.unwrap();
    }
    #[tokio::test]
    async fn test_register_queue() {
        let client = Client::connect(TEST_URL).await.unwrap();
    
        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx))); 
    
        let response: std::result::Result<String, OpenIAPError> = client
            .register_queue(RegisterQueueRequest::byqueuename("secrettestqueue"), {
                let tx = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Queue event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await;
    
        println!("RegisterQueue response: {:?}", response);
    
        assert!(
            response.is_ok(),
            "RegisterQueue failed with {:?}",
            response.err().unwrap()
        );
    
        let queuename = response.unwrap();

        println!("Send message to queue: {:?}", queuename);
        let query = QueueMessageRequest {
            queuename: queuename.clone(),
            data: "{\"test\": \"message\"}".to_string(),
            striptoken: true,
            ..Default::default()
        };
        let response = client.queue_message(query).await;
        assert!(
            response.is_ok(),
            "test_query failed with {:?}",
            response.err().unwrap()
        );
    
        println!("Await the queue event");
        rx.await.unwrap();
        println!("Queue event received");
    
        client.unregister_queue(&queuename).await.unwrap();
    }
    #[tokio::test]// cargo test test_register_exchange -- --nocapture
    async fn test_register_exchange() {
        let exchangename = "secrettestexchange";
        let client = Client::connect(TEST_URL).await.unwrap();
    
        let (tx, rx) = oneshot::channel::<()>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx))); 
    
        let response = client
            .register_exchange(RegisterExchangeRequest::byexchangename(exchangename), {
                let tx = Arc::clone(&tx);
                Box::new(move |event| {
                    println!("Queue event: {:?}", event);
                    if let Some(tx) = tx.lock().unwrap().take() {
                        let _ = tx.send(());
                    }
                })
            })
            .await;
    
        println!("RegisterExchange response: {:?}", response);
    
        assert!(
            response.is_ok(),
            "RegisterExchange failed with {:?}",
            response.err().unwrap()
        );
    
        let queuename = response.unwrap();

        println!("Send message to exchange: {:?}", exchangename);
        let query = QueueMessageRequest {
            exchangename: exchangename.to_string(),
            data: "{\"test\": \"message\"}".to_string(),
            striptoken: true,
            ..Default::default()
        };
        let response = client.queue_message(query).await;
        assert!(
            response.is_ok(),
            "test_exhange failed with {:?}",
            response.err().unwrap()
        );
    
        println!("Await the queue event");
        rx.await.unwrap();
        println!("Queue event received");
    
        client.unregister_queue(&queuename).await.unwrap();
    }
    #[tokio::test] // cargo test test_push_workitem -- --nocapture
    async fn test_push_workitem() {
        let client = Client::connect(TEST_URL).await.unwrap();
    
        let response = client
            .push_workitem(
                PushWorkitemRequest {
                    wiq: "rustqueue".to_string(),
                    name: "test rust workitem".to_string(),
                    files: vec![WorkitemFile {
                        filename: "../../testfile.csv".to_string(),
                        ..Default::default()
                    }],
                    payload: "{\"test\": \"message\"}".to_string(),
                    // nextrun: Some(Timestamp::from(std::time::SystemTime::now() + std::time::Duration::from_secs(60))),
                    ..Default::default()
                }
            )
            .await;
    
        // println!("PushWorkitem response: {:?}", response);
    
        assert!(
            response.is_ok(),
            "PushWorkitem failed with {:?}",
            response.err().unwrap()
        );
    
        let response = client
            .pop_workitem(
                PopWorkitemRequest {
                    wiq: "rustqueue".to_string(),
                    ..Default::default()
                },
                Some(".")
            )
            .await;
        // println!("PopWorkitem response: {:?}", response);
            
        assert!(
            response.is_ok(),
            "PopWorkitem failed with {:?}",
            response.err().unwrap()
        );
        let mut workitem = response.unwrap().workitem.unwrap();
        workitem.name = "updated test rust workitem".to_string();
        workitem.payload = "{\"test\": \"updated message\"}".to_string();
        workitem.state = "successful".to_string();
        // workitem.files[0].filename = "updated_testfile.csv".to_string();
        assert!( workitem.files.len() > 0, "workitem has no files");

        // workitem.files.remove(0);
        workitem.files[0].id = "".to_string();

        // delete testfile.csv if exsits
        if std::path::Path::new("testfile.csv").exists() {
            println!("Deleting testfile.csv");
            std::fs::remove_file("testfile.csv").unwrap();
        }

        let response = client
            .update_workitem(
                UpdateWorkitemRequest {
                    workitem: Some(workitem),
                    files: vec![WorkitemFile {
                        filename: "../../train.csv".to_string(),
                        ..Default::default()
                    }, WorkitemFile {
                        filename: "testfile.csv".to_string(),
                        ..Default::default()
                    }],
                    ..Default::default()
                }
            )
            .await;
        // println!("UpdateWorkitem response: {:?}", response);
        assert!(
            response.is_ok(),
            "UpdateWorkitem failed with {:?}",
            response.err().unwrap()
        );
    }
}
