#![warn(missing_docs)]
//! The `openiap.client` crate provides the [Client] struct and its methods.
//! For now this only support grpc, will over time have added support for web sockets, http, tcp and named pipes.
//! Initialize a new client, by calling the [Client::connect] method.
//! ```
//! use openiap_client::{ OpenIAPError, Client, QueryRequest };
//! #[tokio::main]
//! async fn main() -> Result<(), OpenIAPError> {
//!     let client = Client::connect("").await?;
//!     let q = client.query( QueryRequest::with_projection(
//!         "entities",
//!         "{}",
//!         "{\"name\":1}"
//!     )).await?;
//!     let items: serde_json::Value = serde_json::from_str(&q.results).unwrap();
//!     let items: &Vec<serde_json::Value> = items.as_array().unwrap();
//!     for item in items {
//!         println!("Item: {:?}", item);
//!     }
//!     Ok(())
//! }
//! ```


/// The `Client` struct provides the client for the OpenIAP service.
/// Initialize a new client, by calling the [Client::connect] method.
#[derive(Clone)]
pub struct Client {
    /// The inner client.
    pub inner: Arc<Mutex<ClientInner>>,
    /// The `Config` struct provides the configuration for the OpenIAP service we are connecting to.
    pub config: Option<Config>,
    /// Should client automatically reconnect, if disconnected?
    pub auto_reconnect: bool,
    /// URL used to connect to server, processed and without credentials
    pub url: String,
    event_sender: async_channel::Sender<ClientEvent>,
    event_receiver: async_channel::Receiver<ClientEvent>,
    /// Is client connected?
    pub connected: Arc<std::sync::Mutex<bool>>,
    /// The stdin sender.
    pub stdin_tx: futures::channel::mpsc::UnboundedSender<tokio_tungstenite::tungstenite::Message>,
}

/// The `ClientInner` struct provides the inner client for the OpenIAP service.
#[derive(Clone)]
pub struct ClientInner {
    /// The grpc client.//!
    pub client: ClientEnum,
    /// Websocket read/write streams
    // pub split: Option<Arc<Mutex<SockSplit>>>,
    /// Inceasing message count, used as unique id for messages.
    pub msgcount: Arc<Mutex<i32>>,
    /// Are we signed in?
    pub signedin: bool,
    /// The signed in user.
    pub user: Option<User>,
    /// The stream sender.
    pub stream_tx: mpsc::Sender<Envelope>,
    // pub stdin_tx: futures::channel::mpsc::UnboundedSender<Vec<u8>>,
    // pub stdin_rx: Arc<Mutex<futures::channel::mpsc::UnboundedReceiver<Vec<u8>>>>,
    // pub stdin_rx: Arc<Mutex<futures::channel::mpsc::UnboundedReceiver<tokio_tungstenite::tungstenite::Message>>>,

//     expected struct `futures::futures_channel::mpsc::UnboundedSender<Vec<u8>>`
//    found struct `futures::futures_channel::mpsc::UnboundedSender<tokio_tungstenite::tungstenite::Message>`

    /// list of queries ( messages sent to server we are waiting on a response for )
    pub queries: Arc<Mutex<std::collections::HashMap<String, QuerySender>>>,
    /// Active streams the server (or client) has opened
    pub streams: Arc<Mutex<std::collections::HashMap<String, StreamSender>>>,
    /// List of active watches ( change streams )
    #[allow(clippy::type_complexity)]
    pub watches:
        Arc<Mutex<std::collections::HashMap<String, Box<dyn Fn(WatchEvent) + Send + Sync>>>>,
    /// List of active queues ( message queues / mqqt queues or exchanges )
    #[allow(clippy::type_complexity)]
    pub queues:
        Arc<Mutex<std::collections::HashMap<String, Box<dyn Fn(QueueEvent) + Send + Sync>>>>,
}

// expected struct `WebSocket<tokio_tungstenite::tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>`
//    found struct `WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>`

// type Sock = tokio_tungstenite::tungstenite::WebSocket<tokio_tungstenite::tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;
type Sock = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
// type SockSplit = (
//     SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>,
//     SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>
// );
// type Sock = async_tungstenite::tungstenite::WebSocket<async_tungstenite::tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;
// type SockSplit = (
//     SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, async_tungstenite::tungstenite::Message>,
//     SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>
// );

// use futures::stream::{ SplitSink, SplitStream };
// use futures::StreamExt;
// use futures::{SinkExt, StreamExt};
use futures::{StreamExt };
use async_channel::{unbounded};


/// Client enum, used to determine which client to use.
#[derive(Debug)]
#[derive(Clone)]
pub enum ClientEnum {
    /// Not set yet
    None,
    /// Used when client wants to connect using gRPC 
    Grpc(FlowServiceClient<tonic::transport::Channel>),
    /// Used when client wants to connect using websockets
    WS(Arc<Mutex<Sock>>)
}

/// Client event enum, used to determine which event has occurred.
#[derive(Debug, Clone, PartialEq)]
pub enum ClientEvent {
    /// The client has connected
    Connected,
    /// The client has disconnected
    Disconnected(String),
    /// The client has signed in
    SignedIn,
    /// The client has signed out
    SignedOut,
    /// The client has received a message
    Message(Envelope),
    /// The client has received a stream
    Stream(Vec<u8>),
    /// The client has received a watch event
    Watch(WatchEvent),
    /// The client has received a queue event
    Queue(QueueEvent),
}

pub use openiap_proto::errors::*;
pub use openiap_proto::protos::*;
pub use openiap_proto::*;
pub use prost_types::Timestamp;
pub use protos::flow_service_client::FlowServiceClient;
use prost::Message;
use futures::SinkExt;
// futures-util = "0.3.30"
// use futures_util::{StreamExt, SinkExt};


use tokio_tungstenite::{ // MaybeTlsStream, 
    WebSocketStream };
// use async_tungstenite::tungstenite::stream::MaybeTlsStream;
// use async_tungstenite::{ WebSocketStream };

// use tokio::net::TcpStream;
// use tokio_tungstenite::connect_async;
// use tokio_tungstenite::connect_async_tls_with_config;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
// use tokio_tungstenite::tungstenite::WebSocket;
use tracing::{debug, error, info, trace};

use tokio_stream::{wrappers::ReceiverStream};
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::AddAssign;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
// use std::thread;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use tokio::sync::{mpsc, oneshot};
use tonic::Request;

use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

mod otel;
mod tests;
mod ws;

type QuerySender = oneshot::Sender<Envelope>;
type StreamSender = mpsc::Sender<Vec<u8>>;

/// The `Config` struct provides the configuration for the OpenIAP service we are connecting to.
#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub struct Config {
    #[serde(default)]
    wshost: String,
    #[serde(default)]
    wsurl: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    auto_create_users: bool,
    #[serde(default)]
    namespace: String,
    #[serde(default)]
    agent_domain_schema: String,
    #[serde(default)]
    version: String,
    #[serde(default)]
    validate_emails: bool,
    #[serde(default)]
    forgot_pass_emails: bool,
    #[serde(default)]
    supports_watch: bool,
    #[serde(default)]
    amqp_enabled_exchange: bool,
    #[serde(default)]
    multi_tenant: bool,
    #[serde(default)]
    enable_entity_restriction: bool,
    #[serde(default)]
    enable_web_tours: bool,
    #[serde(default)]
    collections_with_text_index: Vec<String>,
    #[serde(default)]
    timeseries_collections: Vec<String>,
    #[serde(default)]
    ping_clients_interval: i32,
    #[serde(default)]
    validlicense: bool,
    #[serde(default)]
    forceddomains: Vec<String>,
    #[serde(default)]
    grafana_url: String,
    #[serde(default)]
    otel_metric_url: String,
}
impl std::fmt::Debug for ClientInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientInner")
            // .field("client", &self.client)
            .field("signedin", &self.signedin)
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

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{self, BufReader, BufWriter};
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
/// Read a file and compresses it into a `Vec<u8>`.
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

use opentelemetry::{
    // global,
    KeyValue, // trace::Tracer
};
use opentelemetry_otlp::{
    // ExportConfig, Protocol, 
    WithExportConfig};
// use opentelemetry_sdk::metrics::reader::{DefaultAggregationSelector, DefaultTemporalitySelector};
use opentelemetry_sdk::{
    // trace::{self, RandomIdGenerator, Sampler},
    Resource,
};
use std::time::Duration;
use opentelemetry_otlp::{new_exporter, new_pipeline};
use opentelemetry_sdk::{runtime::Tokio};

struct ProviderWrapper {
    provider: Option<opentelemetry_sdk::metrics::SdkMeterProvider>
}
use lazy_static::lazy_static;
lazy_static! {
    static ref provider1: std::sync::Mutex<ProviderWrapper> = std::sync::Mutex::new(ProviderWrapper {
        provider: None
    });
    static ref provider2: std::sync::Mutex<ProviderWrapper> = std::sync::Mutex::new(ProviderWrapper {
        provider: None
    });
}
use opentelemetry::metrics::MeterProvider;
/// Initialize telemetry
pub fn init_telemetry(strurl: &str, otlpurl: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    if strurl.is_empty() {
        return Err(Box::new(OpenIAPError::ClientError("No URL provided".to_string())));
    }
    let period = 5;
    let enable_analytics = std::env::var("enable_analytics").unwrap_or("".to_string());
    let enable_analytics: bool = !enable_analytics.eq_ignore_ascii_case("false");
    let url = url::Url::parse(strurl)
    .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;
    let mut apihostname = url.host_str().unwrap_or("localhost.openiap.io").to_string();
    if apihostname.starts_with("grpc.") {
        apihostname = apihostname[5..].to_string();
    }

    let mut hasher = md5::Context::new();
    hasher.write_all(apihostname.as_bytes()).unwrap();
    let ofid = format!("{:x}", hasher.compute());

    if enable_analytics {
        debug!("Initializing generic telemetry");
        let mut providers1 = provider1.lock().unwrap();
        if providers1.provider.is_none() {
            let exporter1 = new_exporter()
                .tonic()
                .with_tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
                .with_endpoint("https://otel.stats.openiap.io:443");
            let provider = new_pipeline()
            .metrics(Tokio)
            .with_exporter(exporter1)
            .with_resource(Resource::new(vec![KeyValue::new("service.name", "rust")]))
            .with_period(Duration::from_secs(period))
            .build().unwrap();
            let meter1 = provider.meter("process-meter1");
            // let meter: opentelemetry::metrics::Meter = meterprovider1.meter("process-meter1");
            // when not using global::set_meter_provider we need to keep it alive using ProivderWrapper
            match otel::register_metrics(meter1, &ofid) {
                Ok(_) => (),
                Err(e) => {
                    debug!("Failed to initialize process observer: {}", e);
                }
            }
            providers1.provider = Some(provider);
        }
    }

    if !otlpurl.is_empty() {
        debug!("Adding {} for telemetry", otlpurl);
        let mut providers2 = provider2.lock().unwrap();
        if providers2.provider.is_none() {
            let exporter2 = new_exporter()
                .tonic()
                .with_tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
                .with_endpoint(otlpurl);
            let provider = new_pipeline()
                .metrics(Tokio)
                .with_exporter(exporter2)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "rust")]))
                .with_period(Duration::from_secs(period))
                .build().unwrap();

            let meter2 = provider.meter("process-meter2");
            // when not using global::set_meter_provider we need to keep it alive using ProivderWrapper
            match otel::register_metrics(meter2, &ofid) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to initialize process observer: {}", e);
                }
            }
            providers2.provider = Some(provider);
        }
    }

    Ok(())
}
/// Enable global tracing ( cannot be updated once set )\
/// - rust_log is a [tracing_subscriber::EnvFilter] string ( use empty string to use environment variable RUST_LOG )\
/// - tracing is a string that can be empty for nothing, or one of the following: new, enter, exit, close, active or full.\
/// \
/// To enable tracing, and only track debug messsages and all new function calls for this create, use\
/// ```
/// use openiap_client::enable_tracing;
/// enable_tracing("openiap=debug", "new");
/// ```
pub fn enable_tracing(rust_log: &str, tracing: &str) {
    let rust_log = rust_log.to_string();
    let mut filter = tracing_subscriber::EnvFilter::from_default_env();
    if !rust_log.is_empty() {
        filter = tracing_subscriber::EnvFilter::new(rust_log.clone());
    }

    let mut subscriber = tracing_subscriber::fmt::layer();
    let tracing = tracing.to_string();
    if !tracing.is_empty() {
        subscriber = match tracing.to_lowercase().as_str() {
            "new" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW),
            "enter" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::ENTER),
            "exit" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::EXIT),
            "close" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
            "none" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE),
            "active" => {
                subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            }
            "full" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL),
            _ => subscriber,
        }
    }
    let subscriber = tracing_subscriber::Layer::with_subscriber(
        tracing_subscriber::Layer::and_then(subscriber, filter),
        tracing_subscriber::registry(),
    );

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(()) => {
            debug!("Tracing enabled");
        }
        Err(e) => {
            eprintln!("Tracing failed: {:?}", e);
        }
    }
    info!(
        "enable_tracing rust_log: {:?}, tracing: {:?}",
        rust_log, tracing
    );
}
// inner: Arc<Mutex<ClientInner>>
async fn send_envelope(mut client: Client, mut inner: ClientInner, mut envelope: Envelope) -> Result<(), OpenIAPError> {
    // let mut inner = self.inner.lock().await;
    match inner.client {
        ClientEnum::Grpc(ref mut _client) => {
            match inner.stream_tx.send(envelope).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    let errmsg = e.to_string();                    
                    client.set_connected(false, Some(&errmsg));
                    Err(OpenIAPError::ClientError(format!("Failed to send data: {}", errmsg)))
                },
            }
        }
        ClientEnum::WS(ref mut _client) => {
            {
                let mut seq = inner.msgcount.lock().await;
                envelope.seq = seq.clone();
                if envelope.rid.is_empty() {
                    envelope.rid = seq.to_string();
                }
                seq.add_assign(1);
            }
            // let cmd = envelope.command.clone();
            // get envelope length, then add it as the first 4 bytes of the message
            let envelope = envelope.encode_to_vec();
            let size = envelope.len() as u32;

            // Write the size in little-endian format (like writeUInt32LE in Node.js)
            let mut size_bytes = size.to_le_bytes().to_vec(); 

            // Append the actual envelope data
            size_bytes.extend_from_slice(&envelope);

            // Now size_bytes contains the 4-byte length followed by the envelope
            let size_bytes = size_bytes;

            match client.stdin_tx.send(tokio_tungstenite::tungstenite::Message::Binary(size_bytes)).await {
                Ok(_) => Ok(()),
                Err(e) => {
                    let errmsg = e.to_string();
                    client.set_connected(false, Some(&errmsg));
                    Err(OpenIAPError::ClientError(format!("Failed to send data: {}", errmsg)))
                },
            }
        }
        ClientEnum::None => {
            return Err(OpenIAPError::ClientError("Invalid client".to_string()));
        }
    }
}

async fn parse_incomming_envelope(inner: ClientInner, received: Envelope) {
    let command = received.command.clone();
    // let id = received.command.clone();
    let rid = received.rid.clone();
    let mut queries = inner.queries.lock().await;
    let mut streams = inner.streams.lock().await;
    let watches = inner.watches.lock().await;
    let queues = inner.queues.lock().await;

    debug!("Received #{} #{} {} message", received.id, rid, command);
    if command == "ping" {
        ()
    } else if command == "refreshtoken" {
        // TODO: store jwt at some point in the future
    } else if command == "beginstream"
        || command == "stream"
        || command == "endstream"
    {
        let streamresponse: Stream =
            prost::Message::decode(received.data.unwrap().value.as_ref()).unwrap();
        let streamdata = streamresponse.data;

        if !streamdata.is_empty() {
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
        if let Some(stream) = stream {
            let streamdata = vec![];
            match stream.send(streamdata).await {
                Ok(_) => _ = (),
                Err(e) => error!("Failed to send data: {}", e),
            }
        }
        let _ = response_tx.send(received);
    } else {
        error!("Received unhandled {} message: {:?}", command, received);
    }    
}

impl Client {
    /// Connect will initializes a new client and starts a connection to an OpenIAP server.\
    /// Use "" to autodetect the server from the environment variables (apiurl or grpcapiurl), or provide a URL.\
    /// \
    /// You can add username and password, to login using local provider, or set them using OPENIAP_USERNAME and OPENIAP_PASSWORD environment variables.
    /// It is highly recommended to not user username and password, but instead use a JWT token, set using the OPENIAP_JWT (or jwt) environment variable.
    /// You can use the openiap vs.code extension to manage this, if you need to generate one your self, login to the OpenIAP server and then open the /jwtlong page.
    /// If credentials are not provided, the client will run as guest.\
    /// If credentials are found, it will call [Client::signin] after successfully connecting to the server.
    /// 
    /// To troubleshoot issues, call [enable_tracing].
    /// ```
    /// use openiap_client::{ OpenIAPError, Client, QueryRequest };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let q = client.query( QueryRequest::with_projection(
    ///         "entities",
    ///         "{}",
    ///         "{\"name\":1}"
    ///     )).await?;
    ///     let items: serde_json::Value = serde_json::from_str(&q.results).unwrap();
    ///     let items: &Vec<serde_json::Value> = items.as_array().unwrap();
    ///     for item in items {
    ///         println!("Item: {:?}", item);
    ///     }
    ///     Ok(())
    /// }
    /// ```
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
        let usegprc = url.scheme() == "grpc";
        let issecure = url.scheme() == "https" || url.scheme() == "wss";
        if url.scheme() != "http"
            && url.scheme() != "https"
            && url.scheme() != "grpc"
            && url.scheme() != "ws"
            && url.scheme() != "wss"
        {
            return Err(OpenIAPError::ClientError("Invalid URL scheme".to_string()));
        }
        if url.scheme() == "grpc" {
            if url.port() == Some(443) {
                strurl = format!("https://{}", url.host_str().unwrap_or("app.openiap.io"));
            } else {
                strurl = format!("http://{}", url.host_str().unwrap_or("app.openiap.io"));
            }
        }
        let mut url = url::Url::parse(strurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;
        let mut username = "".to_string();
        let mut password = "".to_string();
        if let Some(p) = url.password() {
            password = p.to_string();
        }
        if !url.username().is_empty() {
            username = url.username().to_string();
        }
        url = url::Url::parse(strurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;

        if url.port().is_none() {
            if url.scheme() == "https" {
                strurl = format!(
                    "{}://{}",
                    url.scheme(),
                    url.host_str().unwrap_or("app.openiap.io")
                );
            } else {
                strurl = format!(
                    "{}://{}",
                    url.scheme(),
                    url.host_str().unwrap_or("app.openiap.io")
                );
            }
        } else {
            strurl = format!(
                "{}://{}:{}",
                url.scheme(),
                url.host_str().unwrap_or("localhost.openiap.io"),
                url.port().unwrap_or(80)
            );
        }
        info!("Connecting to {}", strurl);

        let config: Option<Config>;

        //  ", url.scheme() + "://" + url.host_str().replace("grpc.", "").as_str();
        // let configurl = format!("{}://{}/config", url.scheme(), url.host_str().unwrap_or("localhost.openiap.io").replace("grpc.", ""));
        let configurl: String;
        if issecure {
            configurl = format!(
                "{}://{}/config",
                "https",
                url.host_str()
                    .unwrap_or("localhost.openiap.io")
                    .replace("grpc.", "")
            );
        } else {
            configurl = format!(
                "{}://{}/config",
                "http",
                url.host_str()
                    .unwrap_or("localhost.openiap.io")
                    .replace("grpc.", "")
            );
        }

        let configurl = url::Url::parse(configurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;
        let o = minreq::get(configurl).send();
        match o {
            Ok(_) => {
                let response = o.unwrap();
                if response.status_code == 200 {
                    let body = response.as_str().unwrap();
                    config = Some(serde_json::from_str(body).unwrap());
                } else {
                    config = None;
                }
            }
            Err(e) => {
                return Err(OpenIAPError::ClientError(format!(
                    "Failed to get config: {}",
                    e
                )));
            }
        }

        let mut otel_metric_url = std::env::var("OTEL_METRIC_URL").unwrap_or_default();
        if config.is_some() {
            let config = config.as_ref().unwrap();
            if !config.otel_metric_url.is_empty() {
                otel_metric_url = config.otel_metric_url.clone();
            }
        }
        match init_telemetry(&strurl, otel_metric_url.as_str()) {
            Ok(_) => (),
            Err(e) => {
                return Err(OpenIAPError::ClientError(format!(
                    "Failed to initialize telemetry: {}",
                    e
                )));
            }
        }
        let innerclient: ClientEnum;
        let client = if !usegprc {
            strurl = format!("{}/ws/v2", strurl);

            let (stream_tx, stream_rx) = mpsc::channel(4);

            let (socket, _) = tokio_tungstenite::connect_async(strurl.clone())
                .await
                .expect("Can't connect");
            innerclient = ClientEnum::WS(Arc::new(Mutex::new(socket)));
            // // innerclient = ClientEnum::WSSocket(Arc::new(Mutex::new(ws_stream)));

            let (stdin_tx, _stdin_rx) = futures::channel::mpsc::unbounded();

            let inner = ClientInner {
                client: innerclient,
                // split: split,
                // split: None,
                msgcount: Arc::new(Mutex::new(0)),
                signedin: false,
                user: None,
                stream_tx: stream_tx.clone(),
                // stdin_rx: Arc::new(Mutex::new(stdin_rx)),
                queries: Arc::new(Mutex::new(std::collections::HashMap::new())),
                streams: Arc::new(Mutex::new(std::collections::HashMap::new())),
                watches: Arc::new(Mutex::new(std::collections::HashMap::new())),
                queues: Arc::new(Mutex::new(std::collections::HashMap::new())),
            };
            let (ces, cer) = unbounded::<ClientEvent>();
            let inner = Arc::new(Mutex::new(inner));
            let mut client = Client {
                url: strurl.clone(),
                inner: inner,
                config,
                connected: Arc::new(std::sync::Mutex::new(true)),
                auto_reconnect: true,
                event_sender: ces,
                event_receiver: cer,
                stdin_tx,
            };

            // client.inner.lock().await.stdin_tx = setup_ client:: ws::setup(&strurl, stream_tx.clone()).await;
            let stdin_tx = match client.setup_ws(&strurl, stream_tx).await {
                Ok(stdin_tx) => stdin_tx,
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to setup WS: {}",
                        e
                    )));
                }
            };
            client.stdin_tx = stdin_tx;

            let client2 = client.clone();
            tokio::spawn(async move {
                tokio_stream::wrappers::ReceiverStream::new(stream_rx)
                    .for_each(|envelope| async {
                        let command = envelope.command.clone();
                        let rid = envelope.rid.clone();
                        let id = envelope.id.clone();
                        trace!("Received command: {}, id: {}, rid: {}", command, id, rid);
                        let inner = client2.inner.lock().await;
                        parse_incomming_envelope(inner.clone(), envelope).await;
                    })
                    .await;
            });

            client
        } else {
            if url.scheme() == "http" {
                let response = FlowServiceClient::connect(strurl.clone()).await;
                match response {
                    Ok(client) => {
                        innerclient = ClientEnum::Grpc(client);
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
                innerclient = ClientEnum::Grpc(FlowServiceClient::new(channel));
            }

            let (stream_tx, stream_rx) = mpsc::channel(4);
            let in_stream = ReceiverStream::new(stream_rx);
            let (stdin_tx, _stdin_rx) = futures::channel::mpsc::unbounded();

            let inner = ClientInner {
                client: innerclient,
                // split: None,
                msgcount: Arc::new(Mutex::new(0)),
                signedin: false,
                user: None,
                stream_tx,
                // stdin_rx: Arc::new(Mutex::new(stdin_rx)),
                queries: Arc::new(Mutex::new(std::collections::HashMap::new())),
                streams: Arc::new(Mutex::new(std::collections::HashMap::new())),
                watches: Arc::new(Mutex::new(std::collections::HashMap::new())),
                queues: Arc::new(Mutex::new(std::collections::HashMap::new())),
            };

            let (ces, cer) = unbounded::<ClientEvent>();
            let mut client = Client {
                url: strurl.clone(),
                inner: Arc::new(Mutex::new(inner)),
                config,
                auto_reconnect: true,
                connected: Arc::new(std::sync::Mutex::new(true)),
                event_sender: ces,
                event_receiver: cer,
                stdin_tx,
            };
            client.setup_grpc_stream(in_stream).await?;
            client
        };

        client.ping().await;
        if username.is_empty() && password.is_empty() {
            username = std::env::var("OPENIAP_USERNAME").unwrap_or_default();
            password = std::env::var("OPENIAP_PASSWORD").unwrap_or_default();
        }
        if !username.is_empty() && !password.is_empty() {
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
            if !jwt.is_empty() {
                debug!("Signing in with JWT");
                let signin = SigninRequest::with_jwt(jwt.as_str());
                let loginresponse = client.signin(signin).await;
                match loginresponse {
                    Ok(response) => match response.user {
                        Some(user) => {
                            debug!("Signed in as {}", user.username);
                        }
                        None => {
                            debug!("Signed in as guest");
                        }
                    },
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
    /// Reconnect will attempt to reconnect to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    pub async fn reconnect(&mut self) -> Result<(), OpenIAPError> {
        if self.is_connected() {
            return Ok(());
        }
        if !self.auto_reconnect {
            return Ok(());   
        }
        let client;
        let stream_tx;
        let mut in_stream: ReceiverStream<Envelope> = ReceiverStream::new(mpsc::channel(4).1);
        {
            let inner = self.inner.clone();
            let mut inner = inner.lock().await;
            client = inner.client.clone();
            stream_tx = inner.stream_tx.clone();
            if let ClientEnum::Grpc(_) = client {
                let (new_stream_tx, stream_rx) = mpsc::channel(4);
                in_stream = ReceiverStream::new(stream_rx);
                inner.stream_tx = new_stream_tx;
            }
        } // dropped `inner` to unlock the mutex
    
        let me = Arc::new(Mutex::new(self.clone()));
        let me = Arc::clone(&me);
    
        match client {
            ClientEnum::WS(ref _client) => {
                let me = me.lock().await;
                match me.setup_ws(&me.url, stream_tx).await {
                    Ok(stdin_tx) => {
                        self.stdin_tx = stdin_tx;
                    }
                    Err(e) => {
                        return Err(OpenIAPError::ClientError(format!(
                            "Failed to setup WS: {}",
                            e
                        )));
                    }
                }
            }
            ClientEnum::Grpc(ref _client) => {
                println!("Reconnecting to gRPC");
    
                // Call `setup_grpc_stream` after unlocking `inner`
                self.setup_grpc_stream(in_stream).await?;
                println!("Completed reconnecting to gRPC");
            }
            ClientEnum::None => {
                return Err(OpenIAPError::ClientError("Invalid client".to_string()));
            }
        }
    
        Ok(())
    }
    
    /// Set the connected flag to true or false
    pub fn set_connected(&self, connected: bool, message: Option<&str>) {
        {
            let mut conn = self.connected.lock().unwrap();
            println!("Set connected: {} from {}", connected, *conn);
            if connected == true && *conn == false {
                let me = self.clone();
                tokio::spawn(async move {
                    println!("Connected");
                    let client = me.clone();
                    client.event_sender.send(crate::ClientEvent::Connected).await.unwrap();
                });
            }
            if connected == false && *conn == true {
                let me = self.clone();
                let message = match message {
                    Some(message) => message.to_string(),
                    None => "".to_string(),
                };
                tokio::spawn(async move {
                    println!("Disconnected: {}", message);
                    let client = me.clone();
                    client.event_sender.send(crate::ClientEvent::Disconnected(message)).await.unwrap();
                });
            }
            *conn = connected;
        }
        if !connected {
            let client = self.clone();
            tokio::spawn(async move {
                println!("Reconnecting in 5 seconds");
                tokio::time::sleep(Duration::from_secs(10)).await;
                let mut client = client.clone();
                println!("Reconnecting . . .");
                client.reconnect().await.unwrap_or_else(|e| {
                    error!("Failed to reconnect: {}", e);
                });
            });
        }
    }
    /// Check if the client is connected
    pub fn is_connected(&self) -> bool {
        let conn = self.connected.lock().unwrap();
        println!("is_connected: {}", *conn);
        *conn
    }
    /// Method to allow the user to subscribe with a callback function
    pub async fn on_event<F>(&self, callback: F)
    where
        F: Fn(ClientEvent) + Send + Sync + 'static,
    {
        // call the callback function every time there is an event in the client.event_receiver
        let event_receiver = self.event_receiver.clone();
        let callback = callback;
        tokio::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                callback(event);
            }
        });
    }
    /// internal function, used to setup gRPC stream used for communication with the server.
    /// This function is called by [connect] and should not be called directly.
    /// It will "pre" process stream, watch and queue events, and call future promises, when a response is received.
    #[tracing::instrument(skip_all)]
    async fn setup_grpc_stream(&mut self, in_stream: ReceiverStream<Envelope>) -> Result<(), OpenIAPError> {
        println!("setup_grpc_stream");
        let inner = self.inner.lock().await;
        println!("setup_grpc_stream:1");
        let mut client = match inner.client {
            ClientEnum::Grpc(ref client) => client.clone(),
            _ => {
                return Err(OpenIAPError::ClientError("Invalid client".to_string()));
            }
        };
        println!("setup_grpc_stream:2");
        let response = client.setup_stream(Request::new(in_stream)).await;
        println!("setup_grpc_stream:3");
        let response = match response {
            Ok(response) => response,
            Err(e) => {
                return Err(OpenIAPError::ClientError(format!(
                    "Failed to setup stream: {}",
                    e
                )));
            }
        };
        println!("setup_grpc_stream:4");
        let mut resp_stream = response.into_inner();
        let inner = self.inner.clone();
        tokio::spawn(async move {
            while let Some(received) = resp_stream.next().await {
                if let Ok(received) = received {
                    let inner = inner.lock().await;
                    parse_incomming_envelope(inner.clone(), received).await;
                }
            }
        });
        println!("set connected");
        self.set_connected(true, None);
        Ok(())
    }
      /// Internal function, used to generate a unique id for each message sent to the server.
    #[tracing::instrument(skip_all)]
    fn get_id(&self) -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
    /// Internal function, Send a message to the OpenIAP server, and wait for a response.
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
    /// Internal function, Send a message to the OpenIAP server, and do not wait for a response.
    /// used when sending a stream of data, or when we do not need a response.
    #[tracing::instrument(skip_all)]
    async fn send_noawait(
        &self,
        mut msg: Envelope,
    ) -> Result<(oneshot::Receiver<Envelope>, String), OpenIAPError> {
        {
            if !self.is_connected() {
                return Err(OpenIAPError::ClientError("Not connected".to_string()));
            }
        }
        let (response_tx, response_rx) = oneshot::channel();
        let id = self.get_id().to_string();
        debug!("Sending #{} {} message", id, msg.command);
        msg.id = id.clone();
        {
            trace!("get inner lock");
            let inner = self.inner.lock().await;
            {
                trace!("get query lock");
                inner.queries.lock().await.insert(id.clone(), response_tx);
            }
            trace!("call send");
            let res = send_envelope(self.clone(), inner.to_owned() ,msg).await;
            match res {
                Ok(_) => (),
                Err(e) => return Err(OpenIAPError::ClientError(e.to_string())),
            }
        }
        Ok((response_rx, id))
    }
    /// Internal function, Setup a new stream, send a message to the OpenIAP server, and return a stream to send and receive data.
    #[tracing::instrument(skip_all)]
    async fn sendwithstream(
        &self,
        mut msg: Envelope,
    ) -> Result<(oneshot::Receiver<Envelope>, mpsc::Receiver<Vec<u8>>), OpenIAPError> {
        {
            if !self.is_connected() {
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
            let res = send_envelope(self.clone(),inner.to_owned() ,msg).await;
            match res {
                Ok(_) => (),
                Err(e) => return Err(OpenIAPError::ClientError(e.to_string())),
            }
        }
        Ok((response_rx, stream_rx))
    }
    /// Return true if we are connected and signed in to the OpenIAP service.
    #[tracing::instrument(skip_all)]
    async fn signedin(&self) -> bool {
        let inner = self.inner.lock().await;
        inner.signedin
    }
    /// Return the signed in user, if we are signed in.
    #[tracing::instrument(skip_all)]
    async fn user(&self) -> Option<User> {
        let inner = self.inner.lock().await;
        inner.user.clone()
    }
    /// Internal function, used to send a ping to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    async fn ping(&self) {
        let envelope = Envelope {
            command: "ping".into(),
            ..Default::default()
        };
        let inner = self.inner.lock().await;
        match send_envelope(self.clone(), inner.to_owned() ,envelope).await {
            Ok(_) => (),
            Err(e) => error!("Failed to send ping: {}", e),
        }
    }
    /// Sign in to the OpenIAP service. \
    /// If no username and password is provided, it will attempt to use environment variables.\
    /// if config is set to validateonly, it will only validate the credentials, but not sign in.\
    /// If no jwt, username and password is provided, it will attempt to use environment variables.\
    /// will prefere OPENIAP_JWT (or jwt) over OPENIAP_USERNAME and OPENIAP_PASSWORD.
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
                debug!("Sign-in successful");
                let response: SigninResponse =
                    prost::Message::decode(m.data.as_ref().unwrap().value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                if !config.validateonly {
                    inner.signedin = true;
                    inner.user = Some(response.user.as_ref().unwrap().clone());
                }
                Ok(response)
            }
            Err(e) => {
                debug!("Sending Sign-in request failed {:?}", result);
                debug!("Sign-in failed: {}", e.to_string());
                if !config.validateonly {
                    let mut inner = self.inner.lock().await;
                    inner.signedin = false;
                    inner.user = None;
                }
                Err(OpenIAPError::ClientError(e.to_string()))
            }
        }
    }
    /// Return a list of collections in the database
    /// - includehist: include historical collections, default is false.
    /// ```
    /// use openiap_client::{ Client, OpenIAPError };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let collections = client.list_collections(false).await?;
    ///     println!("Collections: {}", collections);
    ///     Ok(())
    /// }
    /// ```
    #[tracing::instrument(skip_all)]
    pub async fn list_collections(&self, includehist: bool) -> Result<String, OpenIAPError> {
        let config = ListCollectionsRequest::new(includehist);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: ListCollectionsResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.results)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Create a new collection in the database.
    /// You can create a collection by simply adding a new document to it using [Client::insert_one].
    /// Or you can create a collecting using the following example:
    /// ```
    /// use openiap_client::{ Client, CreateCollectionRequest, DropCollectionRequest, OpenIAPError };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let config = CreateCollectionRequest::byname("rusttestcollection");
    ///     client.create_collection(config).await?;
    ///     let config = DropCollectionRequest::byname("rusttestcollection");
    ///     client.drop_collection(config).await?;
    ///     Ok(())
    /// }
    /// ```
    /// You can create a normal collection with a TTL index on the _created field, using the following example:
    /// ```
    /// use openiap_client::{ Client, CreateCollectionRequest, DropCollectionRequest, OpenIAPError };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let config = CreateCollectionRequest::with_ttl(
    ///         "rusttestttlcollection",
    ///         60
    ///     );
    ///     client.create_collection(config).await?;
    ///     let config = DropCollectionRequest::byname("rusttestttlcollection");
    ///     client.drop_collection(config).await?;
    ///     Ok(())
    /// }
    /// ```
    /// You can create a time series collection using the following example:
    /// granularity can be one of: seconds, minutes, hours
    /// ```
    /// use openiap_client::{ Client, CreateCollectionRequest, DropCollectionRequest, OpenIAPError };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let config = CreateCollectionRequest::timeseries(
    ///         "rusttesttscollection2",
    ///         "_created",
    ///         "minutes"
    ///     );
    ///     client.create_collection(config).await?;
    ///     let config = DropCollectionRequest::byname("rusttesttscollection2");
    ///     client.drop_collection(config).await?;
    ///     Ok(())
    /// }
    /// ```
    #[tracing::instrument(skip_all)]
    pub async fn create_collection(
        &self,
        config: CreateCollectionRequest,
    ) -> Result<(), OpenIAPError> {
        if config.collectionname.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No collection name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Drop a collection from the database, this will delete all data and indexes for the collection.
    /// See [Client::create_collection] for examples on how to create a collection.
    #[tracing::instrument(skip_all)]
    pub async fn drop_collection(&self, config: DropCollectionRequest) -> Result<(), OpenIAPError> {
        if config.collectionname.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No collection name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Return all indexes for a collection in the database
    /// ```
    /// use openiap_client::{ Client, GetIndexesRequest, OpenIAPError };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let config = GetIndexesRequest::bycollectionname("rustindextestcollection");
    ///     let indexes = client.get_indexes(config).await?;
    ///     println!("Indexes: {}", indexes);
    ///     Ok(())
    /// }
    /// ```
    /// 
    pub async fn get_indexes(&self, config: GetIndexesRequest) -> Result<String, OpenIAPError> {
        if config.collectionname.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No collection name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: GetIndexesResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.results)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Create an index in the database.
    /// Example of creating an index on the name field in the rustindextestcollection collection, and then dropping it again:
    /// ```
    /// use openiap_client::{ Client, DropIndexRequest, CreateIndexRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let config = CreateIndexRequest::bycollectionname(
    ///         "rustindextestcollection",
    ///         "{\"name\": 1}"
    ///     );
    ///     client.create_index(config).await?;
    ///     let config = DropIndexRequest::bycollectionname(
    ///         "rustindextestcollection",
    ///         "name_1"
    ///     );
    ///     client.drop_index(config).await?;
    ///     Ok(())
    /// }
    /// ```
    /// Example of creating an unique index on the address field in the rustindextestcollection collection, and then dropping it again:
    /// ```
    /// use openiap_client::{ Client, DropIndexRequest, CreateIndexRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let mut config = CreateIndexRequest::bycollectionname(
    ///         "rustindextestcollection",
    ///         "{\"address\": 1}"
    ///     );
    ///     config.options = "{\"unique\": true}".to_string();
    ///     client.create_index(config).await?;
    ///     let config = DropIndexRequest::bycollectionname(
    ///         "rustindextestcollection",
    ///         "address_1"
    ///     );
    ///     client.drop_index(config).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_index(&self, config: CreateIndexRequest) -> Result<(), OpenIAPError> {
        if config.collectionname.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No collection name provided".to_string(),
            ));
        }
        if config.index.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No index was provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Drop an index from the database
    /// See [Client::create_index] for an example on how to create and drop an index.
    pub async fn drop_index(&self, config: DropIndexRequest) -> Result<(), OpenIAPError> {
        if config.collectionname.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No collection name provided".to_string(),
            ));
        }
        if config.name.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No index name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// To query all documents in the entities collection where _type is test, you can use the following example:
    /// ```
    /// use openiap_client::{ OpenIAPError, Client, QueryRequest };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let q = client.query( QueryRequest::with_query(
    ///         "entities",
    ///         "{\"_type\":\"test\"}"
    ///     )).await?;
    ///     let items: serde_json::Value = serde_json::from_str(&q.results).unwrap();
    ///     let items: &Vec<serde_json::Value> = items.as_array().unwrap();
    ///     for item in items {
    ///         println!("Item: {:?}", item);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    /// To query all documents in the entities collection, and only return the name and _id field for all documents, you can use the following example:
    /// ```
    /// use openiap_client::{ OpenIAPError, Client, QueryRequest };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let q = client.query( QueryRequest::with_projection(
    ///         "entities",
    ///         "{}",
    ///         "{\"name\":1}"
    ///     )).await?;
    ///     let items: serde_json::Value = serde_json::from_str(&q.results).unwrap();
    ///     let items: &Vec<serde_json::Value> = items.as_array().unwrap();
    ///     for item in items {
    ///         println!("Item: {:?}", item);
    ///     }
    ///     Ok(())
    /// }
    /// ```
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: QueryResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                debug!("Return Ok(response)");
                Ok(response)
            }
            Err(e) => {
                debug!("Error !!");
                Err(OpenIAPError::ClientError(e.to_string()))
            }
        }
    }
    /// Try and get a single document from the database.\
    /// If no document is found, it will return None.
    /// ```
    /// use openiap_client::{ OpenIAPError, Client, QueryRequest };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let config = QueryRequest::with_query(
    ///         "users",
    ///         "{\"username\":\"guest\"}"
    ///     );
    ///     let item = client.get_one(config).await;
    ///     match item {
    ///         Some(item) => {
    ///             assert_eq!(item["username"], "guest");
    ///             println!("Item: {:?}", item);
    ///         }
    ///         None => {
    ///             println!("No item found");
    ///             assert!(false, "No item found");
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    #[tracing::instrument(skip_all)]
    pub async fn get_one(&self, mut config: QueryRequest) -> Option<serde_json::Value> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }
        config.top = 1;
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => return None,
                };
                if m.command == "error" {
                    return None;
                }
                let response: QueryResponse = prost::Message::decode(data.value.as_ref()).ok()?;

                let items: serde_json::Value = serde_json::from_str(&response.results).unwrap();
                let items: &Vec<serde_json::Value> = items.as_array().unwrap();
                if items.len() == 0 {
                    return None;
                }
                let item = items[0].clone();
                Some(item)
            }
            Err(_) => None,
        }
    }

    /// Try and get a specefic version of a document from the database, reconstructing it from the history collection
    /// ```
    /// use openiap_client::{ OpenIAPError, Client, GetDocumentVersionRequest, InsertOneRequest, UpdateOneRequest };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::connect("").await?;
    ///     let item = "{\"name\": \"test from rust\", \"_type\": \"test\"}";
    ///     let query = InsertOneRequest {
    ///         collectionname: "entities".to_string(),
    ///         item: item.to_string(),
    ///         j: true,
    ///         w: 2,
    ///         ..Default::default()
    ///     };
    ///     let response = client.insert_one(query).await;
    ///     let response = match response {
    ///         Ok(r) => r,
    ///         Err(e) => {
    ///             println!("Error: {:?}", e);
    ///             assert!(false, "insert_one failed with {:?}", e);
    ///             return Ok(());
    ///         }
    ///     };
    ///     let _obj: serde_json::Value = serde_json::from_str(&response.result).unwrap();
    ///     let _id = _obj["_id"].as_str().unwrap();
    ///     let item = format!("{{\"name\":\"updated from rust\", \"_id\": \"{}\"}}", _id);
    ///     let query = UpdateOneRequest {
    ///         collectionname: "entities".to_string(),
    ///         item: item.to_string(),
    ///         ..Default::default()
    ///     };
    ///     let response = client.update_one(query).await;
    ///     _ = match response {
    ///         Ok(r) => r,
    ///         Err(e) => {
    ///             println!("Error: {:?}", e);
    ///             assert!(false, "update_one failed with {:?}", e);
    ///             return Ok(());
    ///         }
    ///     };
    ///     let query = GetDocumentVersionRequest {
    ///         collectionname: "entities".to_string(),
    ///         id: _id.to_string(),
    ///         version: 0,
    ///         ..Default::default()
    ///     };
    ///     let response = client.get_document_version(query).await;
    ///     let response = match response {
    ///         Ok(r) => r,
    ///         Err(e) => {
    ///             println!("Error: {:?}", e);
    ///             assert!(false, "get_document_version failed with {:?}", e);
    ///             return Ok(());
    ///         }
    ///     };
    ///     let _obj = serde_json::from_str(&response);
    ///     let _obj: serde_json::Value = match _obj {
    ///         Ok(r) => r,
    ///         Err(e) => {
    ///             println!("Error: {:?}", e);
    ///             assert!(
    ///                 false,
    ///                 "parse get_document_version result failed with {:?}",
    ///                 e
    ///             );
    ///             return Ok(());
    ///         }
    ///     };
    ///     let name = _obj["name"].as_str().unwrap();
    ///     let version = _obj["_version"].as_i64().unwrap();
    ///     println!("version 0 Name: {}, Version: {}", name, version);
    ///     assert_eq!(name, "test from rust");
    ///     let query = GetDocumentVersionRequest {
    ///         collectionname: "entities".to_string(),
    ///         id: _id.to_string(),
    ///         version: 1,
    ///         ..Default::default()
    ///     };
    ///     let response = client.get_document_version(query).await;
    ///     assert!(
    ///         response.is_ok(),
    ///         "test_get_document_version failed with {:?}",
    ///         response.err().unwrap()
    ///     );
    ///     let _obj: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
    ///     let name = _obj["name"].as_str().unwrap();
    ///     let version = _obj["_version"].as_i64().unwrap();
    ///     println!("version 1 Name: {}, Version: {}", name, version);
    ///     assert_eq!(name, "updated from rust");
    ///     let query = GetDocumentVersionRequest {
    ///         collectionname: "entities".to_string(),
    ///         id: _id.to_string(),
    ///         version: -1,
    ///         ..Default::default()
    ///     };
    ///     let response = client.get_document_version(query).await;
    ///     assert!(
    ///         response.is_ok(),
    ///         "test_get_document_version failed with {:?}",
    ///         response.err().unwrap()
    ///     );
    ///     let _obj: serde_json::Value = serde_json::from_str(&response.unwrap()).unwrap();
    ///     let name = _obj["name"].as_str().unwrap();
    ///     let version = _obj["_version"].as_i64().unwrap();
    ///     println!("version -1 Name: {}, Version: {}", name, version);
    ///     assert_eq!(name, "updated from rust");
    ///     Ok(())
    /// }
    /// ```
    #[tracing::instrument(skip_all)]
    pub async fn get_document_version(
        &self,
        mut config: GetDocumentVersionRequest,
    ) -> Result<String, OpenIAPError> {
        if config.collectionname.is_empty() {
            config.collectionname = "entities".to_string();
        }
        if config.id.is_empty() {
            return Err(OpenIAPError::ClientError("No id provided".to_string()));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: GetDocumentVersionResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.result)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Run an aggregate pipeline towards the database
    /// Example of running an aggregate pipeline on the entities collection, counting the number of documents with _type=test, and grouping them by name:
    /// ```
    /// use openiap_client::{ OpenIAPError, Client, AggregateRequest };
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///    let client = Client::connect("").await?;
    ///     let config = AggregateRequest {
    ///         collectionname: "entities".to_string(),
    ///         aggregates: "[{\"$match\": {\"_type\": \"test\"}}, {\"$group\": {\"_id\": \"$name\", \"count\": {\"$sum\": 1}}}]".to_string(),
    ///         ..Default::default()
    ///     };
    ///     let response = client.aggregate(config).await?;
    ///     println!("Response: {:?}", response);
    ///     Ok(())
    /// }
    /// ```
    /// 
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: AggregateResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Count the number of documents in a collection, with an optional query
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: CountResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Get distinct values for a field in a collection, with an optional query
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: DistinctResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Insert a document into a collection
    #[tracing::instrument(skip_all)]
    pub async fn insert_one(
        &self,
        config: InsertOneRequest,
    ) -> Result<InsertOneResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertOneResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Insert many documents into a collection
    #[tracing::instrument(skip_all)]
    pub async fn insert_many(
        &self,
        config: InsertManyRequest,
    ) -> Result<InsertManyResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertManyResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Update ( replace ) a document in a collection
    #[tracing::instrument(skip_all)]
    pub async fn update_one(
        &self,
        config: UpdateOneRequest,
    ) -> Result<UpdateOneResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: UpdateOneResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Using a unique key, insert a document or update it if it already exists ( upsert on steroids )
    #[tracing::instrument(skip_all)]
    pub async fn insert_or_update_one(
        &self,
        config: InsertOrUpdateOneRequest,
    ) -> Result<String, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertOrUpdateOneResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.result)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Using a unique key, insert many documents or update them if they already exist ( upsert on steroids )
    #[tracing::instrument(skip_all)]
    pub async fn insert_or_update_many(
        &self,
        config: InsertOrUpdateManyRequest,
    ) -> Result<InsertOrUpdateManyResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: InsertOrUpdateManyResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Update one or more documents in a collection using a update document
    #[tracing::instrument(skip_all)]
    pub async fn update_document(
        &self,
        config: UpdateDocumentRequest,
    ) -> Result<UpdateDocumentResponse, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: UpdateDocumentResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete a document from a collection using a unique key
    #[tracing::instrument(skip_all)]
    pub async fn delete_one(&self, config: DeleteOneRequest) -> Result<i32, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: DeleteOneResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.affectedrows)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete many documents from a collection using a query or list of unique keys
    #[tracing::instrument(skip_all)]
    pub async fn delete_many(&self, config: DeleteManyRequest) -> Result<i32, OpenIAPError> {
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: DeleteManyResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.affectedrows)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Download a file from the database
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
                while !stream_rx.is_closed() {
                    match stream_rx.recv().await {
                        Some(received) => {
                            if received.is_empty() {
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
                    let data = match response.data {
                        Some(data) => data,
                        None => {
                            return Err(OpenIAPError::ClientError(
                                "No data returned for SERVER error".to_string(),
                            ));
                        }
                    };
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref()).unwrap();
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
    /// Upload a file to the database
    #[tracing::instrument(skip_all)]
    pub async fn upload(
        &self,
        config: UploadRequest,
        filepath: &str,
    ) -> Result<UploadResponse, OpenIAPError> {
        debug!("upload: Uploading file: {}", filepath);
        let mut file = File::open(filepath)
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to open file: {}", e)))?;
        let chunk_size = 1024 * 1024;
        let mut buffer = vec![0; chunk_size];

        let envelope = config.to_envelope();
        let (response_rx, rid) = self.send_noawait(envelope).await?;
        {
            let inner = self.inner.lock().await;

            let envelope = BeginStream::from_rid(rid.clone());
            debug!("Sending beginstream to #{}", rid);
            send_envelope(self.clone(), inner.to_owned(), envelope).await.map_err(|e| OpenIAPError::ClientError(format!("Failed to send data: {}", e)))?;
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
                send_envelope(self.clone(), inner.to_owned() ,envelope).await.map_err(|e| {
                    OpenIAPError::ClientError(format!("Failed to send data: {}", e))
                })?
            }

            let envelope = EndStream::from_rid(rid.clone());
            debug!("Sending endstream to #{}", rid);
            send_envelope(self.clone(), inner.to_owned(), envelope).await
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
    /// Watch for changes in a collection ( change stream )
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: WatchResponse = prost::Message::decode(data.value.as_ref())
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
    /// Cancel a watch ( change stream )
    #[tracing::instrument(skip_all)]
    pub async fn unwatch(&self, id: &str) -> Result<(), OpenIAPError> {
        let config = UnWatchRequest::byid(id);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Register a queue for messaging ( amqp ) in the OpenIAP service
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: RegisterQueueResponse =
                    prost::Message::decode(data.value.as_ref())
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
    /// Unregister a queue or exchange for messaging ( amqp ) in the OpenIAP service
    #[tracing::instrument(skip_all)]
    pub async fn unregister_queue(&self, queuename: &str) -> Result<(), OpenIAPError> {
        let config = UnRegisterQueueRequest::byqueuename(queuename);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Register a exchange for messaging ( amqp ) in the OpenIAP service
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: RegisterExchangeResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                if !response.queuename.is_empty() {
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
    /// Send a message to a queue or exchange in the OpenIAP service
    #[tracing::instrument(skip_all)]
    pub async fn queue_message(
        &self,
        config: QueueMessageRequest,
    ) -> Result<QueueMessageResponse, OpenIAPError> {
        if config.queuename.is_empty() && config.exchangename.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No queue or exchange name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()))
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: QueueMessageResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Send message to a queue or exchange in the OpenIAP service, and wait for a reply
    #[tracing::instrument(skip_all)]
    pub async fn rpc(&self, mut config: QueueMessageRequest) -> Result<String, OpenIAPError> {
        if config.queuename.is_empty() && config.exchangename.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No queue or exchange name provided".to_string(),
            ));
        }

        let (tx, rx) = oneshot::channel::<String>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let q = self
            .register_queue(
                RegisterQueueRequest {
                    queuename: "".to_string(),
                },
                Box::new(move |event| {
                    let tx = tx.lock().unwrap().take().unwrap();
                    tx.send(event.data).unwrap();
                }),
            )
            .await
            .unwrap();

        config.replyto = q.clone();
        let envelope = config.to_envelope();

        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()))
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;

                let response = rx.await.unwrap();

                let ur_response = self.unregister_queue(&q).await;
                match ur_response {
                    Ok(_) => {
                        debug!("Unregistered Response Queue: {:?}", q);
                    }
                    Err(e) => {
                        error!("Failed to unregister Response Queue: {:?}", e);
                    }
                }

                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Push a new workitem to a workitem queue
    /// If the file is less than 5 megabytes it will be attached to the workitem
    /// If the file is larger than 5 megabytes it will be uploaded to the database and attached to the workitem
    #[tracing::instrument(skip_all)]
    pub async fn push_workitem(
        &self,
        mut config: PushWorkitemRequest,
    ) -> Result<PushWorkitemResponse, OpenIAPError> {
        if config.wiq.is_empty() && config.wiqid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No queue name or id provided".to_string(),
            ));
        }
        for f in &mut config.files {
            if f.filename.is_empty() && f.file.is_empty() {
                debug!("Filename is empty");
            } else if !f.filename.is_empty() && f.file.is_empty() && f.id.is_empty() {
                // does file exist?
                if !std::path::Path::new(&f.filename).exists() {
                    debug!("File does not exist: {}", f.filename);
                } else {
                    let filesize = std::fs::metadata(&f.filename).unwrap().len();
                    // if filesize is less than 5 meggabytes attach it, else upload
                    if filesize < 5 * 1024 * 1024 {
                        debug!("File {} exists so ATTACHING it.", f.filename);
                        let filename = std::path::Path::new(&f.filename)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap();
                        f.file = std::fs::read(&f.filename).unwrap();
                        // f.file = compress_file(&f.filename).unwrap();
                        // f.compressed = false;
                        f.file = compress_file_to_vec(&f.filename).unwrap();
                        f.compressed = true;
                        f.filename = filename.to_string();
                        f.id = "findme".to_string();
                        trace!(
                            "File {} was read and assigned to f.file, size: {}",
                            f.filename,
                            f.file.len()
                        );
                    } else {
                        debug!("File {} exists so UPLOADING it.", f.filename);
                        let filename = std::path::Path::new(&f.filename)
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap();
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
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: PushWorkitemResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Push multiple workitems to a workitem queue
    /// If the file is less than 5 megabytes it will be attached to the workitem
    /// If the file is larger than 5 megabytes it will be uploaded to the database and attached to the workitem
    #[tracing::instrument(skip_all)]
    pub async fn push_workitems(
        &self,
        mut config: PushWorkitemsRequest,
    ) -> Result<PushWorkitemsResponse, OpenIAPError> {
        if config.wiq.is_empty() && config.wiqid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No queue name or id provided".to_string(),
            ));
        }
        for wi in &mut config.items {
            for f in &mut wi.files {
                if f.filename.is_empty() && f.file.is_empty() {
                    debug!("Filename is empty");
                } else if !f.filename.is_empty() && f.file.is_empty() && f.id.is_empty() {
                    // does file exist?
                    if !std::path::Path::new(&f.filename).exists() {
                        debug!("File does not exist: {}", f.filename);
                    } else {
                        let filesize = std::fs::metadata(&f.filename).unwrap().len();
                        // if filesize is less than 5 meggabytes attach it, else upload
                        if filesize < 5 * 1024 * 1024 {
                            debug!("File {} exists so ATTACHING it.", f.filename);
                            let filename = std::path::Path::new(&f.filename)
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap();
                            f.file = std::fs::read(&f.filename).unwrap();
                            // f.file = compress_file(&f.filename).unwrap();
                            // f.compressed = false;
                            f.file = compress_file_to_vec(&f.filename).unwrap();
                            f.compressed = true;
                            f.filename = filename.to_string();
                            f.id = "findme".to_string();
                            trace!(
                                "File {} was read and assigned to f.file, size: {}",
                                f.filename,
                                f.file.len()
                            );
                        } else {
                            debug!("File {} exists so UPLOADING it.", f.filename);
                            let filename = std::path::Path::new(&f.filename)
                                .file_name()
                                .unwrap()
                                .to_str()
                                .unwrap();
                            let uploadconfig = UploadRequest {
                                filename: filename.to_string(),
                                collectionname: "fs.files".to_string(),
                                ..Default::default()
                            };
                            let uploadresult =
                                self.upload(uploadconfig, &f.filename).await.unwrap();
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
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: PushWorkitemsResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Pop a workitem from a workitem queue, return None if no workitem is available
    /// Any files attached to the workitem will be downloaded to the downloadfolder ( default "." )
    #[tracing::instrument(skip_all)]
    pub async fn pop_workitem(
        &self,
        config: PopWorkitemRequest,
        downloadfolder: Option<&str>,
    ) -> Result<PopWorkitemResponse, OpenIAPError> {
        if config.wiq.is_empty() && config.wiqid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No queue name or id provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: PopWorkitemResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;

                match &response.workitem {
                    Some(wi) => {
                        for f in &wi.files {
                            if !f.id.is_empty() {
                                let downloadconfig = DownloadRequest {
                                    id: f.id.clone(),
                                    collectionname: "fs.files".to_string(),
                                    ..Default::default()
                                };
                                let downloadresult =
                                    match self.download(downloadconfig, downloadfolder, None).await
                                    {
                                        Ok(r) => r,
                                        Err(e) => {
                                            debug!("Failed to download file: {}", e);
                                            continue;
                                        }
                                    };
                                debug!(
                                    "File {} was downloaded as {}",
                                    f.filename, downloadresult.filename
                                );
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
    /// Update a workitem in a workitem queue
    /// If the file is less than 5 megabytes it will be attached to the workitem
    /// If the file is larger than 5 megabytes it will be uploaded to the database and attached to the workitem
    /// If a fileid is provided it will be used to update the file
    /// if a filename is provided without the id, it will be deleted
    #[tracing::instrument(skip_all)]
    pub async fn update_workitem(
        &self,
        mut config: UpdateWorkitemRequest,
    ) -> Result<UpdateWorkitemResponse, OpenIAPError> {
        match &config.workitem {
            Some(wiq) => {
                if wiq.id.is_empty() {
                    return Err(OpenIAPError::ClientError(
                        "No workitem id provided".to_string(),
                    ));
                }
            }
            None => {
                return Err(OpenIAPError::ClientError(
                    "No workitem provided".to_string(),
                ));
            }
        }
        for f in &mut config.files {
            if f.filename.is_empty() && f.file.is_empty() {
                debug!("Filename is empty");
            } else if !f.filename.is_empty() && f.file.is_empty() && f.id.is_empty() {
                if !std::path::Path::new(&f.filename).exists() {
                    debug!("File does not exist: {}", f.filename);
                } else {
                    debug!("File {} exists so uploading it.", f.filename);
                    let filename = std::path::Path::new(&f.filename)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    let uploadconfig = UploadRequest {
                        filename: filename.to_string(),
                        collectionname: "fs.files".to_string(),
                        ..Default::default()
                    };
                    let uploadresult = self.upload(uploadconfig, &f.filename).await.unwrap();
                    trace!("File {} was upload as {}", filename, uploadresult.id);
                    f.id = uploadresult.id.clone();
                    f.filename = filename.to_string();
                }
            } else {
                debug!("Skipped file");
            }
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: UpdateWorkitemResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete a workitem from a workitem queue
    #[tracing::instrument(skip_all)]
    pub async fn delete_workitem(
        &self,
        config: DeleteWorkitemRequest,
    ) -> Result<DeleteWorkitemResponse, OpenIAPError> {
        if config.id.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No workitem id provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: DeleteWorkitemResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Add a workitem queue to openiap instance
    #[tracing::instrument(skip_all)]
    pub async fn add_workitem_queue(
        &self,
        config: AddWorkItemQueueRequest,
    ) -> Result<WorkItemQueue, OpenIAPError> {
        if config.workitemqueue.is_none() {
            return Err(OpenIAPError::ClientError(
                "No workitem queue name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: AddWorkItemQueueResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                match response.workitemqueue {
                    Some(wiq) => Ok(wiq),
                    None => {
                        return Err(OpenIAPError::ClientError(
                            "No workitem queue returned".to_string(),
                        ));
                    }
                }
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Update a workitem queue in openiap instance
    #[tracing::instrument(skip_all)]
    pub async fn update_workitem_queue(
        &self,
        config: UpdateWorkItemQueueRequest,
    ) -> Result<WorkItemQueue, OpenIAPError> {
        if config.workitemqueue.is_none() {
            return Err(OpenIAPError::ClientError(
                "No workitem queue name provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: UpdateWorkItemQueueResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                match response.workitemqueue {
                    Some(wiq) => Ok(wiq),
                    None => {
                        return Err(OpenIAPError::ClientError(
                            "No workitem queue returned".to_string(),
                        ));
                    }
                }
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete a workitem queue from openiap instance
    #[tracing::instrument(skip_all)]
    pub async fn delete_workitem_queue(
        &self,
        config: DeleteWorkItemQueueRequest,
    ) -> Result<(), OpenIAPError> {
        if config.wiq.is_empty() && config.wiqid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No workitem queue name or id provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Run custom command on server. Custom commands are commands who is "on trail", they may change and are not ready to be moved to the fixed protobuf format yet
    #[tracing::instrument(skip_all)]
    pub async fn custom_command(
        &self,
        config: CustomCommandRequest,
    ) -> Result<String, OpenIAPError> {
        if config.command.is_empty() {
            return Err(OpenIAPError::ClientError("No command provided".to_string()));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: CustomCommandResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.result)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete a package from the database, cleaning up all all files and data
    #[tracing::instrument(skip_all)]
    pub async fn delete_package(&self, packageid: &str) -> Result<(), OpenIAPError> {
        let config = DeletePackageRequest::byid(packageid);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(data) => data,
                    None => {
                        return Err(OpenIAPError::ClientError("No data returned".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Start Agent
    #[tracing::instrument(skip_all)]
    pub async fn start_agent(&self, agentid: &str) -> Result<(), OpenIAPError> {
        let config = StartAgentRequest::byid(agentid);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Stop an agent, this will cleanup all resources and stop the agent
    #[tracing::instrument(skip_all)]
    pub async fn stop_agent(&self, agentid: &str) -> Result<(), OpenIAPError> {
        let config = StopAgentRequest::byid(agentid);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete a pod from an agent, on kubernetes this will remove the pod and kubernetes will re-create it, on docker this will remove the pod. Then use start_agent to start the agent again
    #[tracing::instrument(skip_all)]
    pub async fn delete_agent_pod(&self, agentid: &str, podname: &str) -> Result<(), OpenIAPError> {
        let config = DeleteAgentPodRequest::byid(agentid, podname);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Delete an agent, this will cleanup all resources and delete the agent
    #[tracing::instrument(skip_all)]
    pub async fn delete_agent(&self, agentid: &str) -> Result<(), OpenIAPError> {
        let config = DeleteAgentRequest::byid(agentid);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(())
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Get all pods associated with an agent, if stats is true, it will return memory and cpu usage for each pod
    #[tracing::instrument(skip_all)]
    pub async fn get_agent_pods(&self, agentid: &str, stats: bool) -> Result<String, OpenIAPError> {
        let config = GetAgentPodsRequest::byid(agentid, stats);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: GetAgentPodsResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.results)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Get logs from a pod associated with an agent, leave podname empty to get logs from all pods
    #[tracing::instrument(skip_all)]
    pub async fn get_agent_pod_logs(
        &self,
        agentid: &str,
        podname: &str,
    ) -> Result<String, OpenIAPError> {
        let config = GetAgentLogRequest::new(agentid, podname);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: GetAgentLogResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.result)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }

    /// Create/update a customer in the OpenIAP service. If stripe has been configured, it will create or update a customer in stripe as well
    /// A customer is a customer object that can only be updated using this function, and 2 roles ( customername admins and customername users )
    #[tracing::instrument(skip_all)]
    pub async fn ensure_customer(
        &self,
        config: EnsureCustomerRequest,
    ) -> Result<EnsureCustomerResponse, OpenIAPError> {
        if config.customer.is_none() && config.stripe.is_none() {
            return Err(OpenIAPError::ClientError(
                "No customer or stripe provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: EnsureCustomerResponse = prost::Message::decode(data.value.as_ref())
                    .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
    /// Create a new workflow instance, to be used to workflow in/out nodes in NodeRED
    #[tracing::instrument(skip_all)]
    pub async fn create_workflow_instance(
        &self,
        config: CreateWorkflowInstanceRequest,
    ) -> Result<String, OpenIAPError> {
        if config.workflowid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No workflow id provided".to_string(),
            ));
        }
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()));
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                let response: CreateWorkflowInstanceResponse =
                    prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                Ok(response.instanceid)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }

    /// Invoke a workflow in the OpenRPA robot where robotid is the userid of the user the robot is running as, or a roleid with RPA enabled
    #[tracing::instrument(skip_all)]
    pub async fn invoke_openrpa(
        &self,
        config: InvokeOpenRpaRequest,
    ) -> Result<String, OpenIAPError> {
        if config.robotid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No robot id provided".to_string(),
            ));
        }
        if config.workflowid.is_empty() {
            return Err(OpenIAPError::ClientError(
                "No workflow id provided".to_string(),
            ));
        }

        let (tx, rx) = oneshot::channel::<String>();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let q = self
            .register_queue(
                RegisterQueueRequest {
                    queuename: "".to_string(),
                },
                Box::new(move |event| {
                    let json = event.data.clone();
                    let obj = serde_json::from_str::<serde_json::Value>(&json).unwrap();
                    let command: String = obj["command"].as_str().unwrap().to_string();
                    println!("Received event: {:?}", event);
                    if command.eq("invokesuccess") {
                        println!("Robot successfully started running workflow");
                    } else if command.eq("invokeidle") {
                        println!("Workflow went idle");
                    } else if command.eq("invokeerror") {
                        println!("Robot failed to run workflow");
                        let tx = tx.lock().unwrap().take().unwrap();
                        tx.send(event.data).unwrap();
                    } else if command.eq("timeout") {
                        println!("No robot picked up the workflow");
                        let tx = tx.lock().unwrap().take().unwrap();
                        tx.send(event.data).unwrap();
                    } else if command.eq("invokecompleted") {
                        println!("Robot completed running workflow");
                        let tx = tx.lock().unwrap().take().unwrap();
                        tx.send(event.data).unwrap();
                    } else {
                        let tx = tx.lock().unwrap().take().unwrap();
                        tx.send(event.data).unwrap();
                    }
                }),
            )
            .await
            .unwrap();
        println!("Registered Response Queue: {:?}", q);
        let data = format!(
            "{{\"command\":\"invoke\",\"workflowid\":\"{}\",\"payload\": {}}}",
            config.workflowid, config.payload
        );
        println!("Send Data: {}", data);
        println!("To Queue: {} With reply to: {}", config.robotid, q);
        let config = QueueMessageRequest {
            queuename: config.robotid.clone(),
            replyto: q.clone(),
            data,
            ..Default::default()
        };

        let envelope = config.to_envelope();

        let result = self.send(envelope).await;
        match result {
            Ok(m) => {
                let data = match m.data {
                    Some(d) => d,
                    None => {
                        return Err(OpenIAPError::ClientError("No data in response".to_string()))
                    }
                };
                if m.command == "error" {
                    let e: ErrorResponse = prost::Message::decode(data.value.as_ref())
                        .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;
                    return Err(OpenIAPError::ServerError(format!("{:?}", e.message)));
                }
                // prost::Message::decode(data.value.as_ref())
                //     .map_err(|e| OpenIAPError::CustomError(e.to_string()))?;

                let json = rx.await.unwrap();
                println!("Received json result: {:?}", json);
                let obj = serde_json::from_str::<serde_json::Value>(&json).unwrap();
                let command: String = obj["command"].as_str().unwrap().to_string();
                let mut data = "".to_string();
                if !obj["data"].as_str().is_none() {
                    data = obj["data"].as_str().unwrap().to_string();
                } else if !obj["data"].as_object().is_none() {
                    data = obj["data"].to_string();
                }
                if !command.eq("invokecompleted") {
                    if command.eq("timeout") {
                        return Err(OpenIAPError::ServerError("Timeout".to_string()));
                    } else {
                        if data.is_empty() {
                            return Err(OpenIAPError::ServerError(
                                "Error with no message".to_string(),
                            ));
                        }
                        return Err(OpenIAPError::ServerError(data));
                    }
                }
                let response = self.unregister_queue(&q).await;
                match response {
                    Ok(_) => {
                        debug!("Unregistered Response Queue: {:?}", q);
                    }
                    Err(e) => {
                        error!("Failed to unregister Response Queue: {:?}", e);
                    }
                }
                Ok(data)
            }
            Err(e) => Err(OpenIAPError::ClientError(e.to_string())),
        }
    }
}

