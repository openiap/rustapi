use client::openiap::{
    AggregateRequest, CountRequest, DownloadRequest, Envelope, InsertOneRequest, QueryRequest,
    SigninRequest, UploadRequest, WatchRequest,
};
use std::ffi::CStr;
use std::os::raw::c_char;
use tokio::runtime::Runtime;
use tracing::debug;
pub mod client;
use client::Client;
use std::ffi::CString;

#[allow(dead_code)]
#[repr(C)]
pub struct ClientWrapper {
    success: bool,
    error: *const c_char,
    client: Option<Client>,
    runtime: std::sync::Arc<Runtime>,
}

// use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};
// use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn enable_tracing(rust_log: *const c_char, tracing: *const c_char) {
    // tracing_subscriber::registry()
    //     .with(fmt::layer())
    //     .with(EnvFilter::from_default_env())
    //     .init();
    let rust_log = unsafe { CStr::from_ptr(rust_log).to_str().unwrap() };
    let rust_log = rust_log.to_string();
    let mut filter = EnvFilter::from_default_env();
    if !rust_log.is_empty() {
        filter = EnvFilter::new(rust_log);
    }

    let mut subscriber = fmt::layer();
    let tracing = unsafe { CStr::from_ptr(tracing).to_str().unwrap() };
    let tracing = tracing.to_string();
    if !tracing.is_empty() {
        subscriber = match tracing.to_lowercase().as_str() {
            "new" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::NEW),
            "enter" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::ENTER),
            "exit" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::EXIT),
            "close" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE),
            "none" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::NONE),
            "active" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE),
            "full" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL),
            _ => subscriber,
        }
    }
    let subscriber = subscriber
        // .event_format(fmt::format::format().compact() )
        .and_then(filter)
        .with_subscriber(tracing_subscriber::registry())
        ;

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting global default subscriber failed");
    // EnvFilter::builder()
    // .with_default_directive(LevelFilter::ERROR.into())
    // .from_env_lossy();

    // use tracing_subscriber::{
    //     layer::{Layer, SubscriberExt},
    //     filter::{FilterFn
    //         // , LevelFilter
    //     },
    //     util::SubscriberInitExt,
    // };

    // let my_filter = FilterFn::new(|metadata| {
    //     // print ln!("metadata.target() = {:?}", metadata.target());
    //     // if metadata.target().starts_with("openiap") {
    //     //     print ln!("metadata.target() = {:?}", metadata);
    //     // }
    //     metadata.target().starts_with("openiap")
    // }); // .with_max_level_hint(LevelFilter::DEBUG);

    // let my_layer = tracing_subscriber::fmt::layer()
    //     .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL);

    // tracing_subscriber::registry()
    //     .with(my_layer.with_filter(my_filter))
    //     .init();
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn disable_tracing() {
    // tracing::dispatcher::get_default(|dispatch| {
    //     dispatch.unsubscribe()
    // });
}

type ConnectCallback = extern "C" fn(wrapper: *mut ClientWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_connect(server_address: *const c_char, callback: ConnectCallback) {
    debug!("rust::client_connect");
    let server_address = unsafe { CStr::from_ptr(server_address).to_str().unwrap().to_string() };
    let runtime = std::sync::Arc::new(Runtime::new().unwrap());

    debug!("rust::Spawn the async task");
    let runtime_clone = std::sync::Arc::clone(&runtime);
    runtime.spawn(async move {
        debug!("rust::Simulated async task started");
        // Simulated async task (or replace with actual Client::connect)
        let client_result = Client::connect(&server_address).await;
        debug!("rust::Client::connect::done");
        let wrapper = if let Ok(client) = client_result {
            Box::into_raw(Box::new(ClientWrapper {
                client: Some(client),
                runtime: runtime_clone,
                success: true,
                error: std::ptr::null(),
            }))
        } else {
            let e = client_result.err().unwrap();
            let error_msg = CString::new(format!("Connection failed: {:?}", e))
                .unwrap()
                .into_raw();
            Box::into_raw(Box::new(ClientWrapper {
                client: None,
                runtime: runtime_clone,
                success: false,
                error: error_msg,
            }))
        };

        debug!("rust::Client::Calling callback with result");
        callback(wrapper);
    });

    // Keep the main thread alive for a short time to ensure the async task completes
    std::thread::sleep(std::time::Duration::from_secs(2));
}

#[allow(dead_code)]
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_client(response: *mut ClientWrapper) {
    if response.is_null() {
        debug!("free_client: response is null");
        return;
    }
    unsafe {
        let response_ref: &ClientWrapper = &*response;
        if !response_ref.error.is_null() {
            let error_cstr = CStr::from_ptr(response_ref.error);
            if let Ok(error_str) = error_cstr.to_str() {
                debug!("free_client: error = {}", error_str);
            } else {
                debug!("free_client: error = <invalid UTF-8>");
            }
        }

        if let Some(client) = &response_ref.client {
            // let client_clone = client.clone();
            let runtime = &response_ref.runtime;

            // Ensure that the runtime properly shuts down after the block_on call
            runtime.block_on(async move {
                {
                    // let inner = client_clone.inner.lock().await;
                    let inner = client.inner.lock().await;
                    let mut queries = inner.queries.lock().await;

                    // Cancel pending requests
                    for (id, response_tx) in queries.drain() {
                        debug!("free_client: canceling request with id: {:?}", id);
                        let _ = response_tx.send(Envelope {
                            command: "cancelled".to_string(),
                            ..Default::default()
                        });
                    }

                    // debug!("free_client: released queries lock");
                } // Ensure locks are dropped before proceeding

                {
                    let inner = client.inner.lock().await;
                    let mut streams = inner.streams.lock().await;
                    let stream_keys = streams.keys().cloned().collect::<Vec<String>>();
                    stream_keys.iter().for_each(|k| {
                        debug!("free_client: client inner state: stream: {:?}", k);
                        streams.remove(k.clone().as_str());
                    });
                    // debug!("free_client: released streams lock");
                } // Ensure locks are dropped before proceeding
            });
        }
        // Free the client
        // let _client_wrapper: Box<ClientWrapper> = Box::from_raw(response);
        // debug!("free_client 5");
    }
    debug!("free_client::complete");
}

// #[no_mangle]
// #[tracing::instrument(skip_all)]
// pub extern "C" fn free_client(response: *mut ClientWrapper) {
//     if response.is_null() {
//         debug!("free_client: response is null");
//         return;
//     }
//     unsafe {
//         let response_ref: &ClientWrapper = &*response;
//         if !response_ref.error.is_null() {
//             let error_cstr = CStr::from_ptr(response_ref.error);
//             if let Ok(error_str) = error_cstr.to_str() {
//                 debug!("free_client: error = {}", error_str);
//             } else {
//                 debug!("free_client: error = <invalid UTF-8>");
//             }
//         }

//         if let Some(client) = &response_ref.client {
//             // let client_clone = client.clone();
//             let runtime = &response_ref.runtime;

//             // Ensure that the runtime properly shuts down after the block_on call
//             runtime.block_on(async move {
//                 {
//                     // let inner = client_clone.inner.lock().await;
//                     let inner = client.inner.lock().await;
//                     let mut queries = inner.queries.lock().await;

//                     // Cancel pending requests
//                     for (id, response_tx) in queries.drain() {
//                         debug!("free_client: canceling request with id: {:?}", id);
//                         let _ = response_tx.send(Envelope {
//                             command: "cancelled".to_string(),
//                             ..Default::default()
//                         });
//                     }

//                     // debug!("free_client: released queries lock");
//                 } // Ensure locks are dropped before proceeding

//                 {
//                     let inner = client.inner.lock().await;
//                     let mut streams = inner.streams.lock().await;
//                     let stream_keys = streams.keys().cloned().collect::<Vec<String>>();
//                     stream_keys.iter().for_each(|k| {
//                         debug!("free_client: client inner state: stream: {:?}", k);
//                         streams.remove(k.clone().as_str());
//                     });
//                     // debug!("free_client: released streams lock");
//                 } // Ensure locks are dropped before proceeding
//             });
//         }
//         // Free the client
//         // let _client_wrapper: Box<ClientWrapper> = Box::from_raw(response);
//         // debug!("free_client 5");
//     }
//     debug!("free_client::complete");
// }
#[repr(C)]
pub struct SigninRequestWrapper {
    username: *const c_char,
    password: *const c_char,
    jwt: *const c_char,
    agent: *const c_char,
    version: *const c_char,
    longtoken: bool,
    validateonly: bool,
    ping: bool,
}
#[repr(C)]
pub struct SigninResponseWrapper {
    success: bool,
    jwt: *const c_char,
    error: *const c_char,
}

type SigninCallback = extern "C" fn(wrapper: *mut SigninResponseWrapper);

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_signin(
    client: *mut ClientWrapper,
    options: *mut SigninRequestWrapper,
    callback: SigninCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;

    let request = SigninRequest {
        username: unsafe { CStr::from_ptr(options.username).to_str().unwrap() }.to_string(),
        password: unsafe { CStr::from_ptr(options.password).to_str().unwrap() }.to_string(),
        jwt: unsafe { CStr::from_ptr(options.jwt).to_str().unwrap() }.to_string(),
        agent: unsafe { CStr::from_ptr(options.agent).to_str().unwrap() }.to_string(),
        version: unsafe { CStr::from_ptr(options.version).to_str().unwrap() }.to_string(),
        longtoken: options.longtoken,
        ping: options.ping,
        validateonly: options.validateonly,
        ..Default::default()
    };

    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = SigninResponseWrapper {
            success: false,
            jwt: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();

    runtime.spawn(async move {
        // let result = client_clone.unwrap().signin(request).await;
        let result = client.as_ref().unwrap().signin(request).await;

        let response = match result {
            Ok(data) => {
                let jwt = CString::new(data.jwt).unwrap().into_raw();
                Box::new(SigninResponseWrapper {
                    success: true,
                    jwt,
                    error: std::ptr::null(),
                })
            }
            Err(e) => {
                let error_msg = CString::new(format!("Signin failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                Box::new(SigninResponseWrapper {
                    success: false,
                    jwt: std::ptr::null(),
                    error: error_msg,
                })
            }
        };

        callback(Box::into_raw(response));
    });

    // Keep the main thread alive for a short time to ensure the async task completes
    std::thread::sleep(std::time::Duration::from_secs(2));
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_signin_response(response: *mut SigninResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_set_callback(client: *mut Client, callback: extern "C" fn(*const c_char)) {
    let client = unsafe { &mut *client };
    client.set_callback(Box::new(move |event: String| {
        let c_event = std::ffi::CString::new(event).unwrap();
        callback(c_event.as_ptr());
    }));
}
#[repr(C)]
pub struct QueryRequestWrapper {
    collectionname: *const c_char,
    query: *const c_char,
    projection: *const c_char,
    orderby: *const c_char,
    queryas: *const c_char,
    explain: bool,
    skip: i32,
    top: i32,
}
#[repr(C)]
pub struct QueryResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
}
type QueryCallback = extern "C" fn(wrapper: *mut QueryResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_query(
    client: *mut ClientWrapper,
    options: *mut QueryRequestWrapper,
    callback: QueryCallback,
) {
    debug!("Rust: client_query");
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = QueryRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        query: unsafe { CStr::from_ptr(options.query).to_str().unwrap() }.to_string(),
        projection: unsafe { CStr::from_ptr(options.projection).to_str().unwrap() }.to_string(),
        orderby: unsafe { CStr::from_ptr(options.orderby).to_str().unwrap() }.to_string(),
        queryas: unsafe { CStr::from_ptr(options.queryas).to_str().unwrap() }.to_string(),
        explain: options.explain,
        skip: options.skip,
        top: options.top,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = QueryResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    debug!("Rust: runtime.spawn");
    runtime.spawn(async move {
        debug!("Rust: client.query");
        let result = client.as_ref().unwrap().query(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.query(request).await
        // });
        // let result = client_clone.unwrap().query(request).await;

        let response = match result {
            Ok(data) => {
                let results: *const c_char = CString::new(data.results).unwrap().into_raw();
                // std::mem::forget(results);
                // let results = std::mem::ManuallyDrop::new(CString::new(data.results).unwrap().into_raw());
                QueryResponseWrapper {
                    success: true,
                    results: results,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Query failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                QueryResponseWrapper {
                    success: false,
                    results: std::ptr::null(),
                    error: error_msg,
                }
            }
        };
        debug!("Rust: callback response");
        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_query_response(response: *mut QueryResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct AggregateRequestWrapper {
    collectionname: *const c_char,
    aggregates: *const c_char,
    queryas: *const c_char,
    hint: *const c_char,
    explain: bool,
}
#[repr(C)]
pub struct AggregateResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
}
type AggregateCallback = extern "C" fn(wrapper: *mut AggregateResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_aggregate(
    client: *mut ClientWrapper,
    options: *mut AggregateRequestWrapper,
    callback: AggregateCallback,
) {
    debug!("Rust: client_aggregate");
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = AggregateRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        aggregates: unsafe { CStr::from_ptr(options.aggregates).to_str().unwrap() }.to_string(),
        queryas: unsafe { CStr::from_ptr(options.queryas).to_str().unwrap() }.to_string(),
        hint: unsafe { CStr::from_ptr(options.hint).to_str().unwrap() }.to_string(),
        explain: options.explain,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = AggregateResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    debug!("Rust: runtime.spawn");
    runtime.spawn(async move {
        debug!("Rust: client.aggregate");
        let result = client.as_ref().unwrap().aggregate(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.aggregate(request).await
        // });
        // let result = client_clone.unwrap().aggregate(request).await;

        let response = match result {
            Ok(data) => {
                let results = CString::new(data.results).unwrap().into_raw();
                AggregateResponseWrapper {
                    success: true,
                    results,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Aggregate failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                AggregateResponseWrapper {
                    success: false,
                    results: std::ptr::null(),
                    error: error_msg,
                }
            }
        };
        debug!("Rust: callback response");
        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_aggregate_response(response: *mut AggregateResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct CountRequestWrapper {
    collectionname: *const c_char,
    query: *const c_char,
    queryas: *const c_char,
    explain: bool,
}
#[repr(C)]
pub struct CountResponseWrapper {
    success: bool,
    result: i32,
    error: *const c_char,
}
type CountCallback = extern "C" fn(wrapper: *mut CountResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_count(
    client: *mut ClientWrapper,
    options: *mut CountRequestWrapper,
    callback: CountCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = CountRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        query: unsafe { CStr::from_ptr(options.query).to_str().unwrap() }.to_string(),
        queryas: unsafe { CStr::from_ptr(options.queryas).to_str().unwrap() }.to_string(),
        explain: options.explain,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CountResponseWrapper {
            success: false,
            result: 0,
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().count(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.count(request).await
        // });
        // let result = client_clone.unwrap().count(request).await;

        let response = match result {
            Ok(data) => {
                let result = data.result;
                CountResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Count failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                CountResponseWrapper {
                    success: false,
                    result: 0,
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_count_response(response: *mut CountResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DistinctRequestWrapper {
    collectionname: *const c_char,
    field: *const c_char,
    query: *const c_char,
    queryas: *const c_char,
    explain: bool,
}
#[repr(C)]
pub struct DistinctResponseWrapper {
    success: bool,
    // results: *const c_char,
    results: *mut *const c_char,
    results_count: usize,
    error: *const c_char,
}
type DistinctCallback = extern "C" fn(wrapper: *mut DistinctResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_distinct(
    client: *mut ClientWrapper,
    options: *mut DistinctRequestWrapper,
    callback: DistinctCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = client::openiap::DistinctRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        field: unsafe { CStr::from_ptr(options.field).to_str().unwrap() }.to_string(),
        query: unsafe { CStr::from_ptr(options.query).to_str().unwrap() }.to_string(),
        queryas: unsafe { CStr::from_ptr(options.queryas).to_str().unwrap() }.to_string(),
        explain: options.explain,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DistinctResponseWrapper {
            success: false,
            results: std::ptr::null_mut(),
            results_count: 0,
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().distinct(request).await;
        let response = match result {
            Ok(data) => {
                let results_cstrings: Vec<CString> = data
                    .results
                    .iter()
                    .map(|s| CString::new(s.as_str()).unwrap())
                    .collect();
                let results_ptrs: Vec<*const c_char> =
                    results_cstrings.iter().map(|s| s.as_ptr()).collect();
                let results_array =
                    Box::into_raw(results_ptrs.clone().into_boxed_slice()) as *mut *const c_char;

                std::mem::forget(results_cstrings);

                debug!("Rust: results_array: {:?}", results_array);
                for (i, ptr) in results_ptrs.iter().enumerate() {
                    debug!("Rust: results_ptrs[{}]: {:?}: {:?}", i, ptr, unsafe {
                        CStr::from_ptr(*ptr).to_str().unwrap()
                    });
                }

                DistinctResponseWrapper {
                    success: true,
                    results: results_array,
                    results_count: data.results.len(),
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Distinct failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DistinctResponseWrapper {
                    success: false,
                    results: std::ptr::null_mut(),
                    results_count: 0,
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_distinct_response(response: *mut DistinctResponseWrapper) {
    unsafe {
        if !response.is_null() {
            let response = Box::from_raw(response);
            if !response.results.is_null() {
                for i in 0..response.results_count {
                    let c_str = *response.results.add(i);
                    if !c_str.is_null() {
                        _ = CString::from_raw(c_str as *mut c_char);
                    }
                }
                _ = Box::from_raw(response.results);
            }
            if !response.error.is_null() {
                _ = CString::from_raw(response.error as *mut c_char);
            }
        }
    }
}
// #[no_mangle]
// #[tracing::instrument(skip_all)]
// pub extern "C" fn free_distinct_response(response: *mut DistinctResponseWrapper) {
//     if response.is_null() {
//         return;
//     }
//     unsafe {
//         let _ = Box::from_raw(response);
//     }
// }

#[repr(C)]
pub struct InsertOneRequestWrapper {
    collectionname: *const c_char,
    item: *const c_char,
    w: i32,
    j: bool,
}
#[repr(C)]
pub struct InsertOneResponseWrapper {
    success: bool,
    result: *const c_char,
    error: *const c_char,
}
type InsertOneCallback = extern "C" fn(wrapper: *mut InsertOneResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_insert_one(
    client: *mut ClientWrapper,
    options: *mut InsertOneRequestWrapper,
    callback: InsertOneCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = InsertOneRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        item: unsafe { CStr::from_ptr(options.item).to_str().unwrap() }.to_string(),
        w: options.w,
        j: options.j,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().insert_one(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.insert_one(request).await
        // });
        // let result = client_clone.unwrap().insert_one(request).await;

        let response = match result {
            Ok(data) => {
                let result = CString::new(data.result).unwrap().into_raw();
                InsertOneResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("InsertOne failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                InsertOneResponseWrapper {
                    success: false,
                    result: std::ptr::null(),
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_insert_one_response(response: *mut InsertOneResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DownloadRequestWrapper {
    collectionname: *const c_char,
    id: *const c_char,
    folder: *const c_char,
    filename: *const c_char,
}
#[repr(C)]
pub struct DownloadResponseWrapper {
    success: bool,
    filename: *const c_char,
    error: *const c_char,
}
type DownloadCallback = extern "C" fn(wrapper: *mut DownloadResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_download(
    client: *mut ClientWrapper,
    options: *mut DownloadRequestWrapper,
    callback: DownloadCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let folder = unsafe { CStr::from_ptr(options.folder).to_str().unwrap() };
    let filename = unsafe { CStr::from_ptr(options.filename).to_str().unwrap() };
    let request = DownloadRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        filename: unsafe { CStr::from_ptr(options.filename).to_str().unwrap() }.to_string(),
        id: unsafe { CStr::from_ptr(options.id).to_str().unwrap() }.to_string(),
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DownloadResponseWrapper {
            success: false,
            filename: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();

    runtime.spawn(async move {
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.download(request).await
        // });
        // let result = client_clone
        //     .unwrap()
        //     .download(request, Some(folder), Some(filename))
        //     .await;
        let result = client
            .as_ref()
            .unwrap()
            .download(request, Some(folder), Some(filename))
            .await;

        let response = match result {
            Ok(data) => {
                let filename = CString::new(data.filename).unwrap().into_raw();
                DownloadResponseWrapper {
                    success: true,
                    filename,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Download failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DownloadResponseWrapper {
                    success: false,
                    filename: std::ptr::null(),
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_download_response(response: *mut DownloadResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct UploadRequestWrapper {
    filepath: *const c_char,
    filename: *const c_char,
    mimetype: *const c_char,
    metadata: *const c_char,
    collectionname: *const c_char,
}
#[repr(C)]
pub struct UploadResponseWrapper {
    success: bool,
    id: *const c_char,
    error: *const c_char,
}
type UploadCallback = extern "C" fn(wrapper: *mut UploadResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_upload(
    client: *mut ClientWrapper,
    options: *mut UploadRequestWrapper,
    callback: UploadCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let filepath = unsafe { CStr::from_ptr(options.filepath).to_str().unwrap() };
    if filepath.is_empty() {
        let error_msg = CString::new("Filepath is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let filepath = filepath.to_string();
    debug!("Rust::client_upload: filepath: {}", filepath);
    let filename = unsafe { CStr::from_ptr(options.filename).to_str().unwrap() };
    if filename.is_empty() {
        let error_msg = CString::new("Filename is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let request = UploadRequest {
        filename: filename.to_string(),
        mimetype: unsafe { CStr::from_ptr(options.mimetype).to_str().unwrap() }.to_string(),
        metadata: unsafe { CStr::from_ptr(options.metadata).to_str().unwrap() }.to_string(),
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();

    debug!("Rust::client_upload: runtime.spawn");
    runtime.spawn(async move {
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.upload(request).await
        // });
        debug!("Rust::client_upload: call client.upload");
        // let result = client_clone.unwrap().upload(request, &filepath).await;
        let result = client.as_ref().unwrap().upload(request, &filepath).await;

        debug!("Rust::client_upload: call client.upload done");
        let response = match result {
            Ok(data) => {
                let id = CString::new(data.id).unwrap().into_raw();
                UploadResponseWrapper {
                    success: true,
                    id,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Upload failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                UploadResponseWrapper {
                    success: false,
                    id: std::ptr::null(),
                    error: error_msg,
                }
            }
        };
        debug!("Rust::client_upload: call callback with response");
        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_upload_response(response: *mut UploadResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct WatchRequestWrapper {
    collectionname: *const c_char,
    paths: *const c_char,
}
#[repr(C)]
pub struct WatchResponseWrapper {
    success: bool,
    watchid: *const c_char,
    error: *const c_char,
}
type WatchCallback = extern "C" fn(wrapper: *mut WatchResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_watch(
    client: *mut ClientWrapper,
    options: *mut WatchRequestWrapper,
    callback: WatchCallback,
    event_callback: extern "C" fn(*const c_char),
) {
    debug!("Rust::client_watch");
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let paths = unsafe { CStr::from_ptr(options.paths).to_str().unwrap() };
    let paths = paths.split(",").map(|s| s.to_string()).collect();
    let request = WatchRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        paths: paths,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = WatchResponseWrapper {
            success: false,
            watchid: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();

    debug!("Rust::client_watch: runtime.spawn");
    runtime.spawn(async move {
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.watch(request).await
        // });
        // let result = client_clone.unwrap().watch(request).await;
        debug!("Rust::client_watch: call client.watch");
        // let result = client_clone.unwrap().watch(request,
        //     Box::new(move |event: client::openiap::WatchEvent| {
        //         // convert event to json
        //         let event = serde_json::to_string(&event).unwrap();
        //         let c_event = std::ffi::CString::new(event).unwrap();
        //         event_callback(c_event.as_ptr());
        //     })
        // ).await;
        let result = client
            .as_ref()
            .unwrap()
            .watch(
                request,
                Box::new(move |event: client::openiap::WatchEvent| {
                    // convert event to json
                    let event = serde_json::to_string(&event).unwrap();
                    let c_event = std::ffi::CString::new(event).unwrap();
                    event_callback(c_event.as_ptr());
                }),
            )
            .await;

        let response = match result {
            Ok(data) => {
                let watchid = CString::new(data).unwrap().into_raw();
                WatchResponseWrapper {
                    success: true,
                    watchid,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Watch failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                WatchResponseWrapper {
                    success: false,
                    watchid: std::ptr::null(),
                    error: error_msg,
                }
            }
        };

        debug!("Rust::client_watch: call callback with response");
        callback(Box::into_raw(Box::new(response)));
    });
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_watch_response(response: *mut WatchResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}
#[repr(C)]
pub struct UnWatchResponseWrapper {
    success: bool,
    error: *const c_char,
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_unwatch(
    client: *mut ClientWrapper,
    watchid: *const c_char,
) -> *mut UnWatchResponseWrapper {
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let watchid = unsafe { CStr::from_ptr(watchid).to_str().unwrap() };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UnWatchResponseWrapper {
            success: false,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.unwatch(watchid).await
    });
    match result {
        Ok(_) => {
            let response = UnWatchResponseWrapper {
                success: true,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Unwatch failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = UnWatchResponseWrapper {
                success: false,
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    }
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_unwatch_response(response: *mut UnWatchResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}
