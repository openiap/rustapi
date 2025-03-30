#[allow(unused_imports)]
use std::{thread, time::Duration};

use libc;

use crate::*;

// cargo test --package openiap-clib --lib -- tests::test_pop_workitem_async_memory_leak --nocapture
// heaptrack cargo +nightly test --package openiap-clib --lib -- tests::test_pop_workitem_async_memory_leak --nocapture
// ls -l target/debug/deps/openiap_clib* | grep -E '^-rwx'
// heaptrack openiap/target/debug/deps/openiap_clib-c7e7ca49a4cce355 --exact tests::test_pop_workitem_async_memory_leak --nocapture
#[allow(dead_code)]
extern "C" fn callback(response: *mut PopWorkitemResponseWrapper) {
    free_pop_workitem_response(response); // Free the response after callback is invoked
}
#[allow(dead_code)]
fn get_memory_usage(pid: libc::pid_t) -> u64 {
    // Accessing memory usage from /proc/{pid}/status file on Linux
    let status_file = format!("/proc/{}/status", pid);
    if let Ok(status) = std::fs::read_to_string(status_file) {
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                if let Some(value_str) = line.split_whitespace().nth(1) {
                    if let Ok(value) = value_str.parse::<u64>() {
                        return value; // VmRSS in KB
                    }
                }
            }
        }
    }
    0
}

#[test]
#[allow(unreachable_code)]
fn test_pop_workitem_async_memory_leak() {
    // Setup client and request
    let client = create_client();

    client_connect(client, CString::new("").unwrap().into_raw());
    
    let options = Box::into_raw(Box::new(PopWorkitemRequestWrapper {
        wiq: CString::new("q2").unwrap().into_raw(),
        wiqid: CString::new("67deb12092d595da3fcdc89d").unwrap().into_raw(),
        request_id: 0
    }));

    let pid: i32 = std::process::id() as libc::pid_t;
    let initial_memory = get_memory_usage(pid);
    let mut last = initial_memory;

    println!("initial memory usage: {} KB", initial_memory);

    for i in 0..500 { // 1_000_000
        // Call `pop_workitem_async`
        let downloadfolder = CString::new(".").unwrap();
        pop_workitem_async(client, options, downloadfolder.as_ptr(), callback);

        // Allow some time for async work to complete
        thread::sleep(Duration::from_millis(10));

        // Check memory usage periodically
        if i % 500 == 0  {
            let current_memory = get_memory_usage(pid);
            if last != current_memory {
                println!("Iteration {}: Memory Usage = {} KB", i, current_memory);
                last = current_memory;
            }

            // Basic heuristic to check for potential leaks
            if current_memory > initial_memory + 10_000 { // e.g., if usage grows by more than 10 MB
                panic!("Potential memory leak detected!");
            }
        }
    }

    // Cleanup
    #[allow(unused_must_use)]
    unsafe {        
        CString::from_raw((*options).wiq as *mut c_char); // Free the allocated strings
        CString::from_raw((*options).wiqid as *mut c_char);
        Box::from_raw(options); // Free the options struct
        Box::from_raw(client);  // Free the client struct
    }
}
