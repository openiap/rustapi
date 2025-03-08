use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_unique_filename(base: &str) -> PathBuf {
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
pub fn move_file(from: &str, to: &str) -> std::io::Result<()> {
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
use opentelemetry::{Key, KeyValue};
use std::io::{self, BufReader, BufWriter};
#[allow(dead_code)]
pub fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
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
























use once_cell::sync::{Lazy, OnceCell};
use std::sync::Mutex;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::{Layer, SubscriberExt},
    reload,
    filter::Filtered,
    EnvFilter,
    Registry,
};
use tracing_subscriber::layer::Layered;
use tracing::span::{Attributes, Record};

#[cfg(feature = "otel")]
use {
    opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge,
    opentelemetry_otlp::{LogExporter, WithTonicConfig, WithExportConfig}, // SpanExporter
    opentelemetry_sdk::{
        logs::{SdkLoggerProvider, SdkLogger},
        Resource,
    },
    tracing::Subscriber,
};

static INSTALLED: OnceCell<()> = OnceCell::new();
const HOSTNAME: Key = Key::from_static_str("hostname");
const OFID: Key = Key::from_static_str("ofid");

type ConsoleLayered = Layered<
    Filtered<fmt::Layer<Registry>, reload::Layer<EnvFilter, Registry>, Registry>,
    Registry
>;

static CONSOLE_FILTER_HANDLE: Lazy<OnceCell<reload::Handle<EnvFilter, Registry>>> =
    Lazy::new(OnceCell::new);

#[cfg(feature="otel")]
static OTEL_BRIDGE_HANDLE: Lazy<OnceCell<reload::Handle<OtelBridgeState, ConsoleLayered>>> =
    Lazy::new(OnceCell::new);

#[cfg(feature="otel")]
#[derive(Clone)]
struct PendingOtelConfig {
    log_url: String,
    filter: String,
    ofid: String,
    version: String, 
    agent_name: String, 
    agent_version: String
}

#[cfg(feature="otel")]
static PENDING_OTEL_CONFIG: Lazy<Mutex<Option<PendingOtelConfig>>> =
    Lazy::new(|| Mutex::new(None));

#[derive(Clone)]
struct LastUsedFilters {
    console_filter: String,
    otel_filter: String,
    span_events: String,
}

/// Default to "openiap=info" for console and "openiap=trace" for OTEL, and "none" for span_events
static LAST_USED_FILTERS: Lazy<Mutex<LastUsedFilters>> = Lazy::new(|| Mutex::new(
    LastUsedFilters {
        console_filter: "openiap=info".to_string(),
        otel_filter:    "openiap=trace".to_string(),
        span_events:    "".to_string(),
    }
));
/// Override the default logging filter for opentelemetry
#[allow(dead_code)]
pub fn set_otel_log_filter( otel_filter: &str) {
    {
        let mut last = LAST_USED_FILTERS.lock().unwrap();
        last.otel_filter = otel_filter.to_owned();
    }

    #[cfg(feature="otel")]
    {
        // We want to update the bridging filter. But we do it outside bridging creation:
        if let Some(handle) = OTEL_BRIDGE_HANDLE.get() {
            // short lock to change the filter
            if let Err(e) = handle.modify(|state| {
                // parse new filter
                let parsed = match EnvFilter::try_new(otel_filter) {
                    Ok(f) => f,
                    Err(err) => {
                        eprintln!("Failed to parse otel filter {otel_filter:?}: {err}");
                        return;
                    }
                };
                state.filter = parsed;
            }) {
                eprintln!("Failed to update bridging filter: {e}");
            }
        }
    }
    #[cfg(feature="otel")]
    {
        // If there's a pending OTel config (for instance if user called set_otel_url first),
        // we now apply it. This creates bridging outside the lock, then sets it.
        apply_pending_otel_config();
    }
}
/// Public function: call as many times as you want
/// Added `span_events: &str` to control how span events are logged
pub fn enable_tracing(console_filter: &str, span_events: &str) {
    {
        // Store these filters as our "last known" config
        let mut last = LAST_USED_FILTERS.lock().unwrap();
        last.console_filter = console_filter.to_owned();
        last.span_events    = span_events.to_owned();
    }

    // If not yet installed, do a one-time global subscriber install:
    if INSTALLED.set(()).is_ok() {
        install_global_subscriber();
    }

    // update console filter
    update_console_filter(console_filter);

    #[cfg(feature="otel")]
    {
        // If there's a pending OTel config (for instance if user called set_otel_url first),
        // we now apply it. This creates bridging outside the lock, then sets it.
        apply_pending_otel_config();
    }
}

/// Disable tracing by setting a "none" filter and no span events
pub fn disable_tracing() {
    update_console_filter("none");

    // Also set the "span_events" to none in our stored config
    {
        let mut last = LAST_USED_FILTERS.lock().unwrap();
        last.span_events = "none".to_string();
    }

    #[cfg(feature="otel")]
    {
        // bridging = none
        update_otel_state("", "none", "", "", "", "");
    }
}

/// Dynamically update the console filter
pub fn update_console_filter(new_filter: &str) {
    if let Some(handle) = CONSOLE_FILTER_HANDLE.get() {
        let parsed = match EnvFilter::try_new(new_filter) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Failed to parse console filter {new_filter:?}: {e}");
                return;
            }
        };
        if let Err(err) = handle.modify(|f| *f = parsed) {
            eprintln!("Failed to update console filter: {err}");
        }
    }
}

/// Set the OTel endpoint URL for bridging
#[cfg(feature="otel")]
pub fn set_otel_url(log_url: &str, trace_url: &str, ofid: &str, version: &str, agent_name: &str, agent_version: &str) {
    let log_url = log_url.trim();
    let _trace_url = trace_url.trim();
    let ofid = ofid.trim();

    // Store the configuration for later application
    {
        let mut pending = PENDING_OTEL_CONFIG.lock().unwrap();
        *pending = Some(PendingOtelConfig {
            log_url: log_url.to_string(),
            filter: LAST_USED_FILTERS.lock().unwrap().otel_filter.clone(),
            ofid: ofid.to_string(),
            version: version.to_string(),
            agent_name: agent_name.to_string(),
            agent_version: agent_version.to_string(),
        });
    }

    // If the subscriber wasn't installed yet, install it with defaults
    if INSTALLED.set(()).is_ok() {
        install_global_subscriber(); 
    }

    // Schedule the OTel configuration to be applied on the next event loop iteration
    tokio::spawn(async {
        // Small delay to ensure we're outside any tracing context
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        apply_pending_otel_config();
    });
}

#[cfg(feature="otel")]
fn apply_pending_otel_config() {
    // Take the config outside the lock
    let maybe_config = PENDING_OTEL_CONFIG.lock()
        .unwrap()
        .take();

    if let Some(PendingOtelConfig { log_url, filter, ofid, version, agent_name, agent_version }) = maybe_config {
        // Ensure we're not in a tracing context
        tracing::dispatcher::with_default(
            &tracing::dispatcher::Dispatch::new(tracing_subscriber::Registry::default()),
            || {
                let new_state = build_otel_state(&log_url, &filter, &ofid, &version, &agent_name, &agent_version);
                if let Some(handle) = OTEL_BRIDGE_HANDLE.get() {
                    if let Err(err) = handle.modify(|state| {
                        *state = new_state;
                    }) {
                        eprintln!("Failed to set bridging: {err}");
                    }
                }
            }
        );
    }
}

// INTERNAL: One-time global subscriber
fn install_global_subscriber() {
    let registry = Registry::default();

    // reloadable console filter. Default is "openiap=info"
    let console_filter = EnvFilter::new("openiap=info");
    let (console_filter_layer, console_reload_handle) = reload::Layer::new(console_filter);

    // Get our last-known span_events setting
    let span_events = {
        let last = LAST_USED_FILTERS.lock().unwrap();
        parse_span_events(&last.span_events)
    };

    // Attach the console fmt layer with the chosen span-event setting
    let console_layer = fmt::layer()
        .with_thread_names(true)
        .with_span_events(span_events)
        .with_filter(console_filter_layer);

    let console_subscriber = registry.with(console_layer);

    #[cfg(feature="otel")]
    let (otel_layer, otel_reload_handle) = reload::Layer::new(OtelBridgeState::none());

    #[cfg(feature="otel")]
    let full_subscriber = console_subscriber.with(otel_layer);

    #[cfg(not(feature="otel"))]
    let full_subscriber = console_subscriber;

    tracing::subscriber::set_global_default(full_subscriber)
        .expect("Failed to set global subscriber");

    CONSOLE_FILTER_HANDLE
        .set(console_reload_handle)
        .expect("Failed to store console reload handle");

    #[cfg(feature="otel")]
    {
        if let Err(_e) = OTEL_BRIDGE_HANDLE.set(otel_reload_handle) {
            panic!("Failed to store OTel reload handle");
        }
    }
}

// Helper: parse a string into a `FmtSpan` mode
fn parse_span_events(input: &str) -> FmtSpan {
    match input.to_lowercase().as_str() {
        "new"    => FmtSpan::NEW,
        "enter"  => FmtSpan::ENTER,
        "exit"   => FmtSpan::EXIT,
        "close"  => FmtSpan::CLOSE,
        "active" => FmtSpan::ACTIVE,
        "full"   => FmtSpan::FULL,
        // fallback to none on anything unknown or "none"
        _        => FmtSpan::NONE,
    }
}

// OTel bridging
#[cfg(feature="otel")]
fn update_otel_state(log_url: &str, filter: &str, ofid: &str, version: &str, agent_name: &str, agent_version: &str) {
    let new_state = if log_url.is_empty() {
        OtelBridgeState::none()
    } else {
        build_otel_state(log_url, filter, ofid, version, agent_name, agent_version)
    };

    if let Some(handle) = OTEL_BRIDGE_HANDLE.get() {
        if let Err(err) = handle.modify(|state| {
            *state = new_state;
        }) {
            eprintln!("Failed to update OTel bridging state: {err}");
        }
    }
}

/// Build bridging outside of any reload lock, so we don't re-enter logs on ourselves
#[cfg(feature="otel")]
fn build_otel_state(endpoint: &str, filter_directives: &str, ofid: &str, version: &str, agent_name: &str, agent_version: &str) -> OtelBridgeState {
    if endpoint.is_empty() {
        return OtelBridgeState::none();
    }

    tracing::dispatcher::with_default(
        &tracing::dispatcher::Dispatch::new(tracing_subscriber::Registry::default()),
        || {
            OtelBridgeState::some(endpoint, filter_directives, ofid, version, agent_name, agent_version)
        },
    )
}

#[cfg(feature="otel")]
struct OtelBridgeState {
    bridging: Option<OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger>>,
    filter: EnvFilter,
}

#[cfg(feature="otel")]
impl OtelBridgeState {
    fn none() -> Self {
        Self {
            bridging: None,
            filter: EnvFilter::new("none"),
        }
    }

    fn some(endpoint: &str, filter_directives: &str, ofid: &str, version: &str, agent_name: &str, agent_version: &str) -> Self {
        let exporter = LogExporter::builder()
            .with_tonic()
            .with_tls_config(
                tonic::transport::ClientTlsConfig::new().with_native_roots()
            )
            .with_endpoint(endpoint)
            .build()
            .expect("Failed to build OTel exporter");
        let common_attributes = [
            KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
            KeyValue::new(OFID, ofid.to_string()),
            KeyValue::new("PID", std::process::id().to_string()),
        ];
    
        let resource = Resource::builder().with_service_name("rust")
        .with_attribute(KeyValue::new("service.version", version.to_string() ))
        .with_attribute(KeyValue::new("agent.name", agent_name.to_string() ))
        .with_attribute(KeyValue::new("agent.version", agent_version.to_string() ))
        .with_attributes(common_attributes)
        .build();

        let provider = SdkLoggerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(resource)
            .build();

        let bridging = Some(OpenTelemetryTracingBridge::new(&provider));

        let filter = EnvFilter::try_new(filter_directives)
            .unwrap_or_else(|_| EnvFilter::new("openiap=trace"));

        Self { bridging, filter }
    }
}

#[cfg(feature="otel")]
impl<S> Layer<S> for OtelBridgeState
where
    S: Subscriber + for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let is_enabled = self.filter.enabled(event.metadata(), ctx.clone());
        if is_enabled {
            if let Some(bridge) = &self.bridging {
                bridge.on_event(event, ctx);
            }
        }
    }

    fn on_new_span(
        &self,
        attrs: &Attributes<'_>,
        id: &tracing::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let is_enabled = self.filter.enabled(attrs.metadata(), ctx.clone());
        if is_enabled {
            if let Some(bridge) = &self.bridging {
                bridge.on_new_span(attrs, id, ctx);
            }
        }
    }

    fn on_record(
        &self,
        span: &tracing::Id,
        values: &Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(bridge) = &self.bridging {
            bridge.on_record(span, values, ctx);
        }
    }

    fn on_follows_from(
        &self,
        span: &tracing::Id,
        follows: &tracing::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(bridge) = &self.bridging {
            bridge.on_follows_from(span, follows, ctx);
        }
    }

    fn on_enter(&self, id: &tracing::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let Some(bridge) = &self.bridging {
            bridge.on_enter(id, ctx);
        }
    }

    fn on_exit(&self, id: &tracing::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let Some(bridge) = &self.bridging {
            bridge.on_exit(id, ctx);
        }
    }

    fn on_close(&self, id: tracing::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if let Some(bridge) = &self.bridging {
            bridge.on_close(id, ctx);
        }
    }

    fn on_id_change(
        &self,
        old: &tracing::Id,
        new: &tracing::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(bridge) = &self.bridging {
            bridge.on_id_change(old, new, ctx);
        }
    }
}
