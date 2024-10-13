use crate::otel;
use openiap_proto::errors::OpenIAPError;
use sysinfo::{get_current_pid, System, NetworkExt, ProcessExt, SystemExt};
use opentelemetry::metrics::Meter;
use opentelemetry::Key;
use opentelemetry::global::{set_error_handler, Error as OtelError};
use tracing::{trace, debug, error};
use std::sync::{Arc, Mutex};
use std::io::Write;

const PROCESS_CPU_USAGE: &str = "process.cpu.usage";
const PROCESS_CPU_UTILIZATION: &str = "process.cpu.utilization";
const PROCESS_MEMORY_USAGE: &str = "process.memory.usage";
const PROCESS_MEMORY_VIRTUAL: &str = "process.memory.virtual";
const PROCESS_DISK_IO: &str = "process.disk.io";
const PROCESS_ELAPSED_TIME: &str = "process.elapsed.time";
const PROCESS_NETWORK_IO: &str = "process.network.io";
const DIRECTION: Key = Key::from_static_str("direction");
const HOSTNAME: Key = Key::from_static_str("hostname");
const OFID: Key = Key::from_static_str("ofid");

/// Register metrics for the process with the given OpenTelemetry meter.
#[tracing::instrument(skip_all, target = "otel::register_metrics")]
pub fn register_metrics(meter: Meter, ofid: &str) -> Result<(), String> {
    let pid = get_current_pid()?;

    let mut sys = System::new_all();
    let core_count = match sys.physical_core_count() {
        Some(core_count) => core_count,
        None => Err("Could not get physical core count")?,
    };
    let process_cpu_utilization = meter
        .f64_observable_gauge(PROCESS_CPU_USAGE)
        .with_description("The percentage of CPU in use.")
        .init();
    let process_elapsed_time = meter
        .i64_observable_gauge(PROCESS_ELAPSED_TIME)
        .with_description("The amount of time the process has been running.")
        .init();
    let process_cpu_usage = meter
        .f64_observable_gauge(PROCESS_CPU_UTILIZATION)
        .with_description("The amount of CPU in use.")
        .init();
    let process_memory_usage = meter
        .i64_observable_gauge(PROCESS_MEMORY_USAGE)
        .with_description("The amount of physical memory in use.")
        .init();
    let process_memory_virtual = meter
        .i64_observable_gauge(PROCESS_MEMORY_VIRTUAL)
        .with_description("The amount of committed virtual memory.")
        .init();
    let process_disk_io = meter
        .i64_observable_gauge(PROCESS_DISK_IO)
        .with_description("Disk bytes transferred.")
        .init();
    let process_network_io = meter
        .u64_observable_gauge(PROCESS_NETWORK_IO)
        .with_description("Network bytes transferred.")
        .init();
    let ofid = ofid.to_string();
    let common_attributes = if let Some(_process) = sys.process(pid) {
        [
            HOSTNAME.string(hostname::get().unwrap_or_default().into_string().unwrap()),
            OFID.string(ofid),
        ]
    } else {
        unimplemented!()
    };
    sys.refresh_networks_list();
    let sys = Arc::new(Mutex::new(sys));

    let result = meter.register_callback(
        &[
            process_cpu_utilization.as_any(),
            process_elapsed_time.as_any(),
            process_cpu_usage.as_any(),
            process_memory_usage.as_any(),
            process_memory_virtual.as_any(),
            process_disk_io.as_any(),
            process_network_io.as_any(),
        ],
        move |context| {
            let mut sys = match sys.lock() {
                Ok(sys) => sys,
                Err(_e) => {
                    return ();
                }                
            };
            sys.refresh_process(pid);
            sys.refresh_networks_list();
            if let Some(process) = sys.process(pid) {
                let cpu_usage: f64 = process.cpu_usage().into();
                let disk_io = process.disk_usage();
                let networks = sys.networks();
                let mut received: u64 = 0;
                let mut transmitted: u64 = 0;
                for (_interface_name, network) in networks.into_iter() {
                    let network_io = network;
                    received += network_io.received();
                    transmitted += network_io.transmitted();
                }
                context.observe_u64(
                    &process_network_io,
                    transmitted,
                    &[common_attributes.as_slice(), &[DIRECTION.string("transmitted")]].concat(),
                );
                context.observe_u64(
                    &process_network_io,
                    received,
                    &[common_attributes.as_slice(), &[DIRECTION.string("received")]].concat(),
                );
                let cpu_utilization: f64 = (cpu_usage / core_count as f64).into();
                let pmemory: i64 = process.memory().try_into().unwrap_or_default();
                let vmemory: i64 = process.virtual_memory().try_into().unwrap_or_default();
                context.observe_i64(
                    &process_memory_usage,
                    pmemory,
                    &common_attributes,
                );
                context.observe_i64(
                    &process_memory_virtual,
                    vmemory,
                    &common_attributes,
                );

                context.observe_f64(&process_cpu_usage, cpu_usage, &common_attributes);
                context.observe_f64(
                    &process_cpu_utilization,
                    cpu_utilization,
                    &common_attributes,
                );
                let elapsed_seconds: u64 = process.run_time();
                let elapsed_mili: i64 = (elapsed_seconds * 1000).try_into().unwrap_or_default();
                context.observe_i64(
                    &process_elapsed_time,
                    elapsed_mili,
                    &common_attributes,
                );

                context.observe_i64(
                    &process_disk_io,
                    disk_io.read_bytes.try_into().unwrap_or_default(),
                    &[common_attributes.as_slice(), &[DIRECTION.string("read")]].concat(),
                );
                context.observe_i64(
                    &process_disk_io,
                    disk_io.written_bytes.try_into().unwrap_or_default(),
                    &[common_attributes.as_slice(), &[DIRECTION.string("write")]].concat(),
                );
                // trace!(
                //     "hostname: {:?}, mem: {:?} v mem: {:?} cpu usage: {:?}  cpu util: {:?} elapsed: {:?} rx: {:?} tx: {:?}",
                //     hostname::get().unwrap_or_default().into_string().unwrap_or_default(),
                //     pmemory, vmemory, cpu_usage, cpu_utilization, elapsed_seconds, received, transmitted
                // );
            }
        },
    );
    match result {
        Ok(_) => {
            let _ = set_error_handler(Box::new(|_error: OtelError| {
                trace!("Error in OpenTelemetry: {:?}", _error);
            }));
            Ok(())
        },
        Err(e) => {
            return Err(format!("Could not register callback: {}", e));
        }
    }
}

use opentelemetry::{KeyValue};
use opentelemetry_otlp::{WithExportConfig};
use opentelemetry_sdk::{Resource};
use std::time::Duration;
use opentelemetry_otlp::{new_exporter, new_pipeline};
use opentelemetry_sdk::{runtime::Tokio};
use opentelemetry::metrics::MeterProvider;

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
/// Initialize telemetry
#[tracing::instrument(skip_all, target = "otel::init_telemetry")]
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
    match hasher.write_all(apihostname.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::new(OpenIAPError::ClientError(format!("Failed to write to hasher: {}", e))));
        }
    }
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
                    error!("Failed to initialize process observer: {}", e);
                }
            }
            providers2.provider = Some(provider);
        }
    }

    Ok(())
}