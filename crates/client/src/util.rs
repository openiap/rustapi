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
use tracing_subscriber::{
    fmt,
    layer::{Layer, SubscriberExt},
    reload,
    filter::Filtered,
    EnvFilter,
    Registry,
};
use tracing_subscriber::layer::Layered;
use tracing_subscriber::prelude::*; // for .with()

use tracing::span::{Attributes, Record};

// ----- OTel imports (only if feature="otel") -----
#[cfg(feature = "otel")]
use {
    // The bridging layer
    opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge,

    // OTLP exporter
    opentelemetry_otlp::{LogExporter, WithTonicConfig, WithExportConfig},

    // The OTel SDK provider + logger
    // (Under the hood, it uses a batch processor if you do `with_batch_exporter()`,
    //  but from the bridging perspective, the logger type is `SdkLogger`.)
    opentelemetry_sdk::{
        logs::{SdkLoggerProvider, SdkLogger},
        Resource,
    },
    // For the `S: Subscriber` bound
    tracing::Subscriber,
};

/// Whether the global subscriber has been set up
static INSTALLED: OnceCell<()> = OnceCell::new();

/// Our type for the base console layering:
type ConsoleLayered = Layered<
    Filtered<fmt::Layer<Registry>, reload::Layer<EnvFilter, Registry>, Registry>,
    Registry
>;

/// If "otel" is on, we layer OTel bridging on top of the console subscriber.
#[cfg(feature="otel")]
type FullSubscriber = Layered<
    reload::Layer<OtelBridgeState, ConsoleLayered>,
    ConsoleLayered
>;

/// Reload handle for the console filter
static CONSOLE_FILTER_HANDLE: Lazy<OnceCell<reload::Handle<EnvFilter, Registry>>> =
    Lazy::new(OnceCell::new);

/// Reload handle for the OTel bridging (only if feature="otel")
#[cfg(feature="otel")]
static OTEL_BRIDGE_HANDLE: Lazy<OnceCell<reload::Handle<OtelBridgeState, ConsoleLayered>>> =
    Lazy::new(OnceCell::new);

/// ---------------------------------------------------------------------------
/// PUBLIC API
/// ---------------------------------------------------------------------------

/// Call `enable_tracing(...)` as many times as you want:
///  - On first call, installs a global subscriber (console + optional OTel).
///  - On subsequent calls, just reconfigures filters & bridging.
pub fn enable_tracing(
    console_filter: &str,
    otel_filter: &str,
    otel_url: Option<&str>,
) {
    // If not yet installed, do a one-time global subscriber install:
    if INSTALLED.set(()).is_ok() {
        install_global_subscriber();
    }

    // Update console filter
    update_console_filter(console_filter);

    // If "otel" feature is on, update bridging
    #[cfg(feature="otel")]
    {
        let url = otel_url.unwrap_or("").trim();
        update_otel_state(url, otel_filter);
    }
}

/// Disable all logging: sets console + OTel filters to "none" and bridging = None
pub fn disable_tracing() {
    update_console_filter("none");
    #[cfg(feature="otel")]
    {
        update_otel_state("", "none"); // empty URL => bridging = None
    }
}

/// Dynamically update the console filter (e.g. "warn,mycrate=info").
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

/// ---------------------------------------------------------------------------
/// INTERNAL: One-time global subscriber
/// ---------------------------------------------------------------------------
fn install_global_subscriber() {
    // 1) Base registry
    let registry = Registry::default();

    // 2) Create a reloadable console filter
    let console_filter = EnvFilter::new("info");
    let (console_filter_layer, console_reload_handle) = reload::Layer::new(console_filter);

    // Wrap with a fmt layer
    let console_layer = fmt::layer()
        .with_thread_names(true)
        .with_filter(console_filter_layer);

    // Combine => yields `ConsoleLayered`
    let console_subscriber = registry.with(console_layer);

    // 3) If "otel" is on, create a reloadable bridging layer, default = none
    #[cfg(feature="otel")]
    let (otel_layer, otel_reload_handle) = reload::Layer::new(OtelBridgeState::none());

    #[cfg(feature="otel")]
    let full_subscriber: FullSubscriber = console_subscriber.with(otel_layer);

    #[cfg(not(feature="otel"))]
    let full_subscriber = console_subscriber;

    // 4) Set as global default
    tracing::subscriber::set_global_default(full_subscriber)
        .expect("Failed to set global subscriber");

    // 5) Store reload handles
    CONSOLE_FILTER_HANDLE
        .set(console_reload_handle)
        .expect("Failed to store console filter reload handle");

    #[cfg(feature="otel")]
    {
        // We cannot just `.expect(...)` here because that would require `Debug` on OtelBridgeState.
        // We'll handle the error manually (which is extremely unlikely unless it's already set).
        if let Err(_already_set) = OTEL_BRIDGE_HANDLE.set(otel_reload_handle) {
            panic!("Failed to store OTel bridging reload handle (already set?)");
        }
    }
}

/// If `url` is empty => bridging = None, else bridging = some new OTel provider
#[cfg(feature="otel")]
fn update_otel_state(url: &str, filter: &str) {
    let new_state = if url.is_empty() {
        OtelBridgeState::none()
    } else {
        OtelBridgeState::some(url, filter)
    };

    if let Some(handle) = OTEL_BRIDGE_HANDLE.get() {
        if let Err(err) = handle.modify(|state| {
            *state = new_state;
        }) {
            eprintln!("Failed to update OTel bridging state: {err}");
        }
    }
}

/// ---------------------------------------------------------------------------
/// OTelBridgeState: bridging + a local filter
/// ---------------------------------------------------------------------------
/// We *cannot* `#[derive(Debug)]` because `OpenTelemetryTracingBridge<SdkLoggerProvider,SdkLogger>` 
/// doesn't implement Debug.
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

    fn some(endpoint: &str, filter_directives: &str) -> Self {
        // 1) Build an OTLP exporter
        let exporter = LogExporter::builder()
            .with_tonic()
            .with_tls_config(
                tonic::transport::ClientTlsConfig::new().with_native_roots()
            )
            .with_endpoint(endpoint)
            .build()
            .expect("Failed to build OTel log exporter");

        // 2) Resource
        let resource = Resource::builder()
            .with_service_name("rust")
            .build();

        // 3) Build an SdkLoggerProvider with a "batch exporter"
        //    => bridging wants <SdkLoggerProvider,SdkLogger>
        let provider = SdkLoggerProvider::builder()
            .with_batch_exporter(exporter)
            .with_resource(resource)
            .build();

        // 4) bridging
        let bridging = Some(OpenTelemetryTracingBridge::new(&provider));

        // 5) parse user filter
        let filter = EnvFilter::try_new(filter_directives)
            .unwrap_or_else(|_| EnvFilter::new("info"));

        Self { bridging, filter }
    }
}

/// A custom layer that checks if an event/span passes our `filter`,
/// and if so, forwards it to bridging (if present).
#[cfg(feature="otel")]
impl<S> Layer<S> for OtelBridgeState
where
    S: Subscriber + for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // Clone the context for the filter check => doesn't consume `ctx`
        let is_enabled = self.filter.enabled(event.metadata(), ctx.clone());
        if is_enabled {
            if let Some(bridge) = &self.bridging {
                // now pass the original ctx to bridging
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
        // No filter check needed here if you want all records
        // But if you do want to check the filter, you'd do it similarly:
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
