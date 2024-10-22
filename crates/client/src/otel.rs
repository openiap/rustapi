use crate::{otel, ClientStatistics};
use openiap_proto::errors::OpenIAPError;
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
#[allow(dead_code)]
const PROCESS_NETWORK_IO: &str = "process.network.io";
const CLIENT_COMMANDS : &str = "client.commands";
const CLIENT_CONNECTIONS : &str = "client.connections";
const CLIENT_CONNECTION_ATTEMPTS : &str = "client.connection_attempts";
const CLIENT_PACKAGE_TX : &str = "client.package_tx";
const CLIENT_PACKAGE_RX : &str = "client.package_rx";
const COMMAND: Key = Key::from_static_str("command");
const DIRECTION: Key = Key::from_static_str("direction");
const HOSTNAME: Key = Key::from_static_str("hostname");
const OFID: Key = Key::from_static_str("ofid");

use perf_monitor::cpu::{processor_numbers, ProcessStat};
use perf_monitor::io::get_process_io_stats;
use memory_stats::memory_stats;

/// Register metrics for the process with the given OpenTelemetry meter.
#[tracing::instrument(skip_all, target = "otel::register_metrics")]
pub fn register_metrics(meter: Meter, ofid: &str, stats: &Arc<std::sync::Mutex<ClientStatistics>>) -> Result<(), String> {
    let process_stat = ProcessStat::cur().map_err(|e| format!("Could not retrieve process stat: {}", e))?;
    let core_count = processor_numbers().map_err(|e| format!("Could not get core numbers: {}", e))?;
    let process_stat = Arc::new(Mutex::new( process_stat ));
    let start_time = SystemTime::now();

    let process_cpu_usage = meter
        .f64_observable_gauge(PROCESS_CPU_USAGE)
        .with_description("The percentage of CPU in use.")
        .init();
    let process_cpu_utilization = meter
        .f64_observable_gauge(PROCESS_CPU_UTILIZATION)
        .with_description("The percentage of CPU in use.")
        .init();
    let process_elapsed_time = meter
        .i64_observable_gauge(PROCESS_ELAPSED_TIME)
        .with_description("The uptime of the process in milliseconds.")
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

    let client_commands = meter
        .u64_observable_counter(CLIENT_COMMANDS)
        .with_description("Client Commands")
        .init();
    let client_connections = meter
        .u64_observable_counter(CLIENT_CONNECTIONS)
        .with_description("Client Connections")
        .init();
    let client_connection_attempts = meter
        .u64_observable_counter(CLIENT_CONNECTION_ATTEMPTS)
        .with_description("Client Connection Attempts")
        .init();
    let client_package_tx = meter
        .u64_observable_counter(CLIENT_PACKAGE_TX)
        .with_description("Client Package TX")
        .init();
    let client_package_rx = meter
        .u64_observable_counter(CLIENT_PACKAGE_RX)
        .with_description("Client Package RX")
        .init();




    let ofid = ofid.to_string();
    let common_attributes = [
        HOSTNAME.string(hostname::get().unwrap_or_default().into_string().unwrap()),
        OFID.string(ofid),
    ];

    let sys = Arc::new(Mutex::new(()));
    let previous_values = Arc::new(Mutex::new((0u64, 0u64, 0f64, Instant::now())));


    let stats_clone = Arc::clone(stats);
    let result = meter.register_callback(
        &[
            process_cpu_usage.as_any(),
            process_cpu_utilization.as_any(),
            process_elapsed_time.as_any(),
            process_memory_usage.as_any(),
            process_memory_virtual.as_any(),
            process_disk_io.as_any(),
            client_commands.as_any(),
            client_connections.as_any(),
            client_connection_attempts.as_any(),
            client_package_tx.as_any(),
            client_package_rx.as_any(),
        ],
        move |context| {
            let _sys = sys.lock().unwrap();
            let mut prev = previous_values.lock().unwrap();



            let io_stat = get_process_io_stats().unwrap_or_default();

            let read_bytes_diff = io_stat.read_bytes.saturating_sub(prev.0);
            let write_bytes_diff = io_stat.write_bytes.saturating_sub(prev.1);

            let elapsed_time = start_time.elapsed().unwrap_or_default().as_millis() as i64;
            let cpu_usage = process_stat.lock().unwrap().cpu().unwrap_or_default() * 100.0 as f64;
            let cpu_utilization = process_stat.lock().unwrap().cpu().unwrap_or_default() * 100.0 / core_count as f64;

            context.observe_f64(&process_cpu_usage, cpu_usage, &common_attributes);
            context.observe_f64(&process_cpu_utilization, cpu_utilization, &common_attributes);
            let mut physical_mem: i64 = 0;
            let mut virtual_mem: i64 = 0;
            if let Some(usage) = memory_stats() {
                physical_mem = usage.physical_mem as i64;
                virtual_mem = usage.virtual_mem as i64;
            }

            context.observe_i64(&process_memory_usage, physical_mem, &common_attributes);
            context.observe_i64(&process_memory_virtual, virtual_mem, &common_attributes);
            context.observe_i64(&process_elapsed_time, elapsed_time, &common_attributes);
            context.observe_i64(
                &process_disk_io,
                read_bytes_diff as i64,
                &[common_attributes.as_slice(), &[DIRECTION.string("read")]].concat(),
            );
            context.observe_i64(
                &process_disk_io,
                write_bytes_diff as i64,
                &[common_attributes.as_slice(), &[DIRECTION.string("write")]].concat(),
            );


            {
                let stats = stats_clone.lock().unwrap();
                if stats.connection_attempts > 0 {
                    context.observe_u64(&client_connection_attempts, stats.connection_attempts, &common_attributes);
                }
                if stats.connections > 0 {
                    context.observe_u64(&client_connections, stats.connections, &common_attributes);
                }
                if stats.package_tx > 0 {
                    context.observe_u64(&client_package_tx, stats.package_tx, &common_attributes);
                }
                if stats.package_rx > 0 {
                    context.observe_u64(&client_package_rx, stats.package_rx, &common_attributes);
                }
                
                if stats.signin > 0 {
                    context.observe_u64(&client_commands, stats.signin, 
                        &[common_attributes.as_slice(), &[COMMAND.string("signin")]].concat());
                }
                if stats.download > 0 {
                    context.observe_u64(&client_commands, stats.download, 
                        &[common_attributes.as_slice(), &[COMMAND.string("download")]].concat());
                }
                if stats.getdocumentversion > 0 {
                    context.observe_u64(&client_commands, stats.getdocumentversion, 
                        &[common_attributes.as_slice(), &[COMMAND.string("getdocumentversion")]].concat());
                }
                if stats.customcommand > 0 {
                    context.observe_u64(&client_commands, stats.customcommand, 
                        &[common_attributes.as_slice(), &[COMMAND.string("customcommand")]].concat());
                }
                if stats.listcollections > 0 {
                    context.observe_u64(&client_commands, stats.listcollections, 
                        &[common_attributes.as_slice(), &[COMMAND.string("listcollections")]].concat());
                }
                if stats.createcollection > 0 {
                    context.observe_u64(&client_commands, stats.createcollection, 
                        &[common_attributes.as_slice(), &[COMMAND.string("createcollection")]].concat());
                }
                if stats.dropcollection > 0 {
                    context.observe_u64(&client_commands, stats.dropcollection, 
                        &[common_attributes.as_slice(), &[COMMAND.string("dropcollection")]].concat());
                }
                if stats.ensurecustomer > 0 {
                    context.observe_u64(&client_commands, stats.ensurecustomer, 
                        &[common_attributes.as_slice(), &[COMMAND.string("ensurecustomer")]].concat());
                }
                if stats.invokeopenrpa > 0 {
                    context.observe_u64(&client_commands, stats.invokeopenrpa, 
                        &[common_attributes.as_slice(), &[COMMAND.string("invokeopenrpa")]].concat());
                }
                if stats.registerqueue > 0 {
                    context.observe_u64(&client_commands, stats.registerqueue, 
                        &[common_attributes.as_slice(), &[COMMAND.string("registerqueue")]].concat());
                }
                if stats.registerexchange > 0 {
                    context.observe_u64(&client_commands, stats.registerexchange, 
                        &[common_attributes.as_slice(), &[COMMAND.string("registerexchange")]].concat());
                }
                if stats.unregisterqueue > 0 {
                    context.observe_u64(&client_commands, stats.unregisterqueue, 
                        &[common_attributes.as_slice(), &[COMMAND.string("unregisterqueue")]].concat());
                }
                if stats.watch > 0 {
                    context.observe_u64(&client_commands, stats.watch, 
                        &[common_attributes.as_slice(), &[COMMAND.string("watch")]].concat());
                }
                if stats.unwatch > 0 {
                    context.observe_u64(&client_commands, stats.unwatch, 
                        &[common_attributes.as_slice(), &[COMMAND.string("unwatch")]].concat());
                }
                if stats.queuemessage > 0 {
                    context.observe_u64(&client_commands, stats.queuemessage, 
                        &[common_attributes.as_slice(), &[COMMAND.string("queuemessage")]].concat());
                }
                if stats.pushworkitem > 0 {
                    context.observe_u64(&client_commands, stats.pushworkitem, 
                        &[common_attributes.as_slice(), &[COMMAND.string("pushworkitem")]].concat());
                }
                if stats.pushworkitems > 0 {
                    context.observe_u64(&client_commands, stats.pushworkitems, 
                        &[common_attributes.as_slice(), &[COMMAND.string("pushworkitems")]].concat());
                }
                if stats.popworkitem > 0 {
                    context.observe_u64(&client_commands, stats.popworkitem, 
                        &[common_attributes.as_slice(), &[COMMAND.string("popworkitem")]].concat());
                }
                if stats.updateworkitem > 0 {
                    context.observe_u64(&client_commands, stats.updateworkitem, 
                        &[common_attributes.as_slice(), &[COMMAND.string("updateworkitem")]].concat());
                }
                if stats.deleteworkitem > 0 {
                    context.observe_u64(&client_commands, stats.deleteworkitem, 
                        &[common_attributes.as_slice(), &[COMMAND.string("deleteworkitem")]].concat());
                }
                if stats.addworkitemqueue > 0 {
                    context.observe_u64(&client_commands, stats.addworkitemqueue, 
                        &[common_attributes.as_slice(), &[COMMAND.string("addworkitemqueue")]].concat());
                }
                if stats.updateworkitemqueue > 0 {
                    context.observe_u64(&client_commands, stats.updateworkitemqueue, 
                        &[common_attributes.as_slice(), &[COMMAND.string("updateworkitemqueue")]].concat());
                }
                if stats.deleteworkitemqueue > 0 {
                    context.observe_u64(&client_commands, stats.deleteworkitemqueue, 
                        &[common_attributes.as_slice(), &[COMMAND.string("deleteworkitemqueue")]].concat());
                }
                if stats.getindexes > 0 {
                    context.observe_u64(&client_commands, stats.getindexes, 
                        &[common_attributes.as_slice(), &[COMMAND.string("getindexes")]].concat());
                }
                if stats.createindex > 0 {
                    context.observe_u64(&client_commands, stats.createindex, 
                        &[common_attributes.as_slice(), &[COMMAND.string("createindex")]].concat());
                }
                if stats.dropindex > 0 {
                    context.observe_u64(&client_commands, stats.dropindex, 
                        &[common_attributes.as_slice(), &[COMMAND.string("dropindex")]].concat());
                }
                if stats.upload > 0 {
                    context.observe_u64(&client_commands, stats.upload, 
                        &[common_attributes.as_slice(), &[COMMAND.string("upload")]].concat());
                }
                if stats.query > 0 {
                    context.observe_u64(&client_commands, stats.query, 
                        &[common_attributes.as_slice(), &[COMMAND.string("query")]].concat());
                }
                if stats.count > 0 {
                    context.observe_u64(&client_commands, stats.count, 
                        &[common_attributes.as_slice(), &[COMMAND.string("count")]].concat());
                }
                if stats.distinct > 0 {
                    context.observe_u64(&client_commands, stats.distinct, 
                        &[common_attributes.as_slice(), &[COMMAND.string("distinct")]].concat());
                }
                if stats.aggregate > 0 {
                    context.observe_u64(&client_commands, stats.aggregate, 
                        &[common_attributes.as_slice(), &[COMMAND.string("aggregate")]].concat());
                }
                if stats.insertone > 0 {
                    context.observe_u64(&client_commands, stats.insertone, 
                        &[common_attributes.as_slice(), &[COMMAND.string("insertone")]].concat());
                }
                if stats.insertmany > 0 {
                    context.observe_u64(&client_commands, stats.insertmany, 
                        &[common_attributes.as_slice(), &[COMMAND.string("insertmany")]].concat());
                }
                if stats.insertorupdateone > 0 {
                    context.observe_u64(&client_commands, stats.insertorupdateone, 
                        &[common_attributes.as_slice(), &[COMMAND.string("insertorupdateone")]].concat());
                }
                if stats.insertorupdatemany > 0 {
                    context.observe_u64(&client_commands, stats.insertorupdatemany, 
                        &[common_attributes.as_slice(), &[COMMAND.string("insertorupdatemany")]].concat());
                }
                if stats.updateone > 0 {
                    context.observe_u64(&client_commands, stats.updateone, 
                        &[common_attributes.as_slice(), &[COMMAND.string("updateone")]].concat());
                }
                if stats.updatedocument > 0 {
                    context.observe_u64(&client_commands, stats.updatedocument, 
                        &[common_attributes.as_slice(), &[COMMAND.string("updatedocument")]].concat());
                }
                if stats.deleteone > 0 {
                    context.observe_u64(&client_commands, stats.deleteone, 
                        &[common_attributes.as_slice(), &[COMMAND.string("deleteone")]].concat());
                }
                if stats.deletemany > 0 {
                    context.observe_u64(&client_commands, stats.deletemany, 
                        &[common_attributes.as_slice(), &[COMMAND.string("deletemany")]].concat());
                }
            }


            // Update previous values
            *prev = (io_stat.read_bytes, io_stat.write_bytes, cpu_usage, Instant::now());

            // println!("UPTIME: {}, CPU: {}, MEM: {}, VIRT: {}, READ: {}, WRITE: {}", elapsed_time,
            // cpu_usage, physical_mem, virtual_mem, read_bytes_diff, write_bytes_diff);
        },
    );

    match result {
        Ok(_) => {
            let _ = set_error_handler(Box::new(|_error: OtelError| {
                trace!("Error in OpenTelemetry: {:?}", _error);
            }));
            Ok(())
        },
        Err(e) => Err(format!("Could not register callback: {}", e)),
    }
}






use opentelemetry::{KeyValue};
use opentelemetry_otlp::{WithExportConfig};
use opentelemetry_sdk::{Resource};
use std::time::{Duration, Instant, SystemTime};
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
pub fn init_telemetry(agent: &str, strurl: &str, otlpurl: &str, stats: &Arc<std::sync::Mutex<ClientStatistics>>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
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
            .with_resource(Resource::new(vec![KeyValue::new("service.name", agent.to_string() )]))
            .with_period(Duration::from_secs(period))
            .build().unwrap();
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
                .with_resource(Resource::new(vec![KeyValue::new("service.name", agent.to_string() )]))
                .with_period(Duration::from_secs(period))
                .build().unwrap();

            let meter2 = provider.meter("process-meter2");
            // when not using global::set_meter_provider we need to keep it alive using ProivderWrapper
            match otel::register_metrics(meter2, &ofid, stats) {
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