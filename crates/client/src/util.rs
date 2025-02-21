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

use tracing_subscriber::{fmt, layer::SubscriberExt, reload, EnvFilter, Registry};
use once_cell::sync::Lazy;
use std::sync::Arc;


// Static global to hold a reload handle for updating the filter dynamically.
// reload::Handle expects both a Layer (EnvFilter) and a Subscriber (Registry).
static FILTER_RELOAD_HANDLE: Lazy<Arc<reload::Handle<EnvFilter, Registry>>> = Lazy::new(|| {
    let filter = EnvFilter::from_default_env();
    let (layer, handle) = reload::Layer::new(filter);

    let subscriber = Registry::default().with(layer).with(fmt::layer());

    // Set the global default tracing subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    Arc::new(handle)
});

// Unified function for initializing or updating the tracing filter and span events
#[allow(dead_code)]
pub fn setup_or_update_tracing(rust_log: &str, tracing: &str) {
    // Configure the filter (log level)
    if let Ok(new_filter) = EnvFilter::try_new(rust_log) {
        // Update the existing filter using the reload handle
        if let Err(e) = FILTER_RELOAD_HANDLE.modify(|current_filter| *current_filter = new_filter) {
            error!("Failed to update tracing filter: {:?}", e);
        } else {
            debug!("Tracing filter updated with rust_log: {}", rust_log);
        }
    } else {
        error!("Invalid filter syntax: {}", rust_log);
    }

    // Configure the span event tracking based on user input (tracing level)
    let tracing = tracing.to_string();
    let subscriber = fmt::layer();
    let updated_subscriber = match tracing.to_lowercase().as_str() {
        "new" => subscriber.with_span_events(fmt::format::FmtSpan::NEW),
        "enter" => subscriber.with_span_events(fmt::format::FmtSpan::ENTER),
        "exit" => subscriber.with_span_events(fmt::format::FmtSpan::EXIT),
        "close" => subscriber.with_span_events(fmt::format::FmtSpan::CLOSE),
        "none" => subscriber.with_span_events(fmt::format::FmtSpan::NONE),
        "active" => subscriber.with_span_events(fmt::format::FmtSpan::ACTIVE),
        "full" => subscriber.with_span_events(fmt::format::FmtSpan::FULL),
        _ => subscriber,
    };

    // Add the layer to the existing registry
    let registry = Registry::default().with(updated_subscriber);

    if let Err(e) = tracing::subscriber::set_global_default(registry) {
        debug!("Global subscriber is already set, skipping reinitialization: {:?}", e);
    } else {
        info!("Tracing setup/updated with rust_log: {:?}, tracing: {:?}", rust_log, tracing);
    }
}


/// Enable global tracing, allowing dynamic configuration of tracing settings.
/// - rust_log is a [tracing_subscriber::EnvFilter] string (use empty string to use environment variable RUST_LOG).
/// - tracing is a string that can be empty for nothing, or one of the following: new, enter, exit, close, active, or full.
pub fn enable_tracing(rust_log: &str, tracing: &str) {
    #[cfg(not(feature = "otel"))]
    setup_or_update_tracing(rust_log, tracing);

    #[cfg(feature = "otel")]
    use crate::otel;
    #[cfg(feature = "otel")]
    otel::setup_or_update_tracing(rust_log, tracing);
}

/// Disable tracing by setting a "none" filter.
pub fn disable_tracing() {
    #[cfg(not(feature = "otel"))]
    setup_or_update_tracing("none", "none");

    #[cfg(feature = "otel")]
    use crate::otel;
    #[cfg(feature = "otel")]
    otel::setup_or_update_tracing("none", "none");

    info!("Tracing has been disabled.");
}