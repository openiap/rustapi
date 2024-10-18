use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug,info};
#[cfg(not(test))]
use tracing::{error};

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

// use tracing_subscriber::registry::LookupSpan;

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
    // console_subscriber::init();
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


    // use tracing_subscriber::prelude::*;
    // let console_layer =
    // console_subscriber::ConsoleLayer::builder()
    // .spawn();
    // tracing_subscriber::registry()
    // .with(console_layer)
    // .with(subscriber)
    // .init();


    let subscriber = tracing_subscriber::Layer::with_subscriber(
        tracing_subscriber::Layer::and_then(subscriber, filter),
        tracing_subscriber::registry(),
    );
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(()) => {
            debug!("Tracing enabled");
        }
        Err(_e) => {
            #[cfg(not(test))]
            {
                error!("Tracing failed: {:?}", _e);
            }
        }
    }
    info!(
        "enable_tracing rust_log: {:?}, tracing: {:?}",
        rust_log, tracing
    );
}
/// Rust will not allow us to update or remove the tracing, but once that might get possible this function will be used to disable tracing.
#[tracing::instrument(skip_all)]
pub fn disable_tracing() {
    // tracing::dispatcher::get_default(|dispatch| {
    //     dispatch.unsubscribe()
    // });
}
