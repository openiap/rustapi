// https://github.com/open-telemetry/opentelemetry-rust/blob/fcd7cae39b6730e5f5a907f29e9b0af3ff34d5ce/opentelemetry/CHANGELOG.md?plain=1#L101
use crate::{otel, ClientStatistics};
use openiap_proto::errors::OpenIAPError;
use opentelemetry::metrics::Meter;
use opentelemetry::Key;
use opentelemetry_sdk::metrics::SdkMeterProvider;
use tracing::{debug, error, info};
use std::sync::Arc;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::io::Write;  // Add this line for write_all trait

#[cfg(feature = "otel_cpu")]
const PROCESS_CPU_USAGE: &str = "process.cpu.usage";
#[cfg(feature = "otel_cpu")]
const PROCESS_CPU_UTILIZATION: &str = "process.cpu.utilization";
#[cfg(feature = "otel_memory")]
const PROCESS_MEMORY_USAGE: &str = "process.memory.usage";
#[cfg(feature = "otel_memory")]
const PROCESS_MEMORY_VIRTUAL: &str = "process.memory.virtual";
#[cfg(feature = "otel_disk")]
const PROCESS_DISK_IO: &str = "process.disk.io";
#[cfg(feature = "otel_elapsed")]
const PROCESS_ELAPSED_TIME: &str = "process.elapsed.time";
#[cfg(feature = "otel_network")]
const PROCESS_NETWORK_IO: &str = "process.network.io";
#[cfg(feature = "otel_commands")]
const CLIENT_COMMANDS : &str = "client.commands";
#[cfg(feature = "otel_commands")]
const CLIENT_CONNECTIONS : &str = "client.connections";
#[cfg(feature = "otel_commands")]
const CLIENT_CONNECTION_ATTEMPTS : &str = "client.connection_attempts";
#[cfg(feature = "otel_package_stats")]
const CLIENT_PACKAGE_TX : &str = "client.package_tx";
#[cfg(feature = "otel_package_stats")]
const CLIENT_PACKAGE_RX : &str = "client.package_rx";
#[allow(dead_code)]
const COMMAND: Key = Key::from_static_str("command");
#[cfg(feature = "otel_network")]
const DIRECTION: Key = Key::from_static_str("direction");
const HOSTNAME: Key = Key::from_static_str("hostname");
const OFID: Key = Key::from_static_str("ofid");
#[cfg(feature = "otel_cpu")]
use perf_monitor::cpu::{processor_numbers, ProcessStat};
#[cfg(feature = "otel_disk")]
use perf_monitor::io::get_process_io_stats;
#[cfg(feature = "otel_memory")]
use memory_stats::memory_stats;

/// Register metrics for the process with the given OpenTelemetry meter.
#[tracing::instrument(skip_all, target = "otel::register_metrics")]
pub fn register_metrics(meter: Meter, ofid: &str, stats: &Arc<std::sync::Mutex<ClientStatistics>>) -> Result<(), String> {
    #[cfg(feature = "otel_elapsed")]
    let start_time = SystemTime::now();

    #[cfg(feature = "otel_cpu")]
    let process_stat = ProcessStat::cur().map_err(|e| format!("Could not retrieve process stat: {}", e))?;
    #[cfg(feature = "otel_cpu")]
    let core_count = processor_numbers().map_err(|e| format!("Could not get core numbers: {}", e))?;
    #[cfg(feature = "otel_cpu")]
    let process_stat = Arc::new(Mutex::new( process_stat ));

    #[cfg(feature = "otel_cpu")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_cpu")]
    let process_stat_clone = Arc::clone(&process_stat);
    #[cfg(feature = "otel_cpu")]
    meter
        .f64_observable_gauge(PROCESS_CPU_USAGE)
        .with_description("The percentage of CPU in use.")
        .with_callback(move |gauge| {
            let cpu = &process_stat_clone.lock().unwrap().cpu().unwrap_or_default() * 100.0 as f64;
            gauge.observe(cpu, &common_attributes);
        })
        .build();
    #[cfg(feature = "otel_cpu")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    
    #[cfg(feature = "otel_cpu")]
    meter
        .f64_observable_gauge(PROCESS_CPU_UTILIZATION)
        .with_description("The percentage of CPU in use.")
        .with_callback(move |gauge| {
            let cpu = process_stat.lock().unwrap().cpu().unwrap_or_default() * 100.0 as f64;
            let cpu_utilization = cpu / core_count as f64;
            gauge.observe(cpu_utilization, &common_attributes);
        })
        .build();
    #[cfg(feature = "otel_network")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_network")]
    meter
        .u64_observable_gauge(PROCESS_NETWORK_IO)
        .with_description("Network bytes transferred.")
        .with_callback(move |gauge| {
            let mut net_rx: u64 = 0;
            let mut net_tx: u64 = 0;
            match System::new().networks() {
                Ok(netifs) => {
                    for netif in netifs.values() {
                        let s = System::new().network_stats(&netif.name);
                        match s {
                            Ok(stats) => {
                                net_rx += stats.rx_bytes.as_u64();
                                net_tx += stats.tx_bytes.as_u64();
                            }
                            Err(_x) => (),
                        }
                    }
                }
                Err(_x) => ()
            }
            gauge.observe(net_rx, &[common_attributes.as_slice(), &[KeyValue::new(DIRECTION, "receive")]].concat());
            gauge.observe(net_tx, &[common_attributes.as_slice(), &[KeyValue::new(DIRECTION, "transmit")]].concat());
        })
        .build();
    #[cfg(feature = "otel_disk")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_disk")]
    meter
        .u64_observable_gauge(PROCESS_DISK_IO)
        .with_description("Disk bytes transferred.")
        .with_callback(move |gauge| {
            let io_stat = get_process_io_stats().unwrap_or_default();
            gauge.observe(io_stat.read_bytes, &[common_attributes.as_slice(), &[KeyValue::new(DIRECTION, "read")]].concat());
            gauge.observe(io_stat.write_bytes, &[common_attributes.as_slice(), &[KeyValue::new(DIRECTION, "write")]].concat());
        })
        .build();

    #[cfg(feature = "otel_memory")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_memory")]
    meter
        .u64_observable_gauge(PROCESS_MEMORY_USAGE)
        .with_description("The amount of physical memory in use.")
        .with_callback(move |gauge| {
            let mut physical_mem: u64 = 0;
            if let Some(usage) = memory_stats() {
                physical_mem = usage.physical_mem as u64;
            }
            gauge.observe(physical_mem, &common_attributes);
        })
        .build();
    #[cfg(feature = "otel_memory")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_memory")]
    meter
        .u64_observable_gauge(PROCESS_MEMORY_VIRTUAL)
        .with_description("The amount of committed virtual memory.")
        .with_callback(move |gauge| {
            let mut virtual_mem: u64 = 0;
            if let Some(usage) = memory_stats() {
                virtual_mem = usage.virtual_mem as u64;
            }
            gauge.observe(virtual_mem, &common_attributes);
        })
        .build();
    #[cfg(feature = "otel_elapsed")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_elapsed")]
    meter
        .u64_observable_gauge(PROCESS_ELAPSED_TIME)
        .with_description("The uptime of the process in milliseconds.")
        .with_callback(move |gauge| {
            let elapsed_time = start_time.elapsed().unwrap_or_default().as_millis() as u64;
            gauge.observe(elapsed_time, &common_attributes);
        })
        .build();
    #[cfg(feature = "otel_connections")]        
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_connections")]
    meter
        .u64_observable_counter(CLIENT_CONNECTIONS)
        .with_description("Client Connections")
        .with_callback({
            let stats = Arc::clone(stats);
            move |counter| {
                let stats = stats.lock().unwrap();
                if stats.connections > 0 {
                    counter.observe(stats.connections, &common_attributes);
                }
            }
        })
        .build();
    #[cfg(feature = "otel_connections")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_connections")]
    meter
        .u64_observable_counter(CLIENT_CONNECTION_ATTEMPTS)
        .with_description("Client Connection Attempts")
        .with_callback({
            let stats = Arc::clone(stats);
            move |counter| {
                let stats = stats.lock().unwrap();
                if stats.connection_attempts > 0 {
                    counter.observe(stats.connection_attempts, &common_attributes);
                }
            }
        })
        .build();
    #[cfg(feature = "otel_package_stats")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_package_stats")]
    meter
        .u64_observable_counter(CLIENT_PACKAGE_TX)
        .with_description("Client Package TX")
        .with_callback({
            let stats = Arc::clone(stats);
            move |counter| {
                let stats = stats.lock().unwrap();
                if stats.package_tx > 0 {
                    counter.observe(stats.package_tx, &common_attributes);
                }
            }
        })
        .build();
    #[cfg(feature = "otel_package_stats")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_package_stats")]
    meter
        .u64_observable_counter(CLIENT_PACKAGE_RX)
        .with_description("Client Package RX")
        .with_callback({
            let stats = Arc::clone(stats);
            move |counter| {
                let stats = stats.lock().unwrap();
                if stats.package_rx > 0 {
                    counter.observe(stats.package_rx, &common_attributes);
                }
            }
        })
        .build();

    #[cfg(feature = "otel_commands")]
    let common_attributes = [
        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
        KeyValue::new(OFID, ofid.to_string()),
        KeyValue::new("PID", std::process::id().to_string()),
    ];
    #[cfg(feature = "otel_commands")]
    meter
        .u64_observable_counter(CLIENT_COMMANDS)
        .with_description("Client Commands")
        .with_callback({
            let stats = Arc::clone(stats);
            move |counter| {
                let stats = stats.lock().unwrap();
                if stats.signin > 0 {
                    counter.observe(stats.signin, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND, "signin")]].concat());
                }
                if stats.download > 0 {
                    counter.observe(stats.download, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND, "download")]].concat());
                }
                if stats.getdocumentversion > 0 {
                    counter.observe(stats.getdocumentversion, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"getdocumentversion")]].concat());
                }
                if stats.customcommand > 0 {
                    counter.observe(stats.customcommand, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"customcommand")]].concat());
                }
                if stats.listcollections > 0 {
                    counter.observe(stats.listcollections, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"listcollections")]].concat());
                }
                if stats.createcollection > 0 {
                    counter.observe(stats.createcollection, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"createcollection")]].concat());
                }
                if stats.dropcollection > 0 {
                    counter.observe(stats.dropcollection, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"dropcollection")]].concat());
                }
                if stats.ensurecustomer > 0 {
                    counter.observe(stats.ensurecustomer, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"ensurecustomer")]].concat());
                }
                if stats.invokeopenrpa > 0 {
                    counter.observe(stats.invokeopenrpa, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"invokeopenrpa")]].concat());
                }
                if stats.registerqueue > 0 {
                    counter.observe(stats.registerqueue, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"registerqueue")]].concat());
                }
                if stats.registerexchange > 0 {
                    counter.observe(stats.registerexchange, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"registerexchange")]].concat());
                }
                if stats.unregisterqueue > 0 {
                    counter.observe(stats.unregisterqueue, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"unregisterqueue")]].concat());
                }
                if stats.watch > 0 {
                    counter.observe(stats.watch, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"watch")]].concat());
                }
                if stats.unwatch > 0 {
                    counter.observe(stats.unwatch , 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"unwatch")]].concat());
                }
                if stats.queuemessage > 0 {
                    counter.observe(stats.queuemessage, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"queuemessage")]].concat());
                }
                if stats.pushworkitem > 0 {
                    counter.observe(stats.pushworkitem, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"pushworkitem")]].concat());
                }
                if stats.pushworkitems > 0 {
                    counter.observe(stats.pushworkitems, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"pushworkitems")]].concat());
                }
                if stats.popworkitem > 0 {
                    counter.observe(stats.popworkitem, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"popworkitem")]].concat());
                }
                if stats.updateworkitem > 0 {
                    counter.observe(stats.updateworkitem, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"updateworkitem")]].concat());
                }
                if stats.deleteworkitem > 0 {
                    counter.observe(stats.deleteworkitem, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"deleteworkitem")]].concat());
                }
                if stats.addworkitemqueue > 0 {
                    counter.observe(stats.addworkitemqueue, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"addworkitemqueue")]].concat());
                }
                if stats.updateworkitemqueue > 0 {
                    counter.observe(stats.updateworkitemqueue, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"updateworkitemqueue")]].concat());
                }
                if stats.deleteworkitemqueue > 0 {
                    counter.observe(stats.deleteworkitemqueue, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"deleteworkitemqueue")]].concat());
                }
                if stats.getindexes > 0 {
                    counter.observe(stats.getindexes, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"getindexes")]].concat());
                }
                if stats.createindex > 0 {
                    counter.observe(stats.createindex, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"createindex")]].concat());
                }
                if stats.dropindex > 0 {
                    counter.observe(stats.dropindex, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"dropindex")]].concat());
                }
                if stats.upload > 0 {
                    counter.observe(stats.upload, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"upload")]].concat());
                }
                if stats.query > 0 {
                    counter.observe(stats.query, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"query")]].concat());
                }
                if stats.count > 0 {
                    counter.observe(stats.count, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"count")]].concat());
                }
                if stats.distinct > 0 {
                    counter.observe(stats.distinct, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"distinct")]].concat());
                }
                if stats.aggregate > 0 {
                    counter.observe(stats.aggregate, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"aggregate")]].concat());
                }
                if stats.insertone > 0 {
                    counter.observe(stats.insertone, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"insertone")]].concat());
                }
                if stats.insertmany > 0 {
                    counter.observe(stats.insertmany, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"insertmany")]].concat());
                }
                if stats.insertorupdateone > 0 {
                    counter.observe(stats.insertorupdateone, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"insertorupdateone")]].concat());
                }
                if stats.insertorupdatemany > 0 {
                    counter.observe(stats.insertorupdatemany, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"insertorupdatemany")]].concat());
                }
                if stats.updateone > 0 {
                    counter.observe(stats.updateone, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"updateone")]].concat());
                }
                if stats.updatedocument > 0 {
                    counter.observe(stats.updatedocument, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"updatedocument")]].concat());
                }
                if stats.deleteone > 0 {
                    counter.observe(stats.deleteone, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"deleteone")]].concat());
                }
                if stats.deletemany > 0 {
                    counter.observe(stats.deletemany, 
                        &[common_attributes.as_slice(), &[KeyValue::new(COMMAND,"deletemany")]].concat());
                }
            }
        })
        .build();
    Ok(())
}

use opentelemetry::KeyValue;
use opentelemetry_otlp::{WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::Resource;
use std::time::SystemTime;
use opentelemetry_otlp::MetricExporter;
use opentelemetry::metrics::MeterProvider;
use tracing_subscriber::EnvFilter;
#[allow(dead_code)]
struct ProviderWrapper {
    provider: Option<opentelemetry_sdk::metrics::SdkMeterProvider>,
    tracer: Option<opentelemetry_sdk::trace::SdkTracerProvider>,
    logger: Option<opentelemetry_sdk::logs::SdkLoggerProvider>,
}
use lazy_static::lazy_static;
lazy_static! {
    static ref provider1: std::sync::Mutex<ProviderWrapper> = std::sync::Mutex::new(ProviderWrapper {
        provider: None,
        tracer: None,
        logger: None
    });
    static ref provider2: std::sync::Mutex<ProviderWrapper> = std::sync::Mutex::new(ProviderWrapper {
        provider: None,
        tracer: None,
        logger: None
    });
}

#[derive(Clone)]
struct MetricValue {
    f64value: f64,
    u64value: u64,
    i64value: i64,
    description: String,
    enabled: bool,
}

static METRIC_VALUES: Lazy<std::sync::Mutex<HashMap<String, MetricValue>>> = Lazy::new(|| {
    std::sync::Mutex::new(HashMap::new())
});

#[derive(Clone)]
struct MetricsContext {
    version: String,
    agent_name: String,
    agent_version: String,
    ofid: String,
}

static METRICS_CONTEXT: Lazy<std::sync::Mutex<Option<MetricsContext>>> = Lazy::new(|| {
    std::sync::Mutex::new(None)
});

/// Initialize telemetry
#[allow(unused_variables)]
#[tracing::instrument(skip_all, target = "otel::init_telemetry")]
pub fn init_telemetry(service_name: &str, agent_name: &str, agent_version: &str, version: &str, apihostname: &str, 
    metric_url: &str, trace_url: &str, log_url: &str, 
    stats: &Arc<std::sync::Mutex<ClientStatistics>>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let enable_analytics = std::env::var("enable_analytics").unwrap_or("".to_string());
    let enable_analytics: bool = !enable_analytics.eq_ignore_ascii_case("false");
    let resource = Resource::builder().with_service_name(service_name.to_string())
        .with_attribute(KeyValue::new("service.version", version.to_string() ))
        .with_attribute(KeyValue::new("agent.name", agent_name.to_string() ))
        .with_attribute(KeyValue::new("agent.version", agent_version.to_string() ))
        .build();

    let mut hasher = md5::Context::new();
    match hasher.write_all(apihostname.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::new(OpenIAPError::ClientError(format!("Failed to write to hasher: {}", e))));
        }
    }
    let ofid = format!("{:x}", hasher.compute());

    // if OTEL_METRIC_EXPORT_INTERVAL is not set, set it to 10000 ( instad of 60000 )
    if std::env::var("OTEL_METRIC_EXPORT_INTERVAL").is_err() {
        std::env::set_var("OTEL_METRIC_EXPORT_INTERVAL", "10000");
    }
    
    // Store the context
    *METRICS_CONTEXT.lock().unwrap() = Some(MetricsContext {
        version: version.to_string(),
        agent_name: agent_name.to_string(),
        agent_version: agent_version.to_string(),
        ofid: ofid.clone(),
    });

    if enable_analytics {
        debug!("Initializing generic telemetry");
        let mut providers1 = provider1.lock().unwrap();
        if providers1.provider.is_none() {
            let exporter1 = MetricExporter::builder()
                .with_tonic()
                .with_tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
                .with_endpoint("https://otel.stats.openiap.io:443")
                .build()
                .expect("Failed to create metric exporter");
            let provider = SdkMeterProvider::builder()
                .with_periodic_exporter(exporter1)
                .with_resource(resource.clone())
                .build();
            let meter1 = provider.meter("process-meter1");
            // let meter: opentelemetry::metrics::Meter = meterprovider1.meter("process-meter1");
            // when not using global::set_meter_provider we need to keep it alive using ProivderWrapper
            match otel::register_metrics(meter1, &ofid, stats) {
                Ok(_) => (),
                Err(e) => {
                    debug!("Failed to initialize process observer: {}", e);
                }
            }
            providers1.provider = Some(provider);
        }
    }

    if !log_url.is_empty() {
        #[cfg(not(test))]
        {   
            crate::set_otel_url(log_url, trace_url, &ofid, version, service_name, agent_name, agent_version);
        }
        debug!("added {} for logging observability", log_url);
    }
    if !metric_url.is_empty() {
        let mut providers2 = provider2.lock().unwrap();
        if providers2.provider.is_none() {
            let exporter2 = MetricExporter::builder()
                .with_tonic()
                .with_tls_config(tonic::transport::ClientTlsConfig::new().with_native_roots())
                .with_endpoint(metric_url)
                .build()
                .expect("Failed to create metric exporter");
            let provider = SdkMeterProvider::builder()
                .with_periodic_exporter(exporter2)
                .with_resource(resource.clone())
                .build();

            let meter2 = provider.meter("process-meter2");
            // when not using global::set_meter_provider we need to keep it alive using ProivderWrapper
            match otel::register_metrics(meter2, &ofid, stats) {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to initialize process observer: {}", e);
                }
            }
            providers2.provider = Some(provider);
            debug!("added {} for performance observability", metric_url);
        }
    }

    Ok(())
}
/// Create/Update an onservable gauge metric that can be updated dynamically.
/// this means the value will be stored and send doing each metric update.
pub fn set_f64_observable_gauge(name: &str, value: f64, description: &str) -> Result<(), String> {
    let providers2 = provider2.lock().unwrap();
    if providers2.provider.is_none() {
        return Err("Provider not initialized".to_string());
    }
    
    let name_owned = name.to_string();
    
    // Check if metric already exists and update if it does
    let mut metrics = METRIC_VALUES.lock().unwrap();
    if let Some(metric) = metrics.get_mut(&name_owned) {
        metric.f64value = value;
        metric.enabled = true;
        return Ok(());
    }
    
    // Store metric info in our static map for new metrics
    metrics.insert(name_owned.clone(), MetricValue {
        f64value: value,
        u64value: 0,
        i64value: 0,
        description: description.to_string(),
        enabled: true,
    });
    
    if let Some(provider) = &providers2.provider {
        let meter = provider.meter("custommeter");
        let name_for_callback = name_owned.clone();
        
        // Get the metrics context
        let context = METRICS_CONTEXT.lock().unwrap().clone();
        
        meter
            .f64_observable_gauge(name_owned)
            .with_description(description.to_string())
            .with_callback(move |gauge| {
                let enabled = METRIC_VALUES.lock().unwrap().get(&name_for_callback).map(|m| m.enabled).unwrap_or_default();
                if !enabled {
                    return;
                }                
                if let Some(metric) = METRIC_VALUES.lock().unwrap().get(&name_for_callback) {
                    let mut attributes = vec![
                        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
                        KeyValue::new("metric_name", name_for_callback.clone()),
                        KeyValue::new("PID", std::process::id().to_string()),
                    ];
                    
                    // Add context attributes if available
                    if let Some(ctx) = &context {
                        attributes.extend_from_slice(&[
                            KeyValue::new("service.version", ctx.version.clone()),
                            KeyValue::new("agent.name", ctx.agent_name.clone()),
                            KeyValue::new("agent.version", ctx.agent_version.clone()),
                            KeyValue::new(OFID, ctx.ofid.clone()),
                        ]);
                    }
                    
                    gauge.observe(metric.f64value, &attributes);
                }
            })
            .build();
        
        Ok(())
    } else {
        Err("Provider is None".to_string())
    }
}
/// Create/Update an onservable gauge metric that can be updated dynamically.
/// this means the value will be stored and send doing each metric update.
pub fn set_u64_observable_gauge(name: &str, value: u64, description: &str) -> Result<(), String> {
    let providers2 = provider2.lock().unwrap();
    if providers2.provider.is_none() {
        return Err("Provider not initialized".to_string());
    }
    
    let name_owned = name.to_string();
    
    // Check if metric already exists and update if it does
    let mut metrics = METRIC_VALUES.lock().unwrap();
    if let Some(metric) = metrics.get_mut(&name_owned) {
        metric.u64value = value;
        metric.enabled = true;
        return Ok(());
    }
    
    // Store metric info in our static map for new metrics
    metrics.insert(name_owned.clone(), MetricValue {
        f64value: 0.0,
        u64value: value,
        i64value: 0,
        description: description.to_string(),
        enabled: true,
    });
    
    if let Some(provider) = &providers2.provider {
        let meter = provider.meter("custommeter");
        let name_for_callback = name_owned.clone();
        
        // Get the metrics context
        let context = METRICS_CONTEXT.lock().unwrap().clone();
        
        meter
            .u64_observable_gauge(name_owned)
            .with_description(description.to_string())
            .with_callback(move |gauge| {
                let enabled = METRIC_VALUES.lock().unwrap().get(&name_for_callback).map(|m| m.enabled).unwrap_or_default();
                if !enabled {
                    return;
                }                
                if let Some(metric) = METRIC_VALUES.lock().unwrap().get(&name_for_callback) {
                    let mut attributes = vec![
                        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
                        KeyValue::new("metric_name", name_for_callback.clone()),
                        KeyValue::new("PID", std::process::id().to_string()),
                    ];
                    
                    // Add context attributes if available
                    if let Some(ctx) = &context {
                        attributes.extend_from_slice(&[
                            KeyValue::new("service.version", ctx.version.clone()),
                            KeyValue::new("agent.name", ctx.agent_name.clone()),
                            KeyValue::new("agent.version", ctx.agent_version.clone()),
                            KeyValue::new(OFID, ctx.ofid.clone()),
                        ]);
                    }
                    
                    gauge.observe(metric.u64value, &attributes);
                }
            })
            .build();
        
        Ok(())
    } else {
        Err("Provider is None".to_string())
    }
}
/// Create/Update an onservable gauge metric that can be updated dynamically.
/// this means the value will be stored and send doing each metric update.
pub fn set_i64_observable_gauge(name: &str, value: i64, description: &str) -> Result<(), String> {
    let providers2 = provider2.lock().unwrap();
    if providers2.provider.is_none() {
        return Err("Provider not initialized".to_string());
    }
    
    let name_owned = name.to_string();
    
    // Check if metric already exists and update if it does
    let mut metrics = METRIC_VALUES.lock().unwrap();
    if let Some(metric) = metrics.get_mut(&name_owned) {
        metric.i64value = value;
        metric.enabled = true;
        return Ok(());
    }
    
    // Store metric info in our static map for new metrics
    metrics.insert(name_owned.clone(), MetricValue {
        f64value: 0.0,
        u64value: 0,
        i64value: value,
        description: description.to_string(),
        enabled: true,
    });
    
    if let Some(provider) = &providers2.provider {
        let meter = provider.meter("custommeter");
        let name_for_callback = name_owned.clone();
        
        // Get the metrics context
        let context = METRICS_CONTEXT.lock().unwrap().clone();
        
        meter
            .i64_observable_gauge(name_owned)
            .with_description(description.to_string())
            .with_callback(move |gauge| {
                let enabled = METRIC_VALUES.lock().unwrap().get(&name_for_callback).map(|m| m.enabled).unwrap_or_default();
                if !enabled {
                    return;
                }                
                if let Some(metric) = METRIC_VALUES.lock().unwrap().get(&name_for_callback) {
                    let mut attributes = vec![
                        KeyValue::new(HOSTNAME, hostname::get().unwrap_or_default().into_string().unwrap()),
                        KeyValue::new("metric_name", name_for_callback.clone()),
                        KeyValue::new("PID", std::process::id().to_string()),
                    ];
                    
                    // Add context attributes if available
                    if let Some(ctx) = &context {
                        attributes.extend_from_slice(&[
                            KeyValue::new("service.version", ctx.version.clone()),
                            KeyValue::new("agent.name", ctx.agent_name.clone()),
                            KeyValue::new("agent.version", ctx.agent_version.clone()),
                            KeyValue::new(OFID, ctx.ofid.clone()),
                        ]);
                    }
                    
                    gauge.observe(metric.i64value, &attributes);
                }
            })
            .build();
        
        Ok(())
    } else {
        Err("Provider is None".to_string())
    }
}
/// Disable an observable gauge metric. 
pub fn disable_observable_gauge(name: &str) {
    // Check if metric already exists and update if it does
    let mut metrics = METRIC_VALUES.lock().unwrap();
    if let Some(metric) = metrics.get_mut(name) {
        metric.enabled = false;
    }
}
#[allow(dead_code)]
pub fn get_metric_value(name: &str) -> Option<f64> {
    METRIC_VALUES.lock().unwrap()
        .get(name)
        .map(|m| m.f64value)
}

#[allow(dead_code)]
pub fn list_metrics() -> Vec<(String, f64, String)> {
    METRIC_VALUES.lock().unwrap()
        .iter()
        .map(|(name, metric)| (name.clone(), metric.f64value, metric.description.clone()))
        .collect()
}

use tracing_subscriber::{fmt, layer::SubscriberExt, reload, Registry};


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