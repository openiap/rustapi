use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, debug,info};

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























use std::sync::Arc;
use once_cell::sync::Lazy;
use tracing_subscriber::{fmt, layer::SubscriberExt, reload, EnvFilter, Registry, layer::Layered};
#[cfg(feature = "otel")]
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
#[cfg(feature = "otel")]
use opentelemetry_sdk::logs::SdkLoggerProvider;
#[cfg(feature = "otel")]
use opentelemetry_otlp::{WithExportConfig, LogExporter};
#[cfg(feature = "otel")]
use opentelemetry_sdk::Resource;
#[cfg(feature = "otel")]
use opentelemetry::{global, KeyValue};
#[cfg(feature = "otel")]
use opentelemetry_otlp::WithTonicConfig;

#[cfg(feature = "otel")]
static OTEL_LOGGER: Lazy<Option<SdkLoggerProvider>> = Lazy::new(|| {
    let log_url = std::env::var("otel_log_url").unwrap_or_default();
    if log_url.is_empty() {
        return None;
    }

    let exporter = LogExporter::builder()
        .with_tonic()
        .with_tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
        .with_endpoint(log_url)
        .build()
        .expect("Failed to create OpenTelemetry log exporter");

    let resource = Resource::builder().with_service_name("rust").build();

    let provider = SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    Some(provider)
});

#[cfg(feature = "otel")]
static FILTER_RELOAD_HANDLE: Lazy<Arc<reload::Handle<EnvFilter, Registry>>> = Lazy::new(|| {
    let filter = EnvFilter::from_default_env();
    let (layer, handle) = reload::Layer::new(filter);

    let registry = Registry::default().with(layer).with(fmt::layer()).with(OpenTelemetryTracingBridge::new(OTEL_LOGGER.as_ref().expect("OpenTelemetry provider missing")));


    tracing::subscriber::set_global_default(registry)
        .expect("Failed to set global default subscriber");

    Arc::new(handle)
});

#[cfg(not(feature = "otel"))]
static FILTER_RELOAD_HANDLE: Lazy<Arc<reload::Handle<EnvFilter, Registry>>> = Lazy::new(|| {
    let filter = EnvFilter::from_default_env();
    let (layer, handle) = reload::Layer::new(filter);

    let registry = Registry::default().with(layer).with(fmt::layer());

    tracing::subscriber::set_global_default(registry)
        .expect("Failed to set global default subscriber");

    Arc::new(handle)
});

/// Unified function for setting up or updating the tracing configuration
pub fn setup_or_update_tracing(rust_log: &str) {
    if let Ok(new_filter) = EnvFilter::try_new(rust_log) {
        if let Err(e) = FILTER_RELOAD_HANDLE.modify(|current_filter| *current_filter = new_filter) {
            error!("Failed to update tracing filter: {:?}", e);
        } else {
            debug!("Tracing filter updated with rust_log: {}", rust_log);
        }
    } else {
        error!("Invalid filter syntax: {}", rust_log);
    }
}

/// Enable global tracing with optional OpenTelemetry
pub fn enable_tracing(rust_log: &str, _tracing: &str) {
    setup_or_update_tracing(rust_log);
}

/// Disable tracing by setting a "none" filter.
pub fn disable_tracing() {
    setup_or_update_tracing("none");
    info!("Tracing has been disabled.");
}
