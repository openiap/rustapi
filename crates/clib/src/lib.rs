use openiap_client::protos::{
    AggregateRequest, CountRequest, DistinctRequest, DownloadRequest, Envelope, InsertOneRequest,
    QueryRequest, SigninRequest, UploadRequest, WatchEvent, WatchRequest,
};
use openiap_client::{Client, DeleteManyRequest, DeleteOneRequest, InsertManyRequest, InsertOrUpdateOneRequest, PopWorkitemRequest, PushWorkitemRequest, QueueEvent, RegisterExchangeRequest, RegisterQueueRequest, Timestamp, UpdateOneRequest, WorkitemFile};
// use tracing_subscriber::field::debug;
use std::collections::{HashMap, VecDeque};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;
use std::vec;
use tokio::runtime::Runtime;
use tracing::{ debug, info, trace};

use lazy_static::lazy_static;
lazy_static! {
    static ref WATCH_EVENTS: std::sync::Mutex<HashMap<String, VecDeque<WatchEvent>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    static ref QUEUE_EVENTS: std::sync::Mutex<HashMap<String, VecDeque<QueueEvent>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

}

#[allow(dead_code)]
#[repr(C)]
pub struct ClientWrapper {
    success: bool,
    error: *const c_char,
    client: Option<Client>,
    runtime: std::sync::Arc<Runtime>,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WatchEventWrapper {
    id: *const c_char,
    operation: *const c_char,
    document: *const c_char,
}
impl Default for WatchEventWrapper {
    fn default() -> Self { 
        WatchEventWrapper {
            id: std::ptr::null(),
            operation: std::ptr::null(),
            document: std::ptr::null()
        }
     }
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn query(
    client: *mut ClientWrapper,
    options: *mut QueryRequestWrapper,
) -> *mut QueryResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().query(request).await;
        client.as_ref().unwrap().query(request).await
    });

    let response = match result {
        Ok(data) => {
            let results: *const c_char = CString::new(data.results).unwrap().into_raw();
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

    Box::into_raw(Box::new(response))
}

type QueryCallback = extern "C" fn(wrapper: *mut QueryResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn query_async(
    client: *mut ClientWrapper,
    options: *mut QueryRequestWrapper,
    callback: QueryCallback,
) {
    debug!("Rust: query_async");
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request:QueryRequest;
    unsafe { 
        let collectionname = CStr::from_ptr(options.collectionname).to_str().unwrap();
        let query = CStr::from_ptr(options.query).to_str().unwrap();
        let projection = CStr::from_ptr(options.projection).to_str().unwrap();
        let orderby = CStr::from_ptr(options.orderby).to_str().unwrap();
        let queryas = CStr::from_ptr(options.queryas).to_str().unwrap();
        let explain = options.explain;
        let skip = options.skip;
        let top = options.top;
        debug!("Rust: query_async: collectionname: {}, query: {}, projection: {}, orderby: {}, queryas: {}, explain: {}, skip: {}, top: {}", collectionname, query, projection, orderby, queryas, explain, skip, top);

        request = QueryRequest {
            collectionname: collectionname.to_string(),
            query: query.to_string(),
            projection: projection.to_string(),
            orderby: orderby.to_string(),
            queryas: queryas.to_string(),
            explain: explain,
            skip: skip,
            top: top,
            ..Default::default()
        };    
    }
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
        filter = EnvFilter::new(rust_log.clone());
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
            "active" => {
                subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
            }
            "full" => subscriber.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL),
            _ => subscriber,
        }
    }
    let subscriber = subscriber
        // .event_format(fmt::format::format().compact() )
        .and_then(filter)
        .with_subscriber(tracing_subscriber::registry());

    match tracing::subscriber::set_global_default(subscriber) {
        Ok(()) => {
            debug!("Tracing enabled");
        }
        Err(e) => {
            eprintln!("Tracing failed: {:?}", e);
        }
    }
    info!("enable_tracing rust_log: {:?}, tracing: {:?}", rust_log, tracing);

    // .expect("setting global default subscriber failed");
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

#[no_mangle]
pub extern "C" fn client_connect(server_address: *const c_char) -> *mut ClientWrapper {
    let server_address = unsafe { CStr::from_ptr(server_address).to_str().unwrap() };
    let runtime = std::sync::Arc::new(Runtime::new().unwrap());
    info!("server_address = {:?}", server_address);
    info!("connect::begin");
    let client = runtime.block_on(Client::connect(server_address));
    info!("connect::complete");
    if client.is_err() == true {
        let e = client.err().unwrap();
        info!("error_msg = {:?}", format!("Connaction failed: {:?}", e));
        let error_msg = CString::new(format!("Connaction failed: {:?}", e))
            .unwrap()
            .into_raw();
        
        let result = Box::into_raw(Box::new(ClientWrapper {
            client: None,
            runtime,
            success: false,
            error: error_msg,
        }));
        info!("connect::complete error result address: {:?}", result);
        return result;
    }
    let result = Box::into_raw(Box::new(ClientWrapper {
        client: Some(client.unwrap()),
        runtime,
        success: true,
        error: std::ptr::null(),
    }));
    info!("connect::complete result address: {:?}", result);
    result
}

type ConnectCallback = extern "C" fn(wrapper: *mut ClientWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn connect_async(server_address: *const c_char, callback: ConnectCallback) {
    debug!("rust::connect_async");
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

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn signin(
    client: *mut ClientWrapper,
    options: *mut SigninRequestWrapper,
) -> *mut SigninResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().signin(request).await;
        client.as_ref().unwrap().signin(request).await
    });

    let response = match result {
        Ok(data) => {
            let jwt = CString::new(data.jwt).unwrap().into_raw();
            SigninResponseWrapper {
                success: true,
                jwt,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Signin failed: {:?}", e))
                .unwrap()
                .into_raw();
            SigninResponseWrapper {
                success: false,
                jwt: std::ptr::null(),
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}

type SigninCallback = extern "C" fn(wrapper: *mut SigninResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn signin_async(
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

#[repr(C)]
pub struct AggregateResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn aggregate(
    client: *mut ClientWrapper,
    options: *mut AggregateRequestWrapper,
) -> *mut AggregateResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().aggregate(request).await;
        client.as_ref().unwrap().aggregate(request).await
    });

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

    Box::into_raw(Box::new(response))
}

type AggregateCallback = extern "C" fn(wrapper: *mut AggregateResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn aggregate_async(
    client: *mut ClientWrapper,
    options: *mut AggregateRequestWrapper,
    callback: AggregateCallback,
) {
    debug!("Rust: aggregate_async");
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn count(
    client: *mut ClientWrapper,
    options: *mut CountRequestWrapper,
) -> *mut CountResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().count(request).await;
        client.as_ref().unwrap().count(request).await
    });

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

    Box::into_raw(Box::new(response))
}

type CountCallback = extern "C" fn(wrapper: *mut CountResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn count_async(
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn distinct(
    client: *mut ClientWrapper,
    options: *mut DistinctRequestWrapper,
) -> *mut DistinctResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = DistinctRequest {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().distinct(request).await;
        client.as_ref().unwrap().distinct(request).await
    });

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

    Box::into_raw(Box::new(response))
}

type DistinctCallback = extern "C" fn(wrapper: *mut DistinctResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn distinct_async(
    client: *mut ClientWrapper,
    options: *mut DistinctRequestWrapper,
    callback: DistinctCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = DistinctRequest {
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_one(
    client: *mut ClientWrapper,
    options: *mut InsertOneRequestWrapper,
) -> *mut InsertOneResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().insert_one(request).await;
        client.as_ref().unwrap().insert_one(request).await
    });

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

    Box::into_raw(Box::new(response))
}

type InsertOneCallback = extern "C" fn(wrapper: *mut InsertOneResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_one_async(
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
pub struct InsertManyRequestWrapper {
    pub collectionname: *const c_char,
    pub items: *const c_char,
    pub w: i32,
    pub j: bool,
    pub skipresults: bool,
}
#[repr(C)]
pub struct InsertManyResponseWrapper {
    pub success: bool,
    pub results: *const c_char,
    pub error: String,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_many(
    client: *mut ClientWrapper,
    options: *mut InsertManyRequestWrapper,
) -> *mut InsertManyResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = InsertManyRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        items: unsafe { CStr::from_ptr(options.items).to_str().unwrap() }.to_string(),
        w: options.w,
        j: options.j,
        skipresults: options.skipresults,
        ..Default::default()
    };
    if client.is_none() {
        let response = InsertManyResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: "Client is not connected".to_string(),
        };
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().insert_many(request).await;
        client.as_ref().unwrap().insert_many(request).await
    });

    let response = match result {
        Ok(data) => {
            let results = CString::new(data.results).unwrap().into_raw();
            InsertManyResponseWrapper {
                success: true,
                results,
                error: "".to_string(),
            }
        }
        Err(e) => {
            let error_msg = format!("InsertMany failed: {:?}", e);
            InsertManyResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}

type InsertManyCallback = extern "C" fn(wrapper: *mut InsertManyResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_many_async(
    client: *mut ClientWrapper,
    options: *mut InsertManyRequestWrapper,
    callback: InsertManyCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = InsertManyRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        items: unsafe { CStr::from_ptr(options.items).to_str().unwrap() }.to_string(),
        w: options.w,
        j: options.j,
        skipresults: options.skipresults,
        ..Default::default()
    };
    if client.is_none() {
        let response = InsertManyResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: "Client is not connected".to_string(),
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().insert_many(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.insert_many(request).await
        // });
        // let result = client_clone.unwrap().insert_many(request).await;

        let response = match result {
            Ok(data) => {
                let results = CString::new(data.results).unwrap().into_raw();
                InsertManyResponseWrapper {
                    success: true,
                    results,
                    error: "".to_string(),
                }
            }
            Err(e) => {
                let error_msg = format!("InsertMany failed: {:?}", e);
                InsertManyResponseWrapper {
                    success: false,
                    results: std::ptr::null(),
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_insert_many_response(response: *mut InsertManyResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct UpdateOneRequestWrapper {
    collectionname: *const c_char,
    item: *const c_char,
    w: i32,
    j: bool,
}
#[repr(C)]
pub struct UpdateOneResponseWrapper {
    success: bool,
    result: *const c_char,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn update_one(
    client: *mut ClientWrapper,
    options: *mut UpdateOneRequestWrapper,
) -> *mut UpdateOneResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = UpdateOneRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        item: unsafe { CStr::from_ptr(options.item).to_str().unwrap() }.to_string(),
        w: options.w,
        j: options.j,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().update_one(request).await;
        client.as_ref().unwrap().update_one(request).await
    });

    let response = match result {
        Ok(data) => {
            let result = CString::new(data.result).unwrap().into_raw();
            UpdateOneResponseWrapper {
                success: true,
                result,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("UpdateOne failed: {:?}", e))
                .unwrap()
                .into_raw();
            UpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}

type UpdateOneCallback = extern "C" fn(wrapper: *mut UpdateOneResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn update_one_async(
    client: *mut ClientWrapper,
    options: *mut UpdateOneRequestWrapper,
    callback: UpdateOneCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = UpdateOneRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        item: unsafe { CStr::from_ptr(options.item).to_str().unwrap() }.to_string(),
        w: options.w,
        j: options.j,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().update_one(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.update_one(request).await
        // });
        // let result = client_clone.unwrap().update_one(request).await;

        let response = match result {
            Ok(data) => {
                let result = CString::new(data.result).unwrap().into_raw();
                UpdateOneResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("UpdateOne failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                UpdateOneResponseWrapper {
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
pub extern "C" fn free_update_one_response(response: *mut UpdateOneResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct InsertOrUpdateOneRequestWrapper {
    collectionname: *const c_char,
    uniqeness: *const c_char, 
    item: *const c_char,
    w: i32,
    j: bool,
}
#[repr(C)]
pub struct InsertOrUpdateOneResponseWrapper {
    success: bool,
    result: *const c_char,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_or_update_one(
    client: *mut ClientWrapper,
    options: *mut InsertOrUpdateOneRequestWrapper,
) -> *mut InsertOrUpdateOneResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    debug!("Rust: insert_or_update_one create request");

    trace!("Rust: parse collectionname");
    let collectionname = unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }.to_string();
    trace!("Rust: parse uniqeness");
    let uniqeness = unsafe { CStr::from_ptr(options.uniqeness).to_str().unwrap() }.to_string();
    trace!("Rust: parse item");
    let item = unsafe { CStr::from_ptr(options.item).to_str().unwrap() }.to_string();
    trace!("Rust: parse w");
    let w = options.w;
    trace!("Rust: parse j");
    let j = options.j;
    let request = InsertOrUpdateOneRequest {
        collectionname,
        uniqeness,
        item,
        w,
        j,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOrUpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    debug!("Rust: run insert_or_update_one in runtime");
    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().insert_or_update_one(request).await;
        client.as_ref().unwrap().insert_or_update_one(request).await
    });

    let response = match result {
        Ok(data) => {
            let result = CString::new(data).unwrap().into_raw();
            InsertOrUpdateOneResponseWrapper {
                success: true,
                result,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("InsertOrUpdateOne failed: {:?}", e))
                .unwrap()
                .into_raw();
            InsertOrUpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}

type InsertOrUpdateOneCallback = extern "C" fn(wrapper: *mut InsertOrUpdateOneResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_or_update_one_async(
    client: *mut ClientWrapper,
    options: *mut InsertOrUpdateOneRequestWrapper,
    callback: InsertOrUpdateOneCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    debug!("Rust: insert_or_update_one_async create request");
    let request = InsertOrUpdateOneRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        uniqeness: unsafe { CStr::from_ptr(options.uniqeness).to_str().unwrap() }.to_string(),
        item: unsafe { CStr::from_ptr(options.item).to_str().unwrap() }.to_string(),
        w: options.w,
        j: options.j,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOrUpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().insert_or_update_one(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.insert_or_update_one(request).await
        // });
        // let result = client_clone.unwrap().insert_or_update_one(request).await;

        let response = match result {
            Ok(data) => {
                let result = CString::new(data).unwrap().into_raw();
                InsertOrUpdateOneResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("InsertOrUpdateOne failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                InsertOrUpdateOneResponseWrapper {
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
pub extern "C" fn free_insert_or_update_one_response(response: *mut InsertOrUpdateOneResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DeleteOneRequestWrapper {
    collectionname: *const c_char,
    id: *const c_char,
    recursive: bool,
}
#[repr(C)]
pub struct DeleteOneResponseWrapper {
    success: bool,
    affectedrows: i32,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_one(
    client: *mut ClientWrapper,
    options: *mut DeleteOneRequestWrapper,
) -> *mut DeleteOneResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = DeleteOneRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        id: unsafe { CStr::from_ptr(options.id).to_str().unwrap() }.to_string(),
        recursive: options.recursive,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteOneResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().delete_one(request).await;
        client.as_ref().unwrap().delete_one(request).await
    });

    let response = match result {
        Ok(data) => {
            let affectedrows = data;
            DeleteOneResponseWrapper {
                success: true,
                affectedrows,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("DeleteOne failed: {:?}", e))
                .unwrap()
                .into_raw();
            DeleteOneResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type DeleteOneCallback = extern "C" fn(wrapper: *mut DeleteOneResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_one_async(
    client: *mut ClientWrapper,
    options: *mut DeleteOneRequestWrapper,
    callback: DeleteOneCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = DeleteOneRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        id: unsafe { CStr::from_ptr(options.id).to_str().unwrap() }.to_string(),
        recursive: options.recursive,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteOneResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().delete_one(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.delete_one(request).await
        // });
        // let result = client_clone.unwrap().delete_one(request).await;

        let response = match result {
            Ok(data) => {
                let affectedrows = data;
                DeleteOneResponseWrapper {
                    success: true,
                    affectedrows,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("DeleteOne failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DeleteOneResponseWrapper {
                    success: false,
                    affectedrows: 0,
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_delete_one_response(response: *mut DeleteOneResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DeleteManyRequestWrapper {
    collectionname: *const c_char,
    query: *const c_char,
    recursive: bool,
    ids: *const *const c_char,
}
#[repr(C)]
pub struct DeleteManyResponseWrapper {
    success: bool,
    affectedrows: i32,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_many(
    client: *mut ClientWrapper,
    options: *mut DeleteManyRequestWrapper,
) -> *mut DeleteManyResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = DeleteManyRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        query: unsafe { CStr::from_ptr(options.query).to_str().unwrap() }.to_string(),
        recursive: options.recursive,
        ids: {
            let mut ids = vec![];
            let mut i = 0;
            loop {
                let id = unsafe { *options.ids.add(i) };
                if id.is_null() {
                    break;
                }
                ids.push(unsafe { CStr::from_ptr(id).to_str().unwrap() }.to_string());
                i += 1;
            }
            ids
        },
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteManyResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    let result = runtime.block_on(async {
        // let result = client_clone.unwrap().delete_many(request).await;
        client.as_ref().unwrap().delete_many(request).await
    });

    let response = match result {
        Ok(data) => {
            let affectedrows = data;
            DeleteManyResponseWrapper {
                success: true,
                affectedrows,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("DeleteMany failed: {:?}", e))
                .unwrap()
                .into_raw();
            DeleteManyResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type DeleteManyCallback = extern "C" fn(wrapper: *mut DeleteManyResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_many_async(
    client: *mut ClientWrapper,
    options: *mut DeleteManyRequestWrapper,
    callback: DeleteManyCallback,
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = DeleteManyRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }
            .to_string(),
        query: unsafe { CStr::from_ptr(options.query).to_str().unwrap() }.to_string(),
        recursive: options.recursive,
        ids: {
            let mut ids = vec![];
            let mut i = 0;
            loop {
                let id = unsafe { *options.ids.add(i) };
                if id.is_null() {
                    break;
                }
                ids.push(unsafe { CStr::from_ptr(id).to_str().unwrap() }.to_string());
                i += 1;
            }
            ids
        },
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteManyResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    // let client_clone = client.clone();
    // let runtime_clone = std::sync::Arc::clone(&runtime);

    runtime.spawn(async move {
        let result = client.as_ref().unwrap().delete_many(request).await;
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.delete_many(request).await
        // });
        // let result = client_clone.unwrap().delete_many(request).await;

        let response = match result {
            Ok(data) => {
                let affectedrows = data;
                DeleteManyResponseWrapper {
                    success: true,
                    affectedrows,
                    error: std::ptr::null(),
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("DeleteMany failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DeleteManyResponseWrapper {
                    success: false,
                    affectedrows: 0,
                    error: error_msg,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_delete_many_response(response: *mut DeleteManyResponseWrapper) {
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn download(
    client: *mut ClientWrapper,
    options: *mut DownloadRequestWrapper,
) -> *mut DownloadResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();

    let result = runtime.block_on(async {
        // let c = client.as_ref().unwrap();
        // c.download(request).await
        client
            .as_ref()
            .unwrap()
            .download(request, Some(folder), Some(filename))
            .await
    });

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

    Box::into_raw(Box::new(response))
}

type DownloadCallback = extern "C" fn(wrapper: *mut DownloadResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn download_async(
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn upload(
    client: *mut ClientWrapper,
    options: *mut UploadRequestWrapper,
) -> *mut UploadResponseWrapper {
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
        return Box::into_raw(Box::new(response));
    }
    let filepath = filepath.to_string();
    debug!("Rust::upload: filepath: {}", filepath);
    let filename = unsafe { CStr::from_ptr(options.filename).to_str().unwrap() };
    if filename.is_empty() {
        let error_msg = CString::new("Filename is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
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
        return Box::into_raw(Box::new(response));
    }

    // let client_clone = client.clone();

    debug!("Rust::upload: runtime.block_on");
    let result = runtime.block_on(async {
        // let c = client.as_ref().unwrap();
        // c.upload(request).await
        client.as_ref().unwrap().upload(request, &filepath).await
    });

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
    Box::into_raw(Box::new(response))
}

type UploadCallback = extern "C" fn(wrapper: *mut UploadResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn upload_async(
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
    debug!("Rust::upload_async: filepath: {}", filepath);
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

    debug!("Rust::upload_async: runtime.spawn");
    runtime.spawn(async move {
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.upload(request).await
        // });
        debug!("Rust::upload_async: call client.upload");
        // let result = client_clone.unwrap().upload(request, &filepath).await;
        let result = client.as_ref().unwrap().upload(request, &filepath).await;

        debug!("Rust::upload_async: call client.upload done");
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
        debug!("Rust::upload_async: call callback with response");
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
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn watch(
    client: *mut ClientWrapper,
    options: *mut WatchRequestWrapper,
) -> *mut WatchResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    // let events = &client_wrapper.events;
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
        return Box::into_raw(Box::new(response));
    }
    let result = runtime.block_on(async {
        client
            .as_ref()
            .unwrap()
            .watch(
                request,
                Box::new(move |event: WatchEvent| {
                    // convert event to json
                    // let event = serde_json::to_string(&event).unwrap();
                    // let c_event = std::ffi::CString::new(event).unwrap();
                    debug!("Rust::watch: event: {:?}", event);
                    let watchid = CString::new(event.id.clone())
                        .unwrap()
                        .into_string()
                        .unwrap();
                    let mut e = WATCH_EVENTS.lock().unwrap();
                    let queue = e.get_mut(&watchid);
                    match queue {
                        Some(q) => {
                            q.push_back(event);
                        }
                        None => {
                            let mut q = std::collections::VecDeque::new();
                            q.push_back(event);
                            e.insert(watchid, q);
                        }
                    }
                }),
            )
            .await
    });

    let response = match result {
        Ok(data) => {
            let id = String::from(&data);
            let mut events = WATCH_EVENTS.lock().unwrap();
            let queue = events.get_mut(&id);
            if queue.is_none() {
                let q = std::collections::VecDeque::new();
                let k = String::from(&data);
                events.insert(k, q);
            }
            let watchid = CString::new(id).unwrap().into_raw();
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

    Box::into_raw(Box::new(response))
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn next_watch_event (
    watchid: *const c_char,
) -> *mut WatchEventWrapper {
    debug!("unwrap watchid");
    let watchid = unsafe { CStr::from_ptr(watchid).to_str().unwrap() };
    debug!("watchid {:}", watchid);
    let watchid = watchid.to_string();
    debug!("unwrap events");
    let mut e = WATCH_EVENTS.lock().unwrap();
    debug!("get queue");
    let queue = e.get_mut(&watchid);
    match queue {
        Some(q) => {
            match q.pop_front() {
                Some(event) => {
                    debug!("got event");
                    let id = CString::new(event.id).unwrap().into_raw();
                    let operation = CString::new(event.operation).unwrap().into_raw();
                    let document = CString::new(event.document).unwrap().into_raw();
                    let event = Box::new(WatchEventWrapper {
                        id,
                        operation,
                        document
                    });
                    Box::into_raw(event)
                }
                None => {
                    debug!("No event");
                    Box::into_raw(Box::new(WatchEventWrapper::default())) 
                },
            }
        },
        None => {
            debug!("Queue for {:} not found", watchid);
            Box::into_raw(Box::new(WatchEventWrapper::default())) 
        },
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_watch_event(response: *mut WatchEventWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

type WatchCallback = extern "C" fn(wrapper: *mut WatchResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn watch_async(
    client: *mut ClientWrapper,
    options: *mut WatchRequestWrapper,
    callback: WatchCallback,
    event_callback: extern "C" fn(*const c_char),
) {
    debug!("Rust::watch_async");
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

    debug!("Rust::watch_async: runtime.spawn");
    runtime.spawn(async move {
        // let result = runtime.block_on(async {
        //     let c = client.as_ref().unwrap();
        //     c.watch(request).await
        // });
        // let result = client_clone.unwrap().watch(request).await;
        debug!("Rust::watch_async: call client.watch");
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
                Box::new(move |event: WatchEvent| {
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

        debug!("Rust::watch_async: call callback with response");
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
pub extern "C" fn unwatch(
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


#[repr(C)]
pub struct RegisterQueueRequestWrapper {
    queuename: *const c_char,
}
#[repr(C)]
pub struct RegisterQueueResponseWrapper {
    success: bool,
    queuename: *const c_char,
    error: *const c_char,
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn register_queue(
    client: *mut ClientWrapper,
    options: *mut RegisterQueueRequestWrapper,
) -> *mut RegisterQueueResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    // let events = &client_wrapper.events;
    let request = RegisterQueueRequest {
        queuename: unsafe { CStr::from_ptr(options.queuename).to_str().unwrap() }
        .to_string(),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = RegisterQueueResponseWrapper {
            success: false,
            queuename: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }
    let result = runtime.block_on(async {
        client
            .as_ref()
            .unwrap()
            .register_queue(
                request,
                Box::new(move |event: QueueEvent| {
                    // convert event to json
                    // let event = serde_json::to_string(&event).unwrap();
                    // let c_event = std::ffi::CString::new(event).unwrap();
                    trace!("Rust::queue: event: {:?}", event);
                    let queuename = CString::new(event.queuename.clone())
                        .unwrap()
                        .into_string()
                        .unwrap();
                    let mut e = QUEUE_EVENTS.lock().unwrap();
                    let queue = e.get_mut(&queuename);
                    match queue {
                        Some(q) => {
                            q.push_back(event);
                        }
                        None => {
                            let mut q = std::collections::VecDeque::new();
                            q.push_back(event);
                            e.insert(queuename, q);
                        }
                    }
                }),
            )
            .await
    });

    let response = match result {
        Ok(data) => {
            let id = String::from(&data);
            let mut events = QUEUE_EVENTS.lock().unwrap();
            let queue = events.get_mut(&id);
            if queue.is_none() {
                let q = std::collections::VecDeque::new();
                let k = String::from(&data);
                events.insert(k, q);
            }
            let queuename = CString::new(id).unwrap().into_raw();
            RegisterQueueResponseWrapper {
                success: true,
                queuename,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("queue failed: {:?}", e))
                .unwrap()
                .into_raw();
            RegisterQueueResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
            }
        }
    };
    Box::into_raw(Box::new(response))
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_register_queue_response(response: *mut RegisterQueueResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}


#[repr(C)]
pub struct RegisterExchangeRequestWrapper {
    exchangename: *const c_char,
    algorithm: *const c_char,
    routingkey: *const c_char,
    addqueue: bool
}
#[repr(C)]
pub struct RegisterExchangeResponseWrapper {
    success: bool,
    queuename: *const c_char,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn register_exchange (
    client: *mut ClientWrapper,
    options: *mut RegisterExchangeRequestWrapper,
) -> *mut RegisterExchangeResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = RegisterExchangeRequest {
        exchangename: unsafe { CStr::from_ptr(options.exchangename).to_str().unwrap() }
            .to_string(),
        algorithm: unsafe { CStr::from_ptr(options.algorithm).to_str().unwrap() }
            .to_string(),
        routingkey: unsafe { CStr::from_ptr(options.routingkey).to_str().unwrap() }
            .to_string(),
        addqueue: options.addqueue,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = RegisterExchangeResponseWrapper {
            success: false,
            queuename: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    let result = runtime.block_on(async {
        client
            .as_ref()
            .unwrap()
            .register_exchange(request,
                Box::new(move |event: QueueEvent| {
                    // convert event to json
                    // let event = serde_json::to_string(&event).unwrap();
                    // let c_event = std::ffi::CString::new(event).unwrap();
                    trace!("Rust::exchange: event: {:?}", event);
                    let queuename = CString::new(event.queuename.clone())
                        .unwrap()
                        .into_string()
                        .unwrap();
                    let mut e = QUEUE_EVENTS.lock().unwrap();
                    let queue = e.get_mut(&queuename);
                    match queue {
                        Some(q) => {
                            q.push_back(event);
                        }
                        None => {
                            let mut q = std::collections::VecDeque::new();
                            q.push_back(event);
                            e.insert(queuename, q);
                        }
                    }
                }),
            
            )
            .await
    });

    let response = match result {
        Ok(data) => {
            let queuename = CString::new(data).unwrap().into_raw();
            RegisterExchangeResponseWrapper {
                success: true,
                queuename,
                error: std::ptr::null(),
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("RegisterExchange failed: {:?}", e))
                .unwrap()
                .into_raw();
            RegisterExchangeResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
#[no_mangle]
fn free_register_exchange_response(response: *mut RegisterExchangeResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct QueueEventWrapper {
    queuename: *const c_char,
    correlation_id: *const c_char,
    replyto: *const c_char,
    routingkey: *const c_char,
    exchangename: *const c_char,
    data: *const c_char,
}
impl Default for QueueEventWrapper {
    fn default() -> Self { 
        QueueEventWrapper {
            queuename: std::ptr::null(),
            correlation_id: std::ptr::null(),
            replyto: std::ptr::null(),
            routingkey: std::ptr::null(),
            exchangename: std::ptr::null(),
            data: std::ptr::null(),
        }
     }
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn next_queue_event (
    queuename: *const c_char,
) -> *mut QueueEventWrapper {
    debug!("unwrap watchid");
    let queuename = unsafe { CStr::from_ptr(queuename).to_str().unwrap() };
    debug!("queuename {:}", queuename);
    let queuename = queuename.to_string();
    debug!("unwrap events");
    let mut e = QUEUE_EVENTS.lock().unwrap();
    debug!("get queue");
    let queue = e.get_mut(&queuename);
    match queue {
        Some(q) => {
            match q.pop_front() {
                Some(event) => {
                    debug!("got event");
                    let queuename = CString::new(event.queuename).unwrap().into_raw();
                    let correlation_id = CString::new(event.correlation_id).unwrap().into_raw();
                    let replyto = CString::new(event.replyto).unwrap().into_raw();
                    let routingkey = CString::new(event.routingkey).unwrap().into_raw();
                    let exchangename = CString::new(event.exchangename).unwrap().into_raw();
                    let data = CString::new(event.data).unwrap().into_raw();
                    let event = Box::new(QueueEventWrapper {
                        queuename,
                        correlation_id,
                        replyto,
                        routingkey,
                        exchangename,
                        data,
                    });
                    Box::into_raw(event)
                }
                None => {
                    debug!("No event");
                    Box::into_raw(Box::new(QueueEventWrapper::default())) 
                },
            }
        },
        None => {
            debug!("Queue for {:} not found", queuename);
            Box::into_raw(Box::new(QueueEventWrapper::default())) 
        },
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_queue_event(response: *mut QueueEventWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct UnRegisterQueueResponseWrapper {
    success: bool,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn unregister_queue(
    client: *mut ClientWrapper,
    queuename: *const c_char,
) -> *mut UnRegisterQueueResponseWrapper {
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let queuename = unsafe { CStr::from_ptr(queuename).to_str().unwrap() };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UnRegisterQueueResponseWrapper {
            success: false,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.unregister_queue(queuename).await
    });
    match result {
        Ok(_) => {
            let response = UnRegisterQueueResponseWrapper {
                success: true,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Unregister queue failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = UnRegisterQueueResponseWrapper {
                success: false,
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_unregister_queue_response(response: *mut UnRegisterQueueResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}



#[repr(C)]
#[derive(Debug, Clone)]
pub struct WorkitemFileWrapper {
    filename: *const c_char,
    id: *const c_char,
    compressed: bool,
    file: vec::Vec<u8>,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WorkitemWrapper {
    id: *const c_char,
    name: *const c_char,
    payload: *const c_char,
    priority: i32,
    nextrun: u64,
    lastrun: u64,
    files: *const *const WorkitemFileWrapper,
    files_len: usize,
    state: *const c_char,
    wiq: *const c_char,
    wiqid: *const c_char,
    retries: i32,
    username: *const c_char,
    success_wiqid: *const c_char,
    failed_wiqid: *const c_char,
    success_wiq: *const c_char,
    failed_wiq: *const c_char,
    errormessage: *const c_char,
    errorsource: *const c_char,
    errortype: *const c_char,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PushWorkitemRequestWrapper {
    wiq: *const c_char,
    wiqid: *const c_char,
    name: *const c_char,
    payload: *const c_char,
    nextrun: u64,
    success_wiqid: *const c_char,
    failed_wiqid: *const c_char,
    success_wiq: *const c_char,
    failed_wiq: *const c_char,
    priority: i32,
    files: *const *const WorkitemFileWrapper,
    files_len: usize,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PushWorkitemResponseWrapper {
    success: bool,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn push_workitem(
    client: *mut ClientWrapper,
    options: *mut PushWorkitemRequestWrapper,
) -> *mut PushWorkitemResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let files_len = options.files_len;
    debug!("files_len: {:?}", files_len);
    let mut files: Vec<WorkitemFile> = vec![];
    if files_len > 0 {
        debug!("get files of options");
        let _files = unsafe { &*options.files };
        debug!("slice files");
        let _files = unsafe { std::slice::from_raw_parts(_files, files_len) };
        debug!("loop files");
        files = _files.iter().map(|f| {
            debug!("process a file");
            let file = unsafe { &**f };
            debug!("create WorkitemFile instance");
            WorkitemFile {
                filename: unsafe { CStr::from_ptr(file.filename).to_str().unwrap() }.to_string(),
                id: unsafe { CStr::from_ptr(file.id).to_str().unwrap() }.to_string(),
                compressed: file.compressed,
                ..Default::default()
                // file: file.file.clone(),
            }
        }).collect();
    }
    trace!("nextrun: {:?}", options.nextrun);
    // convert options.nextrun to std::time::SystemTime
    let mut nextrun = Some(Timestamp::from(
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(options.nextrun as u64)
    ));
    trace!("nextrun: {:?}", nextrun);
    if options.nextrun  <= 0 {
        nextrun = None;
    }
    let request = PushWorkitemRequest {
        wiq: unsafe { CStr::from_ptr(options.wiq).to_str().unwrap() }.to_string(),
        wiqid: unsafe { CStr::from_ptr(options.wiqid).to_str().unwrap() }.to_string(),
        name: unsafe { CStr::from_ptr(options.name).to_str().unwrap() }.to_string(),
        payload: unsafe { CStr::from_ptr(options.payload).to_str().unwrap() }.to_string(),
        nextrun: nextrun,
        success_wiqid: unsafe { CStr::from_ptr(options.success_wiqid).to_str().unwrap() }.to_string(),
        failed_wiqid: unsafe { CStr::from_ptr(options.failed_wiqid).to_str().unwrap() }.to_string(),
        success_wiq: unsafe { CStr::from_ptr(options.success_wiq).to_str().unwrap() }.to_string(),
        failed_wiq: unsafe { CStr::from_ptr(options.failed_wiq).to_str().unwrap() }.to_string(),
        priority: options.priority,
        files: files,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PushWorkitemResponseWrapper {
            success: false,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }

    let result = runtime.block_on(async {
        client
            .
            as_ref()
            .unwrap()
            .push_workitem(request)
            .await
    });

    let response = match result {
        Ok(_) => {
            let response = PushWorkitemResponseWrapper {
                success: true,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Push workitem failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = PushWorkitemResponseWrapper {
                success: false,
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    };

    response
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn push_workitem_async( 
    client: *mut ClientWrapper,
    options: *mut PushWorkitemRequestWrapper,
    callback: extern "C" fn(*mut PushWorkitemResponseWrapper),
) {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let files_len = options.files_len;
    debug!("files_len: {:?}", files_len);
    let mut files: Vec<WorkitemFile> = vec![];
    if files_len > 0 {
        debug!("get files of options");
        let _files = unsafe { &*options.files };
        debug!("slice files");
        let _files = unsafe { std::slice::from_raw_parts(_files, files_len) };
        debug!("loop files");
        files = _files.iter().map(|f| {
            debug!("process a file");
            let file = unsafe { &**f };
            debug!("create WorkitemFile instance 2");
            let filename = unsafe { CStr::from_ptr(file.filename).to_str().unwrap() }.to_string();
            debug!("filename: {:?}", filename);
            let id = unsafe { CStr::from_ptr(file.id).to_str().unwrap() }.to_string();
            debug!("id: {:?}", id);
            let compressed = file.compressed;
            debug!("compressed: {:?}", compressed); 
            WorkitemFile {
                filename: unsafe { CStr::from_ptr(file.filename).to_str().unwrap() }.to_string(),
                id: unsafe { CStr::from_ptr(file.id).to_str().unwrap() }.to_string(),
                compressed: file.compressed,
                ..Default::default()
                // file: file.file.clone(),
            }
        }).collect();
    }
    trace!("nextrun: {:?}", options.nextrun);
    // convert options.nextrun to std::time::SystemTime
    let mut nextrun = Some(Timestamp::from(
        std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(options.nextrun as u64)
    ));
    trace!("nextrun: {:?}", nextrun);
    if options.nextrun  <= 0 {
        nextrun = None;
    }
    let request = PushWorkitemRequest {
        wiq: unsafe { CStr::from_ptr(options.wiq).to_str().unwrap() }.to_string(),
        wiqid: unsafe { CStr::from_ptr(options.wiqid).to_str().unwrap() }.to_string(),
        name: unsafe { CStr::from_ptr(options.name).to_str().unwrap() }.to_string(),
        payload: unsafe { CStr::from_ptr(options.payload).to_str().unwrap() }.to_string(),
        nextrun: nextrun,
        success_wiqid: unsafe { CStr::from_ptr(options.success_wiqid).to_str().unwrap() }.to_string(),
        failed_wiqid: unsafe { CStr::from_ptr(options.failed_wiqid).to_str().unwrap() }.to_string(),
        success_wiq: unsafe { CStr::from_ptr(options.success_wiq).to_str().unwrap() }.to_string(),
        failed_wiq: unsafe { CStr::from_ptr(options.failed_wiq).to_str().unwrap() }.to_string(),
        priority: options.priority,
        files: files,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PushWorkitemResponseWrapper {
            success: false,
            error: error_msg,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    
    runtime.spawn(async move {
        let result = client
            .as_ref()
            .unwrap()
            .push_workitem(request)
            .await;
        let response = match result {
            Ok(_) => {
                let response = PushWorkitemResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                };
                Box::into_raw(Box::new(response))
            }
            Err(e) => {
                let error_msg = CString::new(format!("Push workitem failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                let response = PushWorkitemResponseWrapper {
                    success: false,
                    error: error_msg,
                };
                Box::into_raw(Box::new(response))
            }
        };
        callback(response);
    });


}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_push_workitem_response(response: *mut PushWorkitemResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PopWorkitemRequestWrapper {
    wiq: *const c_char,
    wiqid: *const c_char,
    // includefiles: bool,
    // compressed: bool,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PopWorkitemResponseWrapper {
    success: bool,
    error: *const c_char,
    workitem: *const WorkitemWrapper,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn pop_workitem (
    client: *mut ClientWrapper,
    options: *mut PopWorkitemRequestWrapper,
    downloadfolder: *const c_char,
) -> *mut PopWorkitemResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = PopWorkitemRequest {
        wiq: unsafe { CStr::from_ptr(options.wiq).to_str().unwrap() }.to_string(),
        wiqid: unsafe { CStr::from_ptr(options.wiqid).to_str().unwrap() }.to_string(),
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PopWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
        };
        return Box::into_raw(Box::new(response));
    }
    let downloadfolder = unsafe { CStr::from_ptr(downloadfolder).to_str().unwrap() };
    let mut _downloadfolder = Some(downloadfolder);
    if downloadfolder.is_empty() {
        _downloadfolder = None;
    }
    let result = runtime.block_on(async {
        client
            .as_ref()
            .unwrap()
            .pop_workitem(request, _downloadfolder)
            .await
    });
    debug!("pop_workitem completed, parse result");

    let response = match result {
        Ok(data) => {
            let workitem = match data.workitem {
                Some(workitem) => {
                    trace!("parse workitem: {:?}", workitem);
                    let id = CString::new(workitem.id).unwrap().into_raw();
                    let name = CString::new(workitem.name).unwrap().into_raw();
                    let payload = CString::new(workitem.payload).unwrap().into_raw();
                    let state = CString::new(workitem.state).unwrap().into_raw();
                    let wiq = CString::new(workitem.wiq).unwrap().into_raw();
                    let wiqid = CString::new(workitem.wiqid).unwrap().into_raw();
                    let username = CString::new(workitem.username).unwrap().into_raw();
                    let success_wiqid = CString::new(workitem.success_wiqid).unwrap().into_raw();
                    let failed_wiqid = CString::new(workitem.failed_wiqid).unwrap().into_raw();
                    let success_wiq = CString::new(workitem.success_wiq).unwrap().into_raw();
                    let failed_wiq = CString::new(workitem.failed_wiq).unwrap().into_raw();
                    let errormessage = CString::new(workitem.errormessage).unwrap().into_raw();
                    let errorsource = CString::new(workitem.errorsource).unwrap().into_raw();
                    let errortype = CString::new(workitem.errortype).unwrap().into_raw();
                    let mut files: Vec<*const WorkitemFileWrapper> = vec![];
                    for f in &workitem.files {
                        trace!("parse workitem file: {:?}", f);
                        let filename = CString::new(f.filename.clone()).unwrap().into_raw();
                        let id = CString::new(f.id.clone()).unwrap().into_raw();
                        let file: Vec<u8> = Vec::new();
                        let compressed = f.compressed;
                        let file = Box::into_raw(Box::new(WorkitemFileWrapper {
                            filename,
                            id,
                            file,
                            compressed,
                        }));
                        files.push(file);
                    }
                    trace!("files: {:?} at {:?}", files.len(), files);
                    // let files = workitem.files.iter().map(|f| {
                    //     let filename = CString::new(f.filename.clone()).unwrap().into_raw();
                    //     let id = CString::new(f.id.clone()).unwrap().into_raw();
                    //     let file = f.file.clone();
                    //     let compressed = f.compressed;
                    //     let file = Box::new(WorkitemFileWrapper {
                    //         filename,
                    //         id,
                    //         file,
                    //         compressed,
                    //     });
                    //     Box::into_raw(file)
                    // }).collect::<Vec<*const WorkitemFileWrapper>>();
                    let workitem = WorkitemWrapper {
                        id,
                        name,
                        payload,
                        priority: workitem.priority,
                        nextrun: 0,
                        lastrun: 0,
                        files: files.as_ptr(),
                        files_len: files.len(),
                        state,
                        wiq,
                        wiqid,
                        retries: workitem.retries,
                        username,
                        success_wiqid,
                        failed_wiqid,
                        success_wiq,
                        failed_wiq,
                        errormessage,
                        errorsource,
                        errortype,
                    };
                    std::mem::forget(files);
                    Box::into_raw(Box::new(workitem))
                },
                None => {
                    std::ptr::null()
                }
            };
            let response = PopWorkitemResponseWrapper {
                success: true,
                error: std::ptr::null(),
                workitem,
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Pop workitem failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = PopWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
    };
    debug!("completed, return response");
    response
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_pop_workitem_response(response: *mut PopWorkitemResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}


pub type TransferCallback = unsafe extern "C" fn(i32);

// Works
#[no_mangle]
pub extern fn add(n1: i32, n2: i32, cb: TransferCallback){
  unsafe { cb(n1 + n2) } 
}

#[no_mangle]
pub extern fn add_async(n1: i32, n2: i32, cb: TransferCallback) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("add_async running 1");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("add_async running 2");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("add_async running 3");
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("RUST: add_async call callback");
        unsafe { cb(n1 + n2) } 
    });
}
#[no_mangle]
pub extern fn add_async2(n1: i32, n2: i32, cb: TransferCallback) {
  let (tx, rx) = std::sync::mpsc::channel();
  std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("add_async2 running 1");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("add_async2 running 2");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("add_async2 running 3");
    std::thread::sleep(std::time::Duration::from_secs(1));
    tx.send(n1 + n2)
  });
  let sum = rx.recv().unwrap();
  println!("RUST: add_async2 call callback");
  unsafe { cb(sum) } 
}