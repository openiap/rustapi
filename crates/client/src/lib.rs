#![warn(missing_docs)]
//! The `openiap.client` crate provides the [Client] struct and its methods.
//! For now this is only the GRPC and WebSocket client, later we will add a web rest, named pipe and TCP client as well.
//! Initialize a new client, by calling the [Client::new_connect] method.
//! ```
//! use openiap_client::{OpenIAPError, Client, QueryRequest};
//! #[tokio::main]
//! async fn main() -> Result<(), OpenIAPError> {
//!     let client = Client::new_connect("").await?;
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

pub use openiap_proto::errors::*;
pub use openiap_proto::protos::*;
pub use openiap_proto::*;
pub use prost_types::Timestamp;
pub use protos::flow_service_client::FlowServiceClient;
use sqids::Sqids;

use tokio::task::JoinHandle;
use tokio_tungstenite::{WebSocketStream};
use tracing::{debug, error, info, trace};
type StdError = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = StdError> = ::std::result::Result<T, E>;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::Channel;

use tokio::sync::{mpsc, oneshot};

use std::env;
use std::time::Duration;

#[cfg(feature = "otel")]
mod otel;
mod tests;
mod ws;
mod grpc;
mod util;
pub use crate::util::{enable_tracing, disable_tracing};

type QuerySender = oneshot::Sender<Envelope>;
type StreamSender = mpsc::Sender<Vec<u8>>;
type Sock = WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
use futures::{StreamExt };
use async_channel::{unbounded};
const VERSION: &str = "0.0.18";


/// The `Client` struct provides the client for the OpenIAP service.
/// Initialize a new client, by calling the [Client::new_connect] method.
#[derive(Clone)]
pub struct Client {
    /// Ensure we use only call connect once, and then use re-connect instead.
    connect_called: Arc<std::sync::Mutex<bool>>,
    /// The tokio runtime.
    runtime: Arc<std::sync::Mutex<Option<tokio::runtime::Runtime>>>,

    /// Keep track of usage of the client
    stats: Arc<std::sync::Mutex<ClientStatistics>>,

    task_handles: Arc<std::sync::Mutex<Vec<JoinHandle<()>>>>,
    /// The inner client object
    pub client: Arc<std::sync::Mutex<ClientEnum>>,
    /// The signed in user.
    user: Arc<std::sync::Mutex<Option<User>>>,
    /// The inner client.
    pub inner: Arc<Mutex<ClientInner>>,
    /// The `Config` struct provides the configuration for the OpenIAP service we are connecting to.
    pub config: Arc<std::sync::Mutex<Option<Config>>>,
    /// Should client automatically reconnect, if disconnected?
    pub auto_reconnect: Arc<std::sync::Mutex<bool>>,
    /// URL used to connect to server, processed and without credentials
    pub url: Arc<std::sync::Mutex<String>>,
    /// Username used to connect to server
    pub username: Arc<std::sync::Mutex<String>>,
    /// Password used to connect to server
    pub password: Arc<std::sync::Mutex<String>>,
    /// JWT token used to connect to server
    pub jwt: Arc<std::sync::Mutex<String>>,
    agent_name: Arc<std::sync::Mutex<String>>,
    agent_version: Arc<std::sync::Mutex<String>>,
    event_sender: async_channel::Sender<ClientEvent>,
    event_receiver: async_channel::Receiver<ClientEvent>,
    out_envelope_sender: async_channel::Sender<Envelope>,
    out_envelope_receiver: async_channel::Receiver<Envelope>,
    /// The client connection state.
    pub state: Arc<std::sync::Mutex<ClientState>>,
    /// Inceasing message count, used as unique id for messages.
    pub msgcount: Arc<std::sync::Mutex<i32>>,
    /// Reconnect interval in milliseconds, this will slowly increase if we keep getting disconnected.
    pub reconnect_ms: Arc<std::sync::Mutex<i32>>,
    
}
/// The `ClientStatistics` struct provides the statistics for usage of the client
#[derive(Clone, Default)]
pub struct ClientStatistics {
    connection_attempts: u64,
    connections: u64,
    package_tx: u64,
    package_rx: u64,
    signin: u64,
    download: u64,
    getdocumentversion: u64,
    customcommand: u64,
    listcollections: u64,
    createcollection: u64,
    dropcollection: u64,
    ensurecustomer: u64,
    invokeopenrpa: u64,
    registerqueue: u64,
    registerexchange: u64,
    unregisterqueue: u64,
    watch: u64,
    unwatch: u64,
    queuemessage: u64,
    pushworkitem: u64,
    pushworkitems: u64,
    popworkitem: u64,
    updateworkitem: u64,
    deleteworkitem: u64,
    addworkitemqueue: u64,
    updateworkitemqueue: u64,
    deleteworkitemqueue: u64,
    getindexes: u64,
    createindex: u64,
    dropindex: u64,
    upload: u64,
    query: u64,
    count: u64,
    distinct: u64,
    aggregate: u64,
    insertone: u64,
    insertmany: u64,
    insertorupdateone: u64,
    insertorupdatemany: u64,
    updateone: u64,
    updatedocument: u64,
    deleteone: u64,
    deletemany: u64,
}
/// The `ClientInner` struct provides the inner client for the OpenIAP service.
#[derive(Clone)]
pub struct ClientInner {
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
/// Client enum, used to determine which client to use.
#[derive(Clone, Debug)]
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
    Connecting,
    /// The client has connected
    Connected,
    /// The client has disconnected
    Disconnected(String),
    /// The client has signed in
    SignedIn,
    // The client has signed out
    // SignedOut,
    // The client has received a message
    // Message(Envelope),
    // The client has received a ping event from the server
    // Ping,
    // The client has received a stream
    // Stream(Vec<u8>),
    // The client has received a watch event
    // Watch(WatchEvent),
    // The client has received a queue event
    // Queue(QueueEvent),
}
/// Client event enum, used to determine which event has occurred.
#[derive(Debug, Clone, PartialEq)]
pub enum ClientState {
    /// The client is disconnected
    Disconnected,
    /// The client connecting
    Connecting,
    /// The client is connected
    Connected,
    /// The client is signed in and connected
    Signedin
}
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
    #[serde(default)]
    enable_analytics: bool,
}
impl std::fmt::Debug for ClientInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientInner")
            // .field("client", &self.client)
            .field("queries", &self.queries)
            .field("streams", &self.streams)
            .finish()
    }
}
impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Client {
    /// Create a new client.
    pub fn new() -> Self {
        let (ces, cer) = unbounded::<ClientEvent>();
        let (out_es, out_er) = unbounded::<Envelope>();
        Self {
            task_handles: Arc::new(std::sync::Mutex::new(Vec::new())),
            stats: Arc::new(std::sync::Mutex::new(ClientStatistics::default())),
            user: Arc::new(std::sync::Mutex::new(None)),
            client: Arc::new(std::sync::Mutex::new(ClientEnum::None)),
            connect_called: Arc::new(std::sync::Mutex::new(false)),
            runtime: Arc::new(std::sync::Mutex::new(None)),
            msgcount: Arc::new(std::sync::Mutex::new(-1)),
            reconnect_ms: Arc::new(std::sync::Mutex::new(1000)),
            inner: Arc::new(Mutex::new(ClientInner {
                queries: Arc::new(Mutex::new(std::collections::HashMap::new())),
                streams: Arc::new(Mutex::new(std::collections::HashMap::new())),
                watches: Arc::new(Mutex::new(std::collections::HashMap::new())),
                queues: Arc::new(Mutex::new(std::collections::HashMap::new())),
            })),
            config: Arc::new(std::sync::Mutex::new(None)),
            auto_reconnect: Arc::new(std::sync::Mutex::new(true)),
            url: Arc::new(std::sync::Mutex::new("".to_string())),
            username: Arc::new(std::sync::Mutex::new("".to_string())),
            password: Arc::new(std::sync::Mutex::new("".to_string())),
            jwt: Arc::new(std::sync::Mutex::new("".to_string())),
            agent_name: Arc::new(std::sync::Mutex::new("rust".to_string())),
            agent_version: Arc::new(std::sync::Mutex::new(VERSION.to_string())),
            event_sender: ces,
            event_receiver: cer,
            out_envelope_sender: out_es,
            out_envelope_receiver: out_er,
            state: Arc::new(std::sync::Mutex::new(ClientState::Disconnected)),
        }
    }
    /// Connect the client to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    pub fn connect(&self, dst: &str) -> Result<(), OpenIAPError> {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(e) => {
                return Err(OpenIAPError::ClientError(format!(
                    "Failed to create tokio runtime: {}",
                    e
                )));
            }
        };
        self.set_runtime(Some(rt));
        tokio::task::block_in_place(|| {
            let handle = self.get_runtime_handle();
            handle.block_on(self.connect_async(dst))
        })
    }

    /// Load the configuration from the server.
    #[allow(unused_variables)]
    pub async fn load_config(&self, strurl: &str, url: &url::Url) -> Option<Config> {
        let config: Option<Config>;
        let issecure = url.scheme() == "https" || url.scheme() == "wss" || url.port() == Some(443);
        let mut port = url.port().unwrap_or(80);
        if port == 50051 {
            port = 3000;
        }
        let configurl = if issecure {
            format!(
                "{}://{}:{}/config",
                "https",
                url.host_str()
                    .unwrap_or("localhost.openiap.io")
                    .replace("grpc.", ""),
                port
            )
        } else {
            format!(
                "{}://{}:{}/config",
                "http",
                url.host_str()
                    .unwrap_or("localhost.openiap.io")
                    .replace("grpc.", ""),
                port
            )
        };

        let configurl = url::Url::parse(configurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e))).expect("wefew");
        let o = minreq::get(configurl).send();
        match o {
            Ok(_) => {
                let response = match o {
                    Ok(response) => response,
                    Err(e) => {
                        error!("Failed to get config: {}", e);
                        return None;
                    }
                };
                if response.status_code == 200 {
                    let body = response.as_str().unwrap();
                    config = Some(match serde_json::from_str(body) {
                        Ok(config) => config,
                        Err(e) => {
                            error!("Failed to parse config: {}", e);
                            return None;
                        }
                    });
                } else {
                    config = None;
                }
            }
            Err(e) => {
                error!("Failed to get config: {}", e);
                return None;
            }
        }
        let mut _enable_analytics = true;
        let mut _otel_metric_url = std::env::var("OTEL_METRIC_URL").unwrap_or_default();
        let mut apihostname = url.host_str().unwrap_or("localhost.openiap.io").to_string();
        if apihostname.starts_with("grpc.") {
            apihostname = apihostname[5..].to_string();
        }
    
        if config.is_some() {
            let config = config.as_ref().unwrap();
            if !config.otel_metric_url.is_empty() {
                _otel_metric_url = config.otel_metric_url.clone();
            }
            if !config.domain.is_empty() {
                apihostname = config.domain.clone();
            }
            _enable_analytics = config.enable_analytics;
        }
        #[cfg(feature = "otel")]
        if _enable_analytics {
            let agent_name = self.get_agent_name();
            let agent_version = self.get_agent_version();
            match otel::init_telemetry(&agent_name, &agent_version, VERSION, &apihostname, _otel_metric_url.as_str(), &self.stats) {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to initialize telemetry: {}", e);
                    return None;
                }
            }
        }
        config
    }

    /// Connect the client to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    pub async fn connect_async(&self, dst: &str) -> Result<(), OpenIAPError> {
        #[cfg(test)]
        {   
            // enable_tracing("openiap=trace", "new");
            // enable_tracing("openiap=debug", "new");
            // enable_tracing("trace", "");
            enable_tracing("openiap=error", "");
            // enable_tracing("openiap=debug", "");
        }
        if self.is_connect_called() {
            self.set_auto_reconnect(true);
            return self.reconnect().await;
        }
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
        let usegprc = url.scheme() == "grpc" || url.domain().unwrap_or("localhost").to_lowercase().starts_with("grpc.") || url.port() == Some(50051);
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
                strurl = format!("http://{}:{}", url.host_str().unwrap_or("app.openiap.io"), url.port().unwrap_or(80));
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
        if !username.is_empty() && !password.is_empty() {
            self.set_username(&username);
            self.set_password(&password);
        }
        url = url::Url::parse(strurl.as_str())
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to parse URL: {}", e)))?;

        if url.port().is_none() {
            strurl = format!(
                "{}://{}",
                url.scheme(),
                url.host_str().unwrap_or("app.openiap.io")
            );
        } else {
            strurl = format!(
                "{}://{}:{}",
                url.scheme(),
                url.host_str().unwrap_or("localhost.openiap.io"),
                url.port().unwrap_or(80)
            );
        }
        info!("Connecting to {}", strurl);
        let config = self.load_config(strurl.as_str(), &url).await;
        if !usegprc {
            strurl = format!("{}/ws/v2", strurl);

            let (_stream_tx, stream_rx) = mpsc::channel(60);

            let socket = match tokio_tungstenite::connect_async(strurl.clone()).await {
                Ok((socket, _)) => socket,
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to connect to WS: {}",
                        e
                    )));
                }
            };
            self.set_client(ClientEnum::WS(Arc::new(Mutex::new(socket))));
            self.set_connect_called(true);
            self.set_config(config);
            self.set_url(&strurl);
            match self.setup_ws(&strurl).await {
                Ok(_) => (),
                Err(e) => {
                    return Err(OpenIAPError::ClientError(format!(
                        "Failed to setup WS: {}",
                        e
                    )));
                }
            }
            let client2 = self.clone();
            // tokio::task::Builder::new().name("Old Websocket receiver").spawn(async move {
            tokio::task::spawn(async move {
                tokio_stream::wrappers::ReceiverStream::new(stream_rx)
                    .for_each(|envelope: Envelope| async {
                        let command = envelope.command.clone();
                        let rid = envelope.rid.clone();
                        let id = envelope.id.clone();
                        trace!("Received command: {}, id: {}, rid: {}", command, id, rid);
                        client2.parse_incomming_envelope(envelope).await;
                    })
                    .await;
            }); // .map_err(|e| OpenIAPError::ClientError(format!("Failed to spawn Old Websocket receiver task: {:?}", e)))?;
        } else {
            if url.scheme() == "http" {
                let response = Client::connect_grpc(strurl.clone()).await;
                match response {
                    Ok(client) => {
                        self.set_client(ClientEnum::Grpc(client));
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
                self.set_client(ClientEnum::Grpc(FlowServiceClient::new(channel)));
            }
            self.set_connect_called(true);
            self.set_config(config);
            self.set_url(&strurl);
            self.setup_grpc_stream().await?;
        };
        self.post_connected().await
    }

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
    /// use openiap_client::{OpenIAPError, Client, QueryRequest};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    pub async fn new_connect(dst: &str) -> Result<Self, OpenIAPError> {
        #[cfg(test)]
        {   
            // enable_tracing("openiap=trace", "new");
            // enable_tracing("openiap=debug", "new");
            // enable_tracing("trace", "");
            enable_tracing("openiap=error", "");
            // enable_tracing("openiap=debug", "");
        }
        let client  = Client::new();
        client.connect_async(dst).await?;
        Ok(client)
    }
    /// Handle auto-signin after a connection has been established.
    pub async fn post_connected(&self) -> Result<(), OpenIAPError> {
        if self.get_username().is_empty() && self.get_password().is_empty() {
            self.set_username(&std::env::var("OPENIAP_USERNAME").unwrap_or_default());
            self.set_password(&std::env::var("OPENIAP_PASSWORD").unwrap_or_default());
        }
        if !self.get_username().is_empty() && !self.get_password().is_empty() {
            debug!("Signing in with username: {}", self.get_username());
            let signin = SigninRequest::with_userpass(self.get_username().as_str(), self.get_password().as_str());
            let loginresponse = self.signin(signin).await;
            match loginresponse {
                Ok(response) => {
                    self.reset_reconnect_ms();
                    self.set_connected(ClientState::Connected, None);
                    info!("Signed in as {}", response.user.as_ref().unwrap().username);
                    Ok(())
                }
                Err(_e) => {
                    self.set_connected(ClientState::Disconnected, Some(&_e.to_string()));
                    Err(_e)
                }
            }
        } else {
            self.set_jwt(&std::env::var("OPENIAP_JWT").unwrap_or_default());
            if self.get_jwt().is_empty() {
                self.set_jwt(&std::env::var("jwt").unwrap_or_default());
            }
            if !self.get_jwt().is_empty() {
                debug!("Signing in with JWT");
                let signin = SigninRequest::with_jwt(self.get_jwt().as_str());
                let loginresponse = self.signin(signin).await;
                match loginresponse {
                    Ok(response) => match response.user {
                        Some(user) => {
                            self.reset_reconnect_ms();
                            info!("Signed in as {}", user.username);
                            self.set_connected(ClientState::Connected, None);
                            Ok(())
                        }
                        None => {
                            self.reset_reconnect_ms();
                            info!("Signed in as guest");
                            self.set_connected(ClientState::Connected, None);
                            Ok(())
                            // Err(OpenIAPError::ClientError("Signin returned no user object".to_string()))
                        }
                    },
                    Err(_e) => {
                        self.set_connected(ClientState::Disconnected, Some(&_e.to_string()));
                        Err(_e)
                    }
                }
            } else {
                self.reset_reconnect_ms();
                match self.get_element().await {
                    Ok(_) => {
                        debug!("Connected, No credentials provided so is running as guest");
                        self.set_connected(ClientState::Connected, None);
                        Ok(())
                    },
                    Err(e) => {
                        self.set_connected(ClientState::Disconnected, Some(&e.to_string()));
                        Err(e)
                    }
                }
            }
        }
    }
    /// Reconnect will attempt to reconnect to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    pub async fn reconnect(&self) -> Result<(), OpenIAPError> {
        let state = self.get_state();
        if state == ClientState::Connected || state == ClientState::Signedin {
            return Ok(());
        }
        if !self.is_auto_reconnect() {
            return Ok(());   
        }
        let client = self.get_client();
    
        match client {
            ClientEnum::WS(ref _client) => {
                info!("Reconnecting to {} ({} ms)", self.get_url(), (self.get_reconnect_ms() - 500));
                self.setup_ws(&self.get_url()).await?;
                debug!("Completed reconnecting to websocket");
                self.post_connected().await
            }
            ClientEnum::Grpc(ref _client) => {
                info!("Reconnecting to {} ({} ms)", self.get_url(), (self.get_reconnect_ms() - 500));
                match self.setup_grpc_stream().await {
                    Ok(_) => {
                        debug!("Completed reconnecting to gRPC");
                        self.post_connected().await
                    },
                    Err(e) => {
                        return Err(OpenIAPError::ClientError(format!(
                            "Failed to setup gRPC stream: {}",
                            e
                        )));
                    }
                }
            }
            ClientEnum::None => {
                return Err(OpenIAPError::ClientError("Invalid client".to_string()));
            }
        }
    }
    /// Disconnect the client from the OpenIAP server.
    pub fn disconnect(&self) {
        self.set_auto_reconnect(false);
        self.set_connected(ClientState::Disconnected, Some("Disconnected"));
    }
    /// Set the connected flag to true or false
    pub fn set_connected(&self, state: ClientState, message: Option<&str>) {
        {
            let current = self.get_state();
            trace!("Set connected: {:?} from {:?}", state, current);
            if state == ClientState::Connected && current == ClientState::Signedin {
                self.set_state(ClientState::Signedin);
            } else {
                self.set_state(state.clone());
            }            
            if state == ClientState::Connecting && !current.eq(&state) {
                if let Ok(_handle) = tokio::runtime::Handle::try_current() {
                    self.stats.lock().unwrap().connection_attempts += 1;
                    let me = self.clone();
                    tokio::task::spawn(async move {
                        me.event_sender.send(crate::ClientEvent::Connecting).await.unwrap();
                    });
                    // match tokio::task::Builder::new().name("setconnected-notify-connecting").spawn(async move {
                    //     me.event_sender.send(crate::ClientEvent::Connecting).await.unwrap();
                    // }) {
                    //     Ok(_) => (),
                    //     Err(e) => {
                    //         error!("Failed to spawn setconnected-notify-connecting task: {:?}", e);
                    //     }
                    // }
                }

            }
            if (state == ClientState::Connected|| state == ClientState::Signedin) && (current == ClientState::Disconnected || current == ClientState::Connecting) { 
                if let Ok(_handle) = tokio::runtime::Handle::try_current() {
                    self.stats.lock().unwrap().connections += 1;
                    let me = self.clone();
                    tokio::task::spawn(async move {
                        me.event_sender.send(crate::ClientEvent::Connected).await.unwrap();
                    });
                    // match tokio::task::Builder::new().name("setconnected-notify-connected").spawn(async move {
                    //     me.event_sender.send(crate::ClientEvent::Connected).await.unwrap();
                    // }) {
                    //     Ok(_) => (),
                    //     Err(e) => {
                    //         error!("Failed to spawn setconnected-notify-connected task: {:?}", e);
                    //     }
                    // }
                }
            }
            if state == ClientState::Signedin && current != ClientState::Signedin {
                if let Ok(_handle) = tokio::runtime::Handle::try_current() {
                    let me = self.clone();
                    tokio::task::spawn(async move {
                        me.event_sender.send(crate::ClientEvent::SignedIn).await.unwrap();
                    });
                    // match tokio::task::Builder::new().name("set_signedin-notify-connected").spawn(async move {
                    //     me.event_sender.send(crate::ClientEvent::SignedIn).await.unwrap();
                    // }) {
                    //     Ok(_) => (),
                    //     Err(e) => {
                    //         error!("Failed to spawn set_signedin-notify-connected task: {:?}", e);
                    //     }
                    // };
                }
            }
            if state == ClientState::Disconnected && !current.eq(&state) {
                if message.is_some() {
                    info!("Disconnected: {}", message.unwrap());
                } else {
                    info!("Disconnected");
                }
                if let Ok(_handle) = tokio::runtime::Handle::try_current() {
                    let me = self.clone();
                    let message = match message {
                        Some(message) => message.to_string(),
                        None => "".to_string(),
                    };
                    //if current != ClientState::Connecting {
                        tokio::task::spawn(async move {
                            me.event_sender.send(crate::ClientEvent::Disconnected(message)).await.unwrap();
                        });
                        // match tokio::task::Builder::new().name("setconnected-notify-disconnected").spawn(async move {
                        //     trace!("Disconnected: {}", message);
                        //     me.event_sender.send(crate::ClientEvent::Disconnected(message)).await.unwrap();
                        // }) {
                        //     Ok(_) => (),
                        //     Err(e) => {
                        //         error!("Failed to spawn setconnected-notify-disconnected task: {:?}", e);
                        //     }
                        // }
                    //}
                }

                self.kill_handles();
                if let Ok(_handle) = tokio::runtime::Handle::try_current() {
                    let client = self.clone();
                    // match tokio::task::Builder::new().name("kill_handles").spawn(async move {
                        tokio::task::spawn(async move {
                        {
                            let inner = client.inner.lock().await;
                            let mut queries = inner.queries.lock().await;
                            let ids = queries.keys().cloned().collect::<Vec<String>>();
                            debug!("********************************************** Cleaning up");
                            for id in ids {
                                let err = ErrorResponse {
                                    code: 500,
                                    message: "Disconnected".to_string(),
                                    stack: "".to_string(),
                                };
                                let envelope = err.to_envelope();
                                let tx = queries.remove(&id).unwrap();
                                debug!("kill query: {}", id);
                                let _ = tx.send(envelope);
                            }
                            let mut streams = inner.streams.lock().await;
                            let ids = streams.keys().cloned().collect::<Vec<String>>();
                            for id in ids {
                                let tx = streams.remove(&id).unwrap();
                                debug!("kill stream: {}", id);
                                let _ = tx.send(Vec::new()).await;
                            }
                            let mut queues = inner.queues.lock().await;
                            let ids = queues.keys().cloned().collect::<Vec<String>>();
                            for id in ids {
                                let _ = queues.remove(&id).unwrap();
                            }
                            let mut watches = inner.watches.lock().await;
                            let ids = watches.keys().cloned().collect::<Vec<String>>();
                            for id in ids {
                                let _ = watches.remove(&id).unwrap();
                            }
                            debug!("**********************************************************");
                        }
                        if client.is_auto_reconnect() {
                            trace!("Reconnecting in {} seconds", client.get_reconnect_ms() / 1000);
                            tokio::time::sleep(Duration::from_millis(client.get_reconnect_ms() as u64)).await;
                            if client.is_auto_reconnect() {
                                client.inc_reconnect_ms();
                                // let mut client = client.clone();
                                trace!("Reconnecting . . .");
                                client.reconnect().await.unwrap_or_else(|e| {
                                    error!("Failed to reconnect: {}", e);
                                    client.set_connected(ClientState::Disconnected, Some(&e.to_string()));
                                });
                            } else {
                                debug!("Not reconnecting");
                            }
                        } else {
                            debug!("Reconnecting disabled, stop now");
                        }
                    });
                    //  {
                    //     Ok(_) => (),
                    //     Err(e) => {
                    //         error!("Failed to spawn kill_handles task: {:?}", e);
                    //     }
                    // }
                }
    
            }
        }
    }
    /// Get client state
    pub fn get_state(&self) -> ClientState {
        let conn = self.state.lock().unwrap();
        conn.clone()
    }
    /// Set client state
    pub fn set_state(&self, state: ClientState) {
        let mut conn = self.state.lock().unwrap();
        *conn = state;
    }
    /// Set the msgcount value
    pub fn set_msgcount(&self, msgcount: i32) {
        let mut current = self.msgcount.lock().unwrap();
        trace!("Set msgcount: {} from {}", msgcount, *current);
        *current = msgcount;
    }
    /// Increment the msgcount value
    pub fn inc_msgcount(&self) -> i32 {
        let mut current = self.msgcount.lock().unwrap();
        *current += 1;
        *current
    }
    /// Return value of reconnect_ms
    pub fn get_reconnect_ms(&self) -> i32 {
        let reconnect_ms = self.reconnect_ms.lock().unwrap();
        *reconnect_ms
    }
    /// Increment the reconnect_ms value
    pub fn reset_reconnect_ms(&self) {
        let mut current = self.reconnect_ms.lock().unwrap();
        *current = 500;
    }
    /// Increment the reconnect_ms value
    pub fn inc_reconnect_ms(&self) -> i32 {
        let mut current = self.reconnect_ms.lock().unwrap();
        if *current < 30000 {
            *current += 500;
        }
        *current
    }
    
    /// Push tokio task handle to the task_handles vector
    pub fn push_handle(&self, handle: tokio::task::JoinHandle<()>) {
        let mut handles = self.task_handles.lock().unwrap();
        handles.push(handle);
    }
    /// Kill all tokio task handles in the task_handles vector
    pub fn kill_handles(&self) {
        let mut handles = self.task_handles.lock().unwrap();
        for handle in handles.iter() {
            // let id = handle.id();
            // debug!("Killing handle {}", id);
            debug!("Killing handle");
            if !handle.is_finished() {
                handle.abort();
            }
        }
        handles.clear();
        // if let Ok(_handle) = tokio::runtime::Handle::try_current() {
        //     let runtime = self.get_runtime();
        //     if let Some(rt) = runtime.lock().unwrap().take() {
        //         rt.shutdown_background();
        //     }
        // }
    }


    /// Return value of the msgcount flag
    #[tracing::instrument(skip_all)]
    fn get_msgcount(&self) -> i32 {
        let msgcount = self.msgcount.lock().unwrap();
        *msgcount
    }

    /// Set the connect_called flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_connect_called(&self, connect_called: bool) {
        let mut current = self.connect_called.lock().unwrap();
        trace!("Set connect_called: {} from {}", connect_called, *current);
        *current = connect_called;
    }
    /// Return value of the connect_called flag
    #[tracing::instrument(skip_all)]
    fn is_connect_called(&self) -> bool {
        let connect_called = self.connect_called.lock().unwrap();
        *connect_called
    }
    /// Set the auto_reconnect flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_auto_reconnect(&self, auto_reconnect: bool) {
        let mut current = self.auto_reconnect.lock().unwrap();
        trace!("Set auto_reconnect: {} from {}", auto_reconnect, *current);
        *current = auto_reconnect;
    }
    /// Return value of the auto_reconnect flag
    #[tracing::instrument(skip_all)]
    fn is_auto_reconnect(&self) -> bool {
        let auto_reconnect = self.auto_reconnect.lock().unwrap();
        *auto_reconnect
    }
    /// Set the url flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_url(&self, url: &str) {
        let mut current = self.url.lock().unwrap();
        trace!("Set url: {} from {}", url, *current);
        *current = url.to_string();
    }
    /// Return value of the url string
    #[tracing::instrument(skip_all)]
    fn get_url(&self) -> String {
        let url = self.url.lock().unwrap();
        url.to_string()
    }
    /// Set the username flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_username(&self, username: &str) {
        let mut current = self.username.lock().unwrap();
        trace!("Set username: {} from {}", username, *current);
        *current = username.to_string();
    }
    /// Return value of the username string
    #[tracing::instrument(skip_all)]
    fn get_username(&self) -> String {
        let username = self.username.lock().unwrap();
        username.to_string()
    }
    /// Set the password value
    #[tracing::instrument(skip_all)]
    pub fn set_password(&self, password: &str) {
        let mut current = self.password.lock().unwrap();
        trace!("Set password: {} from {}", password, *current);
        *current = password.to_string();
    }
    /// Return value of the password string
    #[tracing::instrument(skip_all)]
    fn get_password(&self) -> String {
        let password = self.password.lock().unwrap();
        password.to_string()
    }
    /// Set the jwt flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_jwt(&self, jwt: &str) {
        let mut current = self.jwt.lock().unwrap();
        trace!("Set jwt: {} from {}", jwt, *current);
        *current = jwt.to_string();
    }
    /// Return value of the jwt string
    #[tracing::instrument(skip_all)]
    fn get_jwt(&self) -> String {
        let jwt = self.jwt.lock().unwrap();
        jwt.to_string()
    }
    /// Set the agent flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_agent_name(&self, agent: &str) {
        let mut current = self.agent_name.lock().unwrap();
        trace!("Set agent: {} from {}", agent, *current);
        *current = agent.to_string();
    }
    /// Return value of the agent string
    #[tracing::instrument(skip_all)]
    pub fn get_agent_name(&self) -> String {
        let agent = self.agent_name.lock().unwrap();
        agent.to_string()
    }
    /// Set the agent version number
    #[tracing::instrument(skip_all)]
    pub fn set_agent_version(&self, version: &str) {
        let mut current = self.agent_version.lock().unwrap();
        trace!("Set agent version: {} from {}", version, *current);
        *current = version.to_string();
    }
    /// Return value of the agent version string
    #[tracing::instrument(skip_all)]
    pub fn get_agent_version(&self) -> String {
        let agent_version = self.agent_version.lock().unwrap();
        agent_version.to_string()
    }
    
    /// Set the config flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_config(&self, config: Option<Config>) {
        let mut current = self.config.lock().unwrap();
        *current = config;
    }
    /// Return value of the config 
    #[tracing::instrument(skip_all)]
    pub fn get_config(&self) -> Option<Config> {
        let config = self.config.lock().unwrap();
        config.clone()
    }
    /// Set the client flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_client(&self, client: ClientEnum) {
        let mut current = self.client.lock().unwrap();
        *current = client;
    }
    /// Return value of the client
    #[tracing::instrument(skip_all)]
    fn get_client(&self) -> ClientEnum {
        let client = self.client.lock().unwrap();
        client.clone()
    }
    /// Set the user flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_user(&self, user: Option<User>) {
        let mut current = self.user.lock().unwrap();
        *current = user;
    }
    /// Return value of the user
    #[tracing::instrument(skip_all)]
    pub fn get_user(&self) -> Option<User> {
        let user = self.user.lock().unwrap();
        user.clone()
    }
    // /// Return the signed in user, if we are signed in.
    // #[tracing::instrument(skip_all)]
    // pub fn get_user(&self) -> Option<User> {
    //     // let inner = self.inner.lock().await;
    //     // inner.user.clone()
    //     self.user.clone()
    // }
    
    /// Set the runtime flag to true or false
    #[tracing::instrument(skip_all)]
    pub fn set_runtime(&self, runtime: Option<tokio::runtime::Runtime>) {
        let mut current = self.runtime.lock().unwrap();
        *current = runtime;
    }
    /// Return value of the runtime
    #[tracing::instrument(skip_all)]
    // pub fn get_runtime(&self) -> Option<Arc<tokio::runtime::Runtime>> {
    pub fn get_runtime(&self) -> &std::sync::Mutex<std::option::Option<tokio::runtime::Runtime>> {
        self.runtime.as_ref()
    }
    /// Return value of the runtime handle
    #[tracing::instrument(skip_all)]
    pub fn get_runtime_handle(&self) -> tokio::runtime::Handle {
        let mut rt = self.runtime.lock().unwrap();
        if rt.is_none() {
            // println!("Rust: Initializing new Tokio runtime");
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            *rt = Some(runtime);
        } else {
            // println!("Rust: Runtime already initialized");
        }
        rt.as_ref().unwrap().handle().clone()
    }
    /// Method to allow the user to subscribe with a callback function
    #[tracing::instrument(skip_all)]
    pub async fn on_event(&self, callback: Box<dyn Fn(ClientEvent) + Send + Sync>)
    {
        // call the callback function every time there is an event in the client.event_receiver
        let event_receiver = self.event_receiver.clone();
        let callback = callback;
        let _handle =  tokio::task::spawn(async move {
            while let Ok(event) = event_receiver.recv().await {
                callback(event);
            }
        }); // .unwrap();
    }
    /// Internal function, used to generate a unique id for each message sent to the server.
    #[tracing::instrument(skip_all)]
    pub fn get_uniqueid() -> String {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        let num1 = COUNTER.fetch_add(1, Ordering::Relaxed) as u64;
        let num2 = COUNTER.fetch_add(1, Ordering::Relaxed) as u64;
        let num3 = COUNTER.fetch_add(1, Ordering::Relaxed) as u64;
        let sqids = Sqids::default();
        sqids.encode(&[num1, num2, num3 ]).unwrap().to_string()
    }
    /// Internal function, Send a message to the OpenIAP server, and wait for a response.
    #[tracing::instrument(skip_all)]
    async fn send(&self, msg: Envelope) -> Result<Envelope, OpenIAPError> {
        let response = self.send_noawait(msg).await;
        match response {
            Ok((response_rx, id)) => {
                // Await the response
                let response = response_rx.await;
    
                // Remove the entry from `inner.queries` after awaiting
                let inner = self.inner.lock().await;
                inner.queries.lock().await.remove(&id);
    
                // Handle the result of the await
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
        let (response_tx, response_rx) = oneshot::channel();
        let id = Client::get_uniqueid();
        msg.id = id.clone();
    
        // Lock and insert the sender into `inner.queries`
        {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.insert(id.clone(), response_tx);
        }
    
        // Send the message and check for errors
        let res = self.send_envelope(msg).await;
        if let Err(e) = res {
            // Remove the entry from `inner.queries` if the send fails
            let inner = self.inner.lock().await;
            inner.queries.lock().await.remove(&id);
            return Err(OpenIAPError::ClientError(e.to_string()));
        }
    
        Ok((response_rx, id))
    }
    /// Internal function, Setup a new stream, send a message to the OpenIAP server, and return a stream to send and receive data.
    #[tracing::instrument(skip_all)]
    async fn sendwithstream(
        &self,
        mut msg: Envelope,
    ) -> Result<(oneshot::Receiver<Envelope>, mpsc::Receiver<Vec<u8>>), OpenIAPError> {
        let (response_tx, response_rx) = oneshot::channel();
        let (stream_tx, stream_rx) = mpsc::channel(1024 * 1024);
        let id = Client::get_uniqueid();
        msg.id = id.clone();
        {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.insert(id.clone(), response_tx);
            inner.streams.lock().await.insert(id.clone(), stream_tx);
            let res = self.send_envelope(msg).await;
            match res {
                Ok(_) => (),
                Err(e) => return Err(OpenIAPError::ClientError(e.to_string())),
            }
        }
        Ok((response_rx, stream_rx))
    }
    #[tracing::instrument(skip_all, target = "openiap::client")]
    async fn send_envelope(&self, mut envelope: Envelope) -> Result<(), OpenIAPError> {
        if (self.get_state() != ClientState::Connected && self.get_state() != ClientState::Signedin ) 
            && envelope.command != "signin" && envelope.command != "getelement" && envelope.command != "pong" {
            return Err(OpenIAPError::ClientError(format!("Not connected ( {:?} )", self.get_state())));
        }
        let env = envelope.clone();
        let command = envelope.command.clone();
        self.stats.lock().unwrap().package_tx += 1;
        match command.as_str() {
            "signin" => { self.stats.lock().unwrap().signin += 1;},
            "upload" => { self.stats.lock().unwrap().upload += 1;},
            "download" => { self.stats.lock().unwrap().download += 1;},
            "getdocumentversion" => { self.stats.lock().unwrap().getdocumentversion += 1;},
            "customcommand" => { self.stats.lock().unwrap().customcommand += 1;},
            "listcollections" => { self.stats.lock().unwrap().listcollections += 1;},
            "createcollection" => { self.stats.lock().unwrap().createcollection += 1;},
            "dropcollection" => { self.stats.lock().unwrap().dropcollection += 1;},
            "ensurecustomer" => { self.stats.lock().unwrap().ensurecustomer += 1;},
            "invokeopenrpa" => { self.stats.lock().unwrap().invokeopenrpa += 1;},

            "registerqueue" => { self.stats.lock().unwrap().registerqueue += 1;},
            "registerexchange" => { self.stats.lock().unwrap().registerexchange += 1;},
            "unregisterqueue" => { self.stats.lock().unwrap().unregisterqueue += 1;},
            "watch" => { self.stats.lock().unwrap().watch += 1;},
            "unwatch" => { self.stats.lock().unwrap().unwatch += 1;},
            "queuemessage" => { self.stats.lock().unwrap().queuemessage += 1;},

            "pushworkitem" => { self.stats.lock().unwrap().pushworkitem += 1;},
            "pushworkitems" => { self.stats.lock().unwrap().pushworkitems += 1;},
            "popworkitem" => { self.stats.lock().unwrap().popworkitem += 1;},
            "updateworkitem" => { self.stats.lock().unwrap().updateworkitem += 1;},
            "deleteworkitem" => { self.stats.lock().unwrap().deleteworkitem += 1;},
            "addworkitemqueue" => { self.stats.lock().unwrap().addworkitemqueue += 1;},
            "updateworkitemqueue" => { self.stats.lock().unwrap().updateworkitemqueue += 1;},
            "deleteworkitemqueue" => { self.stats.lock().unwrap().deleteworkitemqueue += 1;},

            "getindexes" => { self.stats.lock().unwrap().getindexes += 1;},
            "createindex" => { self.stats.lock().unwrap().createindex += 1;},
            "dropindex" => { self.stats.lock().unwrap().dropindex += 1;},
            "query" => { self.stats.lock().unwrap().query += 1;},
            "count" => { self.stats.lock().unwrap().count += 1;},
            "distinct" => { self.stats.lock().unwrap().distinct += 1;},
            "aggregate" => { self.stats.lock().unwrap().aggregate += 1;},
            "insertone" => { self.stats.lock().unwrap().insertone += 1;},
            "insertmany" => { self.stats.lock().unwrap().insertmany += 1;},
            "updateone" => { self.stats.lock().unwrap().updateone += 1;},
            "insertorupdateone" => { self.stats.lock().unwrap().insertorupdateone += 1;},
            "insertorupdatemany" => { self.stats.lock().unwrap().insertorupdatemany += 1;},
            "updatedocument" => { self.stats.lock().unwrap().updatedocument += 1;},
            "deleteone" => { self.stats.lock().unwrap().deleteone += 1;},
            "deletemany" => { self.stats.lock().unwrap().deletemany += 1;},
            _ => {}
        };
        if envelope.id.is_empty() {
            let id = Client::get_uniqueid();
            envelope.id = id.clone();
        }
        trace!("Sending {} message, in the thread", command);
        let res = self.out_envelope_sender.send(env).await;
        if res.is_err() {
            error!("{:?}", res);
            let errmsg = res.unwrap_err().to_string();
            self.set_connected(ClientState::Disconnected, Some(&errmsg));
            return Err(OpenIAPError::ClientError(format!("Failed to send data: {}", errmsg)))
        } else {
            return Ok(())
        }
    }
    #[tracing::instrument(skip_all, target = "openiap::client")]
    async fn parse_incomming_envelope(&self, received: Envelope) {
        self.stats.lock().unwrap().package_rx += 1;
        let command = received.command.clone();
        trace!("parse_incomming_envelope, command: {}", command);
        let inner = self.inner.lock().await;
        let rid = received.rid.clone();
        let mut queries = inner.queries.lock().await;
        let mut streams = inner.streams.lock().await;
        let watches = inner.watches.lock().await;
        let queues = inner.queues.lock().await;
    
        if command != "ping" && command != "pong" && command != "refreshtoken" {
            if rid.is_empty() {
                debug!("Received #{} #{} {} message", received.seq, received.id, command);
            } else {
                debug!("Received #{} #{} (reply to #{}) {} message", received.seq, received.id, rid, command);
            }
        } else if rid.is_empty() {
            trace!("Received #{} #{} {} message", received.seq, received.id, command);
        } else {
            trace!("Received #{} #{} (reply to #{}) {} message", received.seq, received.id, rid, command);
        }
        
        if command == "ping" {
            self.pong(&received.id).await;
            // self.event_sender.send(crate::ClientEvent::Ping).await.unwrap();
        } else if command == "refreshtoken" {
            // TODO: Do we store the new jwt at some point in the future
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
    /// Internal function, used to send a fake getelement to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    async fn get_element(&self) -> Result<(), OpenIAPError> {
        let id = Client::get_uniqueid();
        let envelope = Envelope {
            id: id.clone(),
            command: "getelement".into(),
            ..Default::default()
        };
        let result = match self.send(envelope).await {
            Ok(res) => res,
            Err(e) => {
                return Err(e);
            },
        };
        if result.command == "pong" || result.command == "getelement" {
            Ok(())
        } else if result.command == "error" {
            let e: ErrorResponse = prost::Message::decode(result.data.unwrap().value.as_ref()).unwrap();
            Err(OpenIAPError::ServerError(e.message))
        } else {
            Err(OpenIAPError::ClientError("Failed to receive getelement".to_string()))
        }
    }
    /// Internal function, used to send a ping to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    async fn ping(&self) -> Result<(), OpenIAPError> {
        let id = Client::get_uniqueid();
        let envelope = Envelope {
            id: id.clone(),
            command: "getelement".into(),
            ..Default::default()
        };
        match self.send_envelope(envelope).await {
            Ok(_res) => Ok(()),
            Err(e) => {
                return Err(e);
            },
        }
    }
    /// Internal function, used to send a pong response to the OpenIAP server.
    #[tracing::instrument(skip_all)]
    async fn pong(&self, rid: &str) {
        let id = Client::get_uniqueid();
        let envelope = Envelope {
            id: id.clone(),
            command: "pong".into(),
            rid: rid.to_string(),
            ..Default::default()
        };
        match self.send_envelope(envelope).await {
            Ok(_) => (),
            Err(e) => error!("Failed to send pong: {}", e),
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
            config.agent = self.get_agent_name();
        }

        debug!("Attempting sign-in using {:?}", config);
        let envelope = config.to_envelope();
        let result = self.send(envelope).await;

        match &result {
            Ok(m) => {
                debug!("Sign-in reply received");
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
                    self.set_connected(ClientState::Signedin, None);
                    self.set_user(Some(response.user.as_ref().unwrap().clone()));
                }
                Ok(response)
            }
            Err(e) => {
                debug!("Sending Sign-in request failed {:?}", result);
                debug!("Sign-in failed: {}", e.to_string());
                if !config.validateonly {
                    self.set_user(None);
                }
                Err(OpenIAPError::ClientError(e.to_string()))
            }
        }
    }
    /// Return a list of collections in the database
    /// - includehist: include historical collections, default is false.
    /// please see create_collection for examples on how to create collections.
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
    /// use openiap_client::{Client, CreateCollectionRequest, DropCollectionRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
    ///     //let collections = client.list_collections(false).await?;
    ///     //println!("Collections: {}", collections);
    ///     let config = CreateCollectionRequest::byname("rusttestcollection");
    ///     client.create_collection(config).await?;
    ///     let config = DropCollectionRequest::byname("rusttestcollection");
    ///     client.drop_collection(config).await?;
    ///     Ok(())
    /// }
    /// ```
    /// You can create a normal collection with a TTL index on the _created field, using the following example:
    /// ```
    /// use openiap_client::{Client, CreateCollectionRequest, DropCollectionRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{Client, CreateCollectionRequest, DropCollectionRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{Client, GetIndexesRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{Client, DropIndexRequest, CreateIndexRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{Client, DropIndexRequest, CreateIndexRequest, OpenIAPError};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{OpenIAPError, Client, QueryRequest};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{OpenIAPError, Client, QueryRequest};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{OpenIAPError, Client, QueryRequest};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
                if items.is_empty() {
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
    /// use openiap_client::{OpenIAPError, Client, GetDocumentVersionRequest, InsertOneRequest, UpdateOneRequest};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///     let client = Client::new_connect("").await?;
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
    /// use openiap_client::{OpenIAPError, Client, AggregateRequest};
    /// #[tokio::main]
    /// async fn main() -> Result<(), OpenIAPError> {
    ///    let client = Client::new_connect("").await?;
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
                let temp_file_path = util::generate_unique_filename("openiap");
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
                util::move_file(temp_file_path.to_str().unwrap(), filepath.as_str()).map_err(|e| {
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
        // debug!("upload: Uploading file: {}", filepath);
        // let mut file = File::open(filepath)
        //     .map_err(|e| OpenIAPError::ClientError(format!("Failed to open file: {}", e)))?;
        // let chunk_size = 1024 * 1024;
        // let mut buffer = vec![0; chunk_size];

        // let envelope = config.to_envelope();
        // let (response_rx, rid) = self.send_noawait(envelope).await?;
        // {
        //     let envelope = BeginStream::from_rid(rid.clone());
        //     debug!("Sending beginstream to #{}", rid);
        //     self.send_envelope(envelope).await.map_err(|e| OpenIAPError::ClientError(format!("Failed to send data: {}", e)))?;
        //     let mut counter = 0;

        //     loop {
        //         let bytes_read = file.read(&mut buffer).map_err(|e| {
        //             OpenIAPError::ClientError(format!("Failed to read from file: {}", e))
        //         })?;
        //         counter += 1;

        //         if bytes_read == 0 {
        //             break;
        //         }

        //         let chunk = buffer[..bytes_read].to_vec();
        //         let envelope = Stream::from_rid(chunk, rid.clone());
        //         debug!("Sending chunk {} stream to #{}", counter, envelope.rid);
        //         self.send_envelope(envelope).await.map_err(|e| {
        //             OpenIAPError::ClientError(format!("Failed to send data: {}", e))
        //         })?
        //     }

        //     let envelope = EndStream::from_rid(rid.clone());
        //     debug!("Sending endstream to #{}", rid);
        //     self.send_envelope(envelope).await
        //         .map_err(|e| OpenIAPError::ClientError(format!("Failed to send data: {}", e)))?;
        // }

        // debug!("Wait for upload response for #{}", rid);
        // match response_rx.await {
        //     Ok(response) => {
        //         if response.command == "error" {
        //             let error_response: ErrorResponse = prost::Message::decode(
        //                 response.data.unwrap().value.as_ref(),
        //             )
        //             .map_err(|e| {
        //                 OpenIAPError::ClientError(format!("Failed to decode ErrorResponse: {}", e))
        //             })?;
        //             return Err(OpenIAPError::ServerError(error_response.message));
        //         }
        //         let upload_response: UploadResponse =
        //             prost::Message::decode(response.data.unwrap().value.as_ref()).map_err(|e| {
        //                 OpenIAPError::ClientError(format!("Failed to decode UploadResponse: {}", e))
        //             })?;
        //         Ok(upload_response)
        //     }
        //     Err(e) => Err(OpenIAPError::CustomError(e.to_string())),
        // }
        debug!("upload: Uploading file: {}", filepath);
        let mut file = File::open(filepath)
            .map_err(|e| OpenIAPError::ClientError(format!("Failed to open file: {}", e)))?;
        let chunk_size = 1024 * 1024;
        let mut buffer = vec![0; chunk_size];
    
        // Send the initial upload request
        let envelope = config.to_envelope();
        let (response_rx, rid) = self.send_noawait(envelope).await?;
        
        // Send the BeginStream message
        let envelope = BeginStream::from_rid(rid.clone());
        debug!("Sending beginstream to #{}", rid);
        if let Err(e) = self.send_envelope(envelope).await {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.remove(&rid);
            return Err(OpenIAPError::ClientError(format!("Failed to send data: {}", e)));
        }
    
        // Send file chunks
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
            if let Err(e) = self.send_envelope(envelope).await {
                let inner = self.inner.lock().await;
                inner.queries.lock().await.remove(&rid);
                return Err(OpenIAPError::ClientError(format!("Failed to send data: {}", e)));
            }
        }
    
        // Send the EndStream message
        let envelope = EndStream::from_rid(rid.clone());
        debug!("Sending endstream to #{}", rid);
        if let Err(e) = self.send_envelope(envelope).await {
            let inner = self.inner.lock().await;
            inner.queries.lock().await.remove(&rid);
            return Err(OpenIAPError::ClientError(format!("Failed to send data: {}", e)));
        }
    
        // Await the response and clean up `inner.queries` afterward
        debug!("Wait for upload response for #{}", rid);
        let result = response_rx.await;
        let inner = self.inner.lock().await;
        inner.queries.lock().await.remove(&rid);
    
        match result {
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
                        f.file = util::compress_file_to_vec(&f.filename).unwrap();
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
                            f.file = util::compress_file_to_vec(&f.filename).unwrap();
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
                    debug!("Received event: {:?}", event);
                    if command.eq("invokesuccess") {
                        debug!("Robot successfully started running workflow");
                    } else if command.eq("invokeidle") {
                        debug!("Workflow went idle");
                    } else if command.eq("invokeerror") {
                        debug!("Robot failed to run workflow");
                        let tx = tx.lock().unwrap().take().unwrap();
                        tx.send(event.data).unwrap();
                    } else if command.eq("timeout") {
                        debug!("No robot picked up the workflow");
                        let tx = tx.lock().unwrap().take().unwrap();
                        tx.send(event.data).unwrap();
                    } else if command.eq("invokecompleted") {
                        debug!("Robot completed running workflow");
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
        debug!("Registered Response Queue: {:?}", q);
        let data = format!(
            "{{\"command\":\"invoke\",\"workflowid\":\"{}\",\"payload\": {}}}",
            config.workflowid, config.payload
        );
        debug!("Send Data: {}", data);
        debug!("To Queue: {} With reply to: {}", config.robotid, q);
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
                debug!("Received json result: {:?}", json);
                let obj = serde_json::from_str::<serde_json::Value>(&json).unwrap();
                let command: String = obj["command"].as_str().unwrap().to_string();
                let mut data = "".to_string();
                if obj["data"].as_str().is_some() {
                    data = obj["data"].as_str().unwrap().to_string();
                } else if obj["data"].as_object().is_some() {
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

