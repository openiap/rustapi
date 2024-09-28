use sysinfo::NetworkExt;
use sysinfo::ProcessExt;
use sysinfo::SystemExt;
use sysinfo::{get_current_pid, System};
use opentelemetry::metrics::Meter;
use opentelemetry::Key;
use opentelemetry::global::{set_error_handler, Error as OtelError};
use std::sync::{Arc, Mutex};
use tracing::{trace};

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
#[tracing::instrument(skip_all)]
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
            HOSTNAME.string(hostname::get().unwrap().into_string().unwrap()),
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
            let mut sys = sys.lock().unwrap();
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
                let pmemory: i64 = process.memory().try_into().unwrap();
                let vmemory: i64 = process.virtual_memory().try_into().unwrap();
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
                let elapsed_mili: i64 = (elapsed_seconds * 1000).try_into().unwrap();
                context.observe_i64(
                    &process_elapsed_time,
                    elapsed_mili,
                    &common_attributes,
                );

                context.observe_i64(
                    &process_disk_io,
                    disk_io.read_bytes.try_into().unwrap(),
                    &[common_attributes.as_slice(), &[DIRECTION.string("read")]].concat(),
                );
                context.observe_i64(
                    &process_disk_io,
                    disk_io.written_bytes.try_into().unwrap(),
                    &[common_attributes.as_slice(), &[DIRECTION.string("write")]].concat(),
                );
                trace!(
                    "hostname: {:?}, mem: {:?} v mem: {:?} cpu usage: {:?}  cpu util: {:?} elapsed: {:?} rx: {:?} tx: {:?}",
                    hostname::get().unwrap().into_string().unwrap(),
                    pmemory, vmemory, cpu_usage, cpu_utilization, elapsed_seconds, received, transmitted
                );
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
