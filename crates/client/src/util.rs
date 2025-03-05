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
























use once_cell::sync::Lazy;
use tracing_subscriber::{
    fmt,
    layer::Layer, // we need Layer trait in scope
    reload,
    EnvFilter,
    Registry,
};
use tracing_subscriber::prelude::*; // for .with_filter() and .and_then()
use std::sync::OnceLock;

// ----- OPTIONAL: OTel support (compiles only if feature="otel") -----
#[cfg(feature = "otel")]
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
#[cfg(feature = "otel")]
use opentelemetry_otlp::{WithTonicConfig, WithExportConfig, LogExporter};
#[cfg(feature = "otel")]
use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};

#[cfg(feature = "otel")]
static OTEL_LOGGER: Lazy<Option<SdkLoggerProvider>> = Lazy::new(|| {
    let log_url = std::env::var("otel_log_url").unwrap_or_default();
    if log_url.is_empty() {
        return None;
    }

    let exporter = LogExporter::builder()
        .with_tonic()
        .with_tls_config(
            tonic::transport::ClientTlsConfig::new().with_native_roots()
        )
        .with_endpoint(log_url)
        .build()
        .expect("Failed to create OpenTelemetry log exporter");

    let resource = Resource::builder()
        .with_service_name("rust")
        .build();

    let provider = SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    Some(provider)
});

/// Each filter's reload handle is typed against the base `Registry`.
static mut CONSOLE_FILTER_HANDLE: Option<reload::Handle<EnvFilter, Registry>> = None;

#[cfg(feature="otel")]
static mut OTEL_FILTER_HANDLE: Option<reload::Handle<EnvFilter, Registry>> = None;

//------------------------------------------------------------------------------
// TRACING INITIALIZATION
//------------------------------------------------------------------------------

/// With "otel" feature: sets up console + OTel logs, each reloadable, merged for a single subscriber.
#[cfg(feature="otel")]
pub fn init_tracing() {
    // 1) Start with a base Registry
    let base_subscriber = Registry::default();

    // 2) Create a reloadable console filter
    let console_filter = EnvFilter::from_default_env();
    let (console_filter_layer, console_handle) = reload::Layer::new(console_filter);

    // Store the handle
    unsafe {
        CONSOLE_FILTER_HANDLE = Some(console_handle);
    }

    // Then transform it into the actual "fmt" layer
    let console_layer = fmt::layer()
        .with_thread_names(true)
        .with_filter(console_filter_layer);

    // 3) Create a reloadable OTel filter
    let otel_filter = EnvFilter::new("info")
        .add_directive("openiap=debug".parse().unwrap());
    let (otel_filter_layer, otel_handle) = reload::Layer::new(otel_filter);

    // Store the handle
    unsafe {
        OTEL_FILTER_HANDLE = Some(otel_handle);
    }

    // Then transform it into the bridging layer
    let otel_layer = OpenTelemetryTracingBridge::new(
        OTEL_LOGGER.as_ref().expect("OTel logger missing")
    )
    .with_filter(otel_filter_layer);

    // 4) Combine them into a single layer that implements Layer<Registry>:
    //    - "console_layer.and_then(otel_layer)" means events pass to the console layer,
    //      and if they're still enabled, they're also passed to OTel.
    let combined_layer = console_layer.and_then(otel_layer);

    // 5) Attach that combined layer to the Registry
    let final_subscriber = base_subscriber.with(combined_layer);

    // 6) Done: set as global default
    tracing::subscriber::set_global_default(final_subscriber)
        .expect("Failed to set global subscriber");
}

/// Without "otel" feature: sets up only a console logs with a reloadable filter.
#[cfg(not(feature="otel"))]
pub fn init_tracing() {
    // 1) Base Registry
    let base_subscriber = Registry::default();

    // 2) Create reloadable console filter
    let console_filter = EnvFilter::from_default_env();
    let (console_filter_layer, console_handle) = reload::Layer::new(console_filter);

    // Store the handle
    unsafe {
        CONSOLE_FILTER_HANDLE = Some(console_handle);
    }

    // Then transform it into the "fmt" layer
    let console_layer = fmt::layer()
        .with_thread_names(true)
        .with_filter(console_filter_layer);

    // 3) Attach to the Registry
    let final_subscriber = base_subscriber.with(console_layer);

    // 4) Set as global
    tracing::subscriber::set_global_default(final_subscriber)
        .expect("Failed to set global subscriber");
}

//------------------------------------------------------------------------------
// RUNTIME RELOAD
//------------------------------------------------------------------------------

/// Dynamically update the console filter
pub fn update_console_filter(new_filter: &str) {
    if let Ok(parsed_filter) = EnvFilter::try_new(new_filter) {
        unsafe {
            if let Some(handle) = &CONSOLE_FILTER_HANDLE {
                if let Err(err) = handle.modify(|f| *f = parsed_filter) {
                    eprintln!("Failed to update console filter: {err:?}");
                }
            }
        }
    }
}

/// Dynamically update the OTel filter (only if compiled with `--features otel`)
#[cfg(feature="otel")]
pub fn update_otel_filter(new_filter: &str) {
    if let Ok(parsed_filter) = EnvFilter::try_new(new_filter) {
        unsafe {
            if let Some(handle) = &OTEL_FILTER_HANDLE {
                if let Err(err) = handle.modify(|f| *f = parsed_filter) {
                    eprintln!("Failed to update OTel filter: {err:?}");
                }
            }
        }
    }
}
/// This is our guard to ensure we install the subscriber only once.
static INSTALLED: OnceLock<()> = OnceLock::new();

/// Helper: set console + OTel logs together
#[allow(dead_code)]
pub fn enable_tracing(console_log: &str, otel_log: &str) {
    // If not yet installed, do the one-time setup
    if INSTALLED.set(()).is_ok() {
        init_tracing();
    }
    
    update_console_filter(console_log);
    #[cfg(feature="otel")]
    update_otel_filter(otel_log);
}

/// Helper: disable all logs
#[allow(dead_code)]
pub fn disable_tracing() {
    update_console_filter("none");
    #[cfg(feature="otel")]
    update_otel_filter("none");
}
