// #![warn(missing_docs)]
//! FFI bindings for the OpenIAP client library.
//! used by the OpenIAP client library for other programming languages to interact with the client library.
//! For now, nodejs, python and dotnet 6
use openiap_client::protos::{
    AggregateRequest, CountRequest, DistinctRequest, DownloadRequest, Envelope, InsertOneRequest,
    QueryRequest, SigninRequest, UploadRequest, WatchEvent, WatchRequest,
};
use openiap_client::{Client, ClientEvent, CreateCollectionRequest, CreateIndexRequest, DeleteManyRequest, DeleteOneRequest, DeleteWorkitemRequest, DropCollectionRequest, DropIndexRequest, GetIndexesRequest, InsertManyRequest, InsertOrUpdateOneRequest, PopWorkitemRequest, PushWorkitemRequest, QueueEvent, QueueMessageRequest, RegisterExchangeRequest, RegisterQueueRequest, Timestamp, UpdateOneRequest, UpdateWorkitemRequest, Workitem, WorkitemFile};

#[cfg(all(test, not(windows)))]
mod tests;

use std::collections::{HashMap, VecDeque};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Mutex;
use std::vec;
// use tokio::runtime::Runtime;
use tracing::{debug, info, trace};

mod safe_wrappers;
use safe_wrappers::{c_char_to_str, safe_wrapper};

use lazy_static::lazy_static;
lazy_static! {
    // TODO: Add max size for the queues
    static ref WATCH_EVENTS: std::sync::Mutex<HashMap<String, VecDeque<WatchEvent>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    static ref QUEUE_EVENTS: std::sync::Mutex<HashMap<String, VecDeque<QueueEvent>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    static ref CLIENT_EVENTS: std::sync::Mutex<HashMap<String, VecDeque<ClientEvent>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };

}
/// A wrapper for the client library.
/// This struct is used to hold the client instance and the runtime instance.
#[repr(C)]
pub struct ClientWrapper {
    success: bool,
    error: *const c_char,
    client: Option<Client>
}
/// WatchEventWrapper is a wrapper for the WatchEvent struct.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WatchEventWrapper {
    id: *const c_char,
    operation: *const c_char,
    document: *const c_char,
    request_id: i32,
}
impl Default for WatchEventWrapper {
    fn default() -> Self { 
        WatchEventWrapper {
            id: std::ptr::null(),
            operation: std::ptr::null(),
            document: std::ptr::null(),
            request_id: 0,
        }
     }
}
#[repr(C)]
pub struct UserWrapper {
    id: *const c_char,
    name: *const c_char,
    username: *const c_char,
    email: *const c_char,
    roles: *const *const c_char,
    roles_len: i32,
}

/// Return currentlly signed in user
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_user(
    client: *mut ClientWrapper
) -> *const UserWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            return std::ptr::null();
        }
    };
    let b = client_wrapper.client.clone().unwrap();
    let user = b.get_user();
    match user {
        None => {
            return std::ptr::null();
        }
        Some(user) => {
            let role_strings: Vec<CString> = user
                .roles
                .iter()
                .map(|role| CString::new(role.name.clone()).unwrap())
                .collect();

            let role_ptrs: Vec<*const c_char> =
                role_strings.iter().map(|cs| cs.as_ptr()).collect();
            let roles_buf = role_ptrs.into_boxed_slice();
            let roles_ptr = roles_buf.as_ptr();
            let roles_len = role_strings.len();

            let response = UserWrapper {
                id: CString::new(user.id).unwrap().into_raw(),
                name: CString::new(user.name).unwrap().into_raw(),
                username: CString::new(user.username).unwrap().into_raw(),
                email: CString::new(user.email).unwrap().into_raw(),
                roles: roles_ptr,
                roles_len: roles_len as i32,
            };
            return Box::into_raw(Box::new(response)) as *mut UserWrapper;
        }
    }
}
/// Free the user wrapper
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_user(user: *mut UserWrapper) {
    if user.is_null() {
        return;
    }
    unsafe {
        if !(*user).id.is_null() {
            let _ = CString::from_raw((*user).id as *mut c_char);
        }
        if !(*user).name.is_null() {
            let _ = CString::from_raw((*user).name as *mut c_char);
        }
        if !(*user).username.is_null() {
            let _ = CString::from_raw((*user).username as *mut c_char);
        }
        if !(*user).email.is_null() {
            let _ = CString::from_raw((*user).email as *mut c_char);
        }
        if !(*user).roles.is_null() {
            let roles = (*user).roles;
            let mut i = 0;
            while !roles.offset(i).is_null() {
                let _ = CString::from_raw(roles.offset(i) as *mut c_char);
                i += 1;
            }
        }
        let _ = Box::from_raw(user);
    }
}

/// QueryRequestWrapper is a wrapper for the QuQueryResponseWrappereryRequest struct.
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
    request_id: i32,
}
/// QueryResponseWrapper is a wrapper for the QueryResponse struct.
#[repr(C)]
pub struct QueryResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
    request_id: i32,
}
// run query syncronously
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn query(
    client: *mut ClientWrapper,
    options: *mut QueryRequestWrapper,
) -> *mut QueryResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = QueryResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = QueryResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let request = QueryRequest {
        collectionname: c_char_to_str(options.collectionname),
        query: c_char_to_str(options.query),
        projection: c_char_to_str(options.projection),
        orderby: c_char_to_str(options.orderby),
        queryas: c_char_to_str(options.queryas),
        explain: options.explain,
        skip: options.skip,
        top: options.top
    };
    if client_wrapper.client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = QueryResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client_wrapper.client.clone().unwrap();

    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.query(request))
    });
    Box::into_raw(Box::new(match result {
        Ok(data) => {
            let results: *const c_char = CString::new(data.results).unwrap().into_raw();
            QueryResponseWrapper {
                success: true,
                results,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
            }
        }
    }))
}
/// QueryCallback is a callback function for the query_async function.
type QueryCallback = extern "C" fn(wrapper: *mut QueryResponseWrapper);
// run query asyncronously
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn query_async(
    client: *mut ClientWrapper,
    options: *mut QueryRequestWrapper,
    callback: QueryCallback,
) {
    debug!("Rust: query_async");
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = QueryResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = QueryResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let collectionname = c_char_to_str(options.collectionname);
    let query = c_char_to_str(options.query);
    let projection = c_char_to_str(options.projection);
    let orderby = c_char_to_str(options.orderby);
    let queryas = c_char_to_str(options.queryas);
    let explain = options.explain;
    let skip = options.skip;
    let top = options.top;
    debug!("Rust: query_async: collectionname: {}, query: {}, projection: {}, orderby: {}, queryas: {}, explain: {}, skip: {}, top: {}", collectionname, query, projection, orderby, queryas, explain, skip, top);

    let request = QueryRequest {
        collectionname: collectionname.to_string(),
        query: query.to_string(),
        projection: projection.to_string(),
        orderby: orderby.to_string(),
        queryas: queryas.to_string(),
        explain,
        skip,
        top
    };    
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = QueryResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    debug!("Rust: runtime.spawn");
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    // tokio::spawn(async move {
    let _guard = handle.enter();
    handle.spawn(async move {
        debug!("Rust: client.query");
        let result = client.query(request).await;

        let response = match result {
            Ok(data) => {
                let results: *const c_char = CString::new(data.results).unwrap().into_raw();
                QueryResponseWrapper {
                    success: true,
                    results,
                    error: std::ptr::null(),
                    request_id: request_id,
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
                    request_id: request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).results.is_null() {
            let _ = CString::from_raw((*response).results as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}


#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn enable_tracing(rust_log: *const c_char, tracing: *const c_char) {
    let rust_log = c_char_to_str(rust_log);
    let rust_log = rust_log.to_string();
    let tracing = c_char_to_str(tracing);
    let tracing = tracing.to_string();
    openiap_client::enable_tracing(&rust_log, &tracing);
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn disable_tracing() {
    openiap_client::disable_tracing();
}


fn free<T>(ptr: *mut T) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn create_client() -> *mut ClientWrapper {
    let client = Client::new();
    trace!("create_client");
    Box::into_raw(Box::new(ClientWrapper {
        client: Some(client),
        success: true,
        error: std::ptr::null(),
    }))    
}
/// A wrapper for the client library.
/// This struct is used to hold the client instance and the runtime instance.
#[repr(C)]
pub struct ConnectResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
pub extern "C" fn client_connect(client_wrap: *mut ClientWrapper, server_address: *const c_char) -> *mut ConnectResponseWrapper {
    let server_address = c_char_to_str(server_address);
    info!("server_address = {:?}", server_address);
    let client = match safe_wrapper( client_wrap ) {
        Some( wrap ) => wrap.client.clone().unwrap(),
        None => {
            Client::new()
        }
    };
    info!("connect::begin");
    let res: Result<(), openiap_client::OpenIAPError> = client.connect(&server_address);
    info!("connect::complete");
    if res.is_err() {
        let e = res.err().unwrap();
        info!("error_msg = {:?}", format!("Connection failed: {:?}", e));
        let error_msg = CString::new(format!("Connection failed: {:?}", e))
            .unwrap()
            .into_raw();
        
        let result = Box::into_raw(Box::new(ConnectResponseWrapper {
            success: false,
            error: error_msg,
            request_id: 0,
        }));
        info!("connect::complete error result address: {:?}", result);
        return result;
    }
    let result = Box::into_raw(Box::new(ConnectResponseWrapper {
        success: true,
        error: std::ptr::null(),
        request_id: 0,
    }));
    info!("connect::complete result address: {:?}", result);
    result
}
type ConnectCallback = extern "C" fn(wrapper: *mut ConnectResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn connect_async(client: *mut ClientWrapper, server_address: *const c_char, request_id: i32, callback: ConnectCallback) {
    debug!("connect_async");
    let server_address = c_char_to_str(server_address);
    debug!("server_address = {:?}", server_address);

    let client = match safe_wrapper( client ) {
        Some( wrap ) => wrap.client.clone().unwrap(),
        None => {
            Client::new()
        }
    };

    trace!("Spawn the async task");
    let client_result: Result<(), openiap_client::OpenIAPError> = client.connect(&server_address);
    let handle = client.get_runtime_handle();
    handle.spawn(async move {
        let wrapper = if client_result.is_ok() {
            Box::into_raw(Box::new(ConnectResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id,
            }))
        } else {
            let e = client_result.err().unwrap();
            let error_msg = CString::new(format!("Connection failed: {:?}", e))
                .unwrap()
                .into_raw();
            Box::into_raw(Box::new(ConnectResponseWrapper {
                success: false,
                error: error_msg,
                request_id,
            }))
        };
        trace!("Client::Calling callback with result");
        callback(wrapper);
    });
}
#[no_mangle]
pub extern "C" fn client_set_agent_name(client_wrap: *mut ClientWrapper, agent_name: *const c_char) {
    let agent_name = c_char_to_str(agent_name);
    debug!("set_agent_name = {:?}", agent_name);
    let client = match safe_wrapper( client_wrap ) {
        Some( wrap ) => wrap.client.clone().unwrap(),
        None => {
            Client::new()
        }
    };
    client.set_agent_name(&agent_name);
}
#[no_mangle]
pub extern "C" fn client_set_agent_version(client_wrap: *mut ClientWrapper, agent_version: *const c_char) {
    let agent_version = c_char_to_str(agent_version);
    debug!("set_agent_version = {:?}", agent_version);
    let client = match safe_wrapper( client_wrap ) {
        Some( wrap ) => wrap.client.clone().unwrap(),
        None => {
            Client::new()
        }
    };
    client.set_agent_version(&agent_version);
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_connect_response(response: *mut ConnectResponseWrapper) {
    free(response);
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn client_disconnect(client_wrap: *mut ClientWrapper) {
    let client = match safe_wrapper( client_wrap ) {
        Some( wrap ) => wrap.client.clone().unwrap(),
        None => {
            Client::new()
        }
    };
    client.disconnect();
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_client(response: *mut ClientWrapper) {
    if response.is_null() {
        debug!("free_client: response is null");
        return;
    }
    unsafe {
        let response_ref: &ClientWrapper = match safe_wrapper(response) {
            Some(response) => response,
            None => {
                debug!("free_client: response is not valid");
                return;
            }
        };
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
            client.disconnect();

            let handle = client.get_runtime_handle();
            // Ensure that the runtime properly shuts down after the block_on call
            handle.spawn(async move {
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
    request_id: i32
}
#[repr(C)]
pub struct SigninResponseWrapper {
    success: bool,
    jwt: *const c_char,
    error: *const c_char,
    request_id: i32
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn signin(
    client: *mut ClientWrapper,
    options: *mut SigninRequestWrapper,
) -> *mut SigninResponseWrapper {
    let options = match safe_wrapper( options ) {
        Some( options ) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = SigninResponseWrapper {
                success: false,
                jwt: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper: &mut ClientWrapper = match safe_wrapper( client ) {
        Some( client ) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = SigninResponseWrapper {
                success: false,
                jwt: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();

    let request = SigninRequest {
        username: c_char_to_str(options.username),
        password: c_char_to_str(options.password),
        jwt: c_char_to_str(options.jwt),
        agent: c_char_to_str(options.agent),
        version: c_char_to_str(options.version),
        longtoken: options.longtoken,
        ping: options.ping,
        validateonly: options.validateonly
    };

    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = SigninResponseWrapper {
            success: false,
            jwt: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.signin(request))
    });

    let response = match result {
        Ok(data) => {
            let jwt = CString::new(data.jwt).unwrap().into_raw();
            SigninResponseWrapper {
                success: true,
                jwt,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options ) {
        Some( options ) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = SigninResponseWrapper {
                success: false,
                jwt: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some( client ) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = SigninResponseWrapper {
                success: false,
                jwt: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();

    let request = SigninRequest {
        username: c_char_to_str(options.username),
        password: c_char_to_str(options.password),
        jwt: c_char_to_str(options.jwt),
        agent: c_char_to_str(options.agent),
        version: c_char_to_str(options.version),
        longtoken: options.longtoken,
        ping: options.ping,
        validateonly: options.validateonly
    };

    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = SigninResponseWrapper {
            success: false,
            jwt: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        // let result = client_clone.unwrap().signin(request).await;
        let result = client.signin(request).await;

        let response = match result {
            Ok(data) => {
                let jwt = CString::new(data.jwt).unwrap().into_raw();
                Box::new(SigninResponseWrapper {
                    success: true,
                    jwt,
                    error: std::ptr::null(),
                    request_id
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
                    request_id
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).jwt.is_null() {
            let _ = CString::from_raw((*response).jwt as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct ListCollectionsResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn list_collections(
    client: *mut ClientWrapper,
    includehist: bool,
) -> *mut ListCollectionsResponseWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = ListCollectionsResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = ListCollectionsResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: 0,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.list_collections(includehist))
    });

    let response = match result {
        Ok(data) => {
            let results = CString::new(data).unwrap().into_raw();
            ListCollectionsResponseWrapper {
                success: true,
                results,
                error: std::ptr::null(),
                request_id: 0,                
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("List collections failed: {:?}", e))
                .unwrap()
                .into_raw();
            ListCollectionsResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type ListCollectionsCallback = extern "C" fn(wrapper: *mut ListCollectionsResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn list_collections_async(
    client: *mut ClientWrapper,
    includehist: bool,
    request_id: i32,
    callback: ListCollectionsCallback,
) {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = ListCollectionsResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = ListCollectionsResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    handle.spawn(async move {
        let result = client.list_collections(includehist).await;

        let response = match result {
            Ok(data) => {
                let results = CString::new(data).unwrap().into_raw();
                ListCollectionsResponseWrapper {
                    success: true,
                    results,
                    error: std::ptr::null(),
                    request_id: request_id,
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("List collections failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                ListCollectionsResponseWrapper {
                    success: false,
                    results: std::ptr::null(),
                    error: error_msg,
                    request_id: request_id,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_list_collections_response(response: *mut ListCollectionsResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).results.is_null() {
            let _ = CString::from_raw((*response).results as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct ColCollationWrapper {
    locale: *const c_char,
    case_level: bool,
    case_first: *const c_char,
    strength: i32,
    numeric_ordering: bool,
    alternate: *const c_char,
    max_variable: *const c_char,
    backwards: bool,
}
#[repr(C)]
pub struct ColTimeseriesWrapper {
    time_field: *const c_char,
    meta_field: *const c_char,
    granularity: *const c_char,
}
#[repr(C)]
pub struct CreateCollectionRequestWrapper {
    collectionname: *const c_char,
    collation: *mut ColCollationWrapper,
    timeseries: *mut ColTimeseriesWrapper,
    expire_after_seconds: i32,
    change_stream_pre_and_post_images: bool,
    capped: bool,
    max: i32,
    size: i32,
    request_id: i32,
}
#[repr(C)]
pub struct CreateCollectionResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn create_collection(
    client: *mut ClientWrapper,
    options: *mut CreateCollectionRequestWrapper,
) -> *mut CreateCollectionResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = CreateCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = CreateCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = CreateCollectionRequest {
        collectionname: c_char_to_str(options.collectionname),
        collation: match safe_wrapper(options.collation) {
            Some(collation) => {
                Some(openiap_client::protos::ColCollation {
                    locale: c_char_to_str(collation.locale),
                    case_level: collation.case_level,
                    case_first: c_char_to_str(collation.case_first),
                    strength: collation.strength,
                    numeric_ordering: collation.numeric_ordering,
                    alternate: c_char_to_str(collation.alternate),
                    max_variable: c_char_to_str(collation.max_variable),
                    backwards: collation.backwards,
                })
            }
            None => None,
        },
        timeseries: match safe_wrapper(options.timeseries) {
            Some(timeseries) => {
                Some(openiap_client::protos::ColTimeseries {
                    time_field: c_char_to_str(timeseries.time_field),
                    meta_field: c_char_to_str(timeseries.meta_field),
                    granularity: c_char_to_str(timeseries.granularity),
                })
            }
            None => None,
        },
        expire_after_seconds: options.expire_after_seconds,
        change_stream_pre_and_post_images: options.change_stream_pre_and_post_images,
        capped: options.capped,
        max: options.max,
        size: options.size,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CreateCollectionResponseWrapper {
            success: false,
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.create_collection(request))
    });

    let response = match result {
        Ok(_) => {
            CreateCollectionResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id: options.request_id,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Create collection failed: {:?}", e))
                .unwrap()
                .into_raw();
            CreateCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id,
            }
        }
    };
    Box::into_raw(Box::new(response))
}
type CreateCollectionCallback = extern "C" fn(wrapper: *mut CreateCollectionResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn create_collection_async(
    client: *mut ClientWrapper,
    options: *mut CreateCollectionRequestWrapper,
    callback: CreateCollectionCallback,
) {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = CreateCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = CreateCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = CreateCollectionRequest {
        collectionname: c_char_to_str(options.collectionname),
        collation: match safe_wrapper(options.collation) {
            Some(collation) => {
                Some(openiap_client::protos::ColCollation {
                    locale: c_char_to_str(collation.locale),
                    case_level: collation.case_level,
                    case_first: c_char_to_str(collation.case_first),
                    strength: collation.strength,
                    numeric_ordering: collation.numeric_ordering,
                    alternate: c_char_to_str(collation.alternate),
                    max_variable: c_char_to_str(collation.max_variable),
                    backwards: collation.backwards,
                })
            }
            None => None,
        },
        timeseries: match safe_wrapper(options.timeseries) {
            Some(timeseries) => {
                Some(openiap_client::protos::ColTimeseries {
                    time_field: c_char_to_str(timeseries.time_field),
                    meta_field: c_char_to_str(timeseries.meta_field),
                    granularity: c_char_to_str(timeseries.granularity),
                })
            }
            None => None,
        },
        expire_after_seconds: options.expire_after_seconds,
        change_stream_pre_and_post_images: options.change_stream_pre_and_post_images,
        capped: options.capped,
        max: options.max,
        size: options.size,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CreateCollectionResponseWrapper {
            success: false,
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.create_collection(request).await;

        let response = match result {
            Ok(_) => {
                CreateCollectionResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                    request_id
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Create collection failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                CreateCollectionResponseWrapper {
                    success: false,
                    error: error_msg,
                    request_id
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_create_collection_response(response: *mut CreateCollectionResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DropCollectionResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn drop_collection(
    client: *mut ClientWrapper,
    collectionname: *const c_char,
) -> *mut DropCollectionResponseWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DropCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DropCollectionRequest {
        collectionname: c_char_to_str(collectionname),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DropCollectionResponseWrapper {
            success: false,
            error: error_msg,
            request_id: 0,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.drop_collection(request))
    });

    let response = match result {
        Ok(_) => {
            DropCollectionResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id: 0,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Drop collection failed: {:?}", e))
                .unwrap()
                .into_raw();
            DropCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type DropCollectionCallback = extern "C" fn(wrapper: *mut DropCollectionResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn drop_collection_async(
    client: *mut ClientWrapper,
    collectionname: *const c_char,
    request_id: i32,
    callback: DropCollectionCallback,
) {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DropCollectionResponseWrapper {
                success: false,
                error: error_msg,
                request_id: request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DropCollectionRequest {
        collectionname: c_char_to_str(collectionname),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DropCollectionResponseWrapper {
            success: false,
            error: error_msg,
            request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    handle.spawn(async move {
        let result = client.drop_collection(request).await;

        let response = match result {
            Ok(_) => {
                DropCollectionResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                    request_id,
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Drop collection failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DropCollectionResponseWrapper {
                    success: false,
                    error: error_msg,
                    request_id,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_drop_collection_response(response: *mut DropCollectionResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct GetIndexesResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn get_indexes(
    client: *mut ClientWrapper,
    collectionname: *const c_char,
) -> *mut GetIndexesResponseWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = GetIndexesResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = GetIndexesRequest {
        collectionname: c_char_to_str(collectionname),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = GetIndexesResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: 0,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.get_indexes(request))
    });

    let response = match result {
        Ok(data) => {
            let results = CString::new(data).unwrap().into_raw();
            GetIndexesResponseWrapper {
                success: true,
                results,
                error: std::ptr::null(),
                request_id: 0,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Get indexes failed: {:?}", e))
                .unwrap()
                .into_raw();
            GetIndexesResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type GetIndexesCallback = extern "C" fn(wrapper: *mut GetIndexesResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn get_indexes_async(
    client: *mut ClientWrapper,
    collectionname: *const c_char,
    request_id: i32,
    callback: GetIndexesCallback,
)  {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = GetIndexesResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = GetIndexesRequest {
        collectionname: c_char_to_str(collectionname),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = GetIndexesResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    handle.spawn(async move {
        let result = client.get_indexes(request).await;

        let response = match result {
            Ok(data) => {
                let results = CString::new(data).unwrap().into_raw();
                GetIndexesResponseWrapper {
                    success: true,
                    results,
                    error: std::ptr::null(),
                    request_id,
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Get indexes failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                GetIndexesResponseWrapper {
                    success: false,
                    results: std::ptr::null(),
                    error: error_msg,
                    request_id,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });

}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_get_indexes_response(response: *mut GetIndexesResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).results.is_null() {
            let _ = CString::from_raw((*response).results as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct CreateIndexRequestWrapper {
    collectionname: *const c_char,
    index: *const c_char,
    options: *const c_char,
    name: *const c_char,
    request_id: i32,
}
#[repr(C)]
pub struct CreateIndexResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn create_index(
    client: *mut ClientWrapper,
    options: *mut CreateIndexRequestWrapper,
) -> *mut CreateIndexResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = CreateIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = CreateIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = CreateIndexRequest {
        collectionname: c_char_to_str(options.collectionname),
        index: c_char_to_str(options.index),
        options: c_char_to_str(options.options),
        name: c_char_to_str(options.name),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CreateIndexResponseWrapper {
            success: false,
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.create_index(request))
    });

    let response = match result {
        Ok(_) => {
            CreateIndexResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id: options.request_id,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Create index failed: {:?}", e))
                .unwrap()
                .into_raw();
            CreateIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type CreateIndexCallback = extern "C" fn(wrapper: *mut CreateIndexResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn create_index_async(
    client: *mut ClientWrapper,
    options: *mut CreateIndexRequestWrapper,
    callback: CreateIndexCallback,
) {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = CreateIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = CreateIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = CreateIndexRequest {
        collectionname: c_char_to_str(options.collectionname),
        index: c_char_to_str(options.index),
        options: c_char_to_str(options.options),
        name: c_char_to_str(options.name),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CreateIndexResponseWrapper {
            success: false,
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.create_index(request).await;

        let response = match result {
            Ok(_) => {
                CreateIndexResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                    request_id
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Create index failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                CreateIndexResponseWrapper {
                    success: false,
                    error: error_msg,
                    request_id
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_create_index_response(response: *mut CreateIndexResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DropIndexResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn drop_index(
    client: *mut ClientWrapper,
    collectionname: *const c_char,
    name: *const c_char,
) -> *mut DropIndexResponseWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DropIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DropIndexRequest {
        collectionname: c_char_to_str(collectionname),
        name: c_char_to_str(name),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DropIndexResponseWrapper {
            success: false,
            error: error_msg,
            request_id: 0,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.drop_index(request))
    });

    let response = match result {
        Ok(_) => {
            DropIndexResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id: 0,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Drop index failed: {:?}", e))
                .unwrap()
                .into_raw();
            DropIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            }
        }
    };

    Box::into_raw(Box::new(response))
}
type DropIndexCallback = extern "C" fn(wrapper: *mut DropIndexResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn drop_index_async(
    client: *mut ClientWrapper,
    collectionname: *const c_char,
    name: *const c_char,
    request_id: i32,
    callback: DropIndexCallback,
) {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DropIndexResponseWrapper {
                success: false,
                error: error_msg,
                request_id
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DropIndexRequest {
        collectionname: c_char_to_str(collectionname),
        name: c_char_to_str(name),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DropIndexResponseWrapper {
            success: false,
            error: error_msg,
            request_id
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    handle.spawn(async move {
        let result = client.drop_index(request).await;

        let response = match result {
            Ok(_) => {
                DropIndexResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                    request_id
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Drop index failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DropIndexResponseWrapper {
                    success: false,
                    error: error_msg,
                    request_id
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_drop_index_response(response: *mut DropIndexResponseWrapper) {
    if response.is_null() {
        return;        
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
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
    request_id: i32
}
#[repr(C)]
pub struct AggregateResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn aggregate(
    client: *mut ClientWrapper,
    options: *mut AggregateRequestWrapper,
) -> *mut AggregateResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = AggregateResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client ) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = AggregateResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = AggregateRequest {
        collectionname: c_char_to_str(options.collectionname),
        aggregates: c_char_to_str(options.aggregates),
        queryas: c_char_to_str(options.queryas),
        hint: c_char_to_str(options.hint),
        explain: options.explain
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = AggregateResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.aggregate(request))
    });

    let response = match result {
        Ok(data) => {
            let results = CString::new(data.results).unwrap().into_raw();
            AggregateResponseWrapper {
                success: true,
                results,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options ) {
        Some( options ) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = AggregateResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some( client ) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = AggregateResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = AggregateRequest {
        collectionname: c_char_to_str(options.collectionname),
        aggregates: c_char_to_str(options.aggregates),
        queryas: c_char_to_str(options.queryas),
        hint: c_char_to_str(options.hint),
        explain: options.explain
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = AggregateResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    debug!("Rust: runtime.spawn");
    handle.spawn(async move {
        debug!("Rust: client.aggregate");
        let result = client.aggregate(request).await;
        let response = match result {
            Ok(data) => {
                let results = CString::new(data.results).unwrap().into_raw();
                AggregateResponseWrapper {
                    success: true,
                    results,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).results.is_null() {
            let _ = CString::from_raw((*response).results as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct CountRequestWrapper {
    collectionname: *const c_char,
    query: *const c_char,
    queryas: *const c_char,
    explain: bool,
    request_id: i32
}
#[repr(C)]
pub struct CountResponseWrapper {
    success: bool,
    result: i32,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn count(
    client: *mut ClientWrapper,
    options: *mut CountRequestWrapper,
) -> *mut CountResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = CountResponseWrapper {
                success: false,
                result: 0,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = CountResponseWrapper {
                success: false,
                result: 0,
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = CountRequest {
        collectionname: c_char_to_str(options.collectionname),
        query: c_char_to_str(options.query),
        queryas: c_char_to_str(options.queryas),
        explain: options.explain
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CountResponseWrapper {
            success: false,
            result: 0,
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }

    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.count(request))
    });

    let response = match result {
        Ok(data) => {
            let result = data.result;
            CountResponseWrapper {
                success: true,
                result,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = CountResponseWrapper {
                success: false,
                result: 0,
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = CountResponseWrapper {
                success: false,
                result: 0,
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = CountRequest {
        collectionname: c_char_to_str(options.collectionname),
        query: c_char_to_str(options.query),
        queryas: c_char_to_str(options.queryas),
        explain: options.explain
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = CountResponseWrapper {
            success: false,
            result: 0,
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.count(request).await;
        let response = match result {
            Ok(data) => {
                let result = data.result;
                CountResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
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
    request_id: i32
}
#[repr(C)]
pub struct DistinctResponseWrapper {
    success: bool,
    // results: *const c_char,
    results: *mut *const c_char,
    error: *const c_char,
    results_len: i32,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn distinct(
    client: *mut ClientWrapper,
    options: *mut DistinctRequestWrapper,
) -> *mut DistinctResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DistinctResponseWrapper {
                success: false,
                results: std::ptr::null_mut(),
                error: error_msg,
                results_len: 0,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DistinctResponseWrapper {
                success: false,
                results: std::ptr::null_mut(),
                error: error_msg,
                results_len: 0,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DistinctRequest {
        collectionname: c_char_to_str(options.collectionname),
        field: c_char_to_str(options.field),
        query: c_char_to_str(options.query),
        queryas: c_char_to_str(options.queryas),
        explain: options.explain,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DistinctResponseWrapper {
            success: false,
            results: std::ptr::null_mut(),
            error: error_msg,
            results_len: 0,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.distinct(request))
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
                error: std::ptr::null(),
                results_len: data.results.len().try_into().unwrap(),
                request_id: options.request_id,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("Distinct failed: {:?}", e))
                .unwrap()
                .into_raw();
            DistinctResponseWrapper {
                success: false,
                results: std::ptr::null_mut(),
                error: error_msg,
                results_len: 0,
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DistinctResponseWrapper {
                success: false,
                results: std::ptr::null_mut(),
                error: error_msg,
                results_len: 0,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DistinctResponseWrapper {
                success: false,
                results: std::ptr::null_mut(),
                error: error_msg,
                results_len: 0,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DistinctRequest {
        collectionname: c_char_to_str(options.collectionname),
        field: c_char_to_str(options.field),
        query: c_char_to_str(options.query),
        queryas: c_char_to_str(options.queryas),
        explain: options.explain,
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DistinctResponseWrapper {
            success: false,
            results: std::ptr::null_mut(),
            error: error_msg,
            results_len: 0,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.distinct(request).await;
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
                    error: std::ptr::null(),
                    results_len: data.results.len().try_into().unwrap(),
                    request_id,
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Distinct failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                DistinctResponseWrapper {
                    success: false,
                    results: std::ptr::null_mut(),
                    error: error_msg,
                    results_len: 0,
                    request_id,
                }
            }
        };

        callback(Box::into_raw(Box::new(response)));
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_distinct_response(response: *mut DistinctResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).results.is_null() {
            for i in 0..(*response).results_len {
                let c_str_ptr = *(*response).results.add(i as usize);
                if !c_str_ptr.is_null() {
                    let _ = CString::from_raw(c_str_ptr as *mut c_char); // Free the C string
                }
            }
            let _ = Box::from_raw((*response).results);
        }

        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}
#[repr(C)]
pub struct InsertOneRequestWrapper {
    collectionname: *const c_char,
    item: *const c_char,
    w: i32,
    j: bool,
    request_id: i32
}
#[repr(C)]
pub struct InsertOneResponseWrapper {
    success: bool,
    result: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_one(
    client: *mut ClientWrapper,
    options: *mut InsertOneRequestWrapper,
) -> *mut InsertOneResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = InsertOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = InsertOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = InsertOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        item: c_char_to_str(options.item),
        w: options.w,
        j: options.j,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.insert_one(request))
    });

    let response = match result {
        Ok(data) => {
            let result = CString::new(data.result).unwrap().into_raw();
            InsertOneResponseWrapper {
                success: true,
                result,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = InsertOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = InsertOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = InsertOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        item: c_char_to_str(options.item),
        w: options.w,
        j: options.j,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.insert_one(request).await;
        let response = match result {
            Ok(data) => {
                let result = CString::new(data.result).unwrap().into_raw();
                InsertOneResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id, 
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).result.is_null() {
            let _ = CString::from_raw((*response).result as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}
#[repr(C)]
pub struct InsertManyRequestWrapper {
    collectionname: *const c_char,
    items: *const c_char,
    w: i32,
    j: bool,
    skipresults: bool,
    request_id: i32
}
#[repr(C)]
pub struct InsertManyResponseWrapper {
    success: bool,
    results: *const c_char,
    error: *const c_char,
    request_id: i32,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_many(
    client: *mut ClientWrapper,
    options: *mut InsertManyRequestWrapper,
) -> *mut InsertManyResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = InsertManyResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = InsertManyResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = InsertManyRequest {
        collectionname: c_char_to_str(options.collectionname),
        items: c_char_to_str(options.items),
        w: options.w,
        j: options.j,
        skipresults: options.skipresults
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertManyResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.insert_many(request))
    });

    let response = match result {
        Ok(data) => {
            let results = CString::new(data.results).unwrap().into_raw();
            InsertManyResponseWrapper {
                success: true,
                results,
                error: std::ptr::null(),
                request_id: options.request_id,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("InsertMany failed: {:?}", e))
                .unwrap()
                .into_raw();
            InsertManyResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = InsertManyResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = InsertManyResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = InsertManyRequest {
        collectionname: c_char_to_str(options.collectionname),
        items: c_char_to_str(options.items),
        w: options.w,
        j: options.j,
        skipresults: options.skipresults
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertManyResponseWrapper {
            success: false,
            results: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.insert_many(request).await;
        let response = match result {
            Ok(data) => {
                let results = CString::new(data.results.clone()).unwrap().into_raw();
                InsertManyResponseWrapper {
                    success: true,
                    results,
                    error: std::ptr::null(),
                    request_id,
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("InsertMany failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                InsertManyResponseWrapper {
                    success: false,
                    results: std::ptr::null(),
                    error: error_msg,
                    request_id,
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
        if !(*response).results.is_null() {
            let _ = CString::from_raw((*response).results as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct UpdateOneRequestWrapper {
    collectionname: *const c_char,
    item: *const c_char,
    w: i32,
    j: bool,
    request_id: i32
}
#[repr(C)]
pub struct UpdateOneResponseWrapper {
    success: bool,
    result: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn update_one(
    client: *mut ClientWrapper,
    options: *mut UpdateOneRequestWrapper,
) -> *mut UpdateOneResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = UpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = UpdateOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        item: c_char_to_str(options.item),
        w: options.w,
        j: options.j
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.update_one(request))
    });

    let response = match result {
        Ok(data) => {
            let result = CString::new(data.result).unwrap().into_raw();
            UpdateOneResponseWrapper {
                success: true,
                result,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = UpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = UpdateOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        item: c_char_to_str(options.item),
        w: options.w,
        j: options.j
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.update_one(request).await;
        let response = match result {
            Ok(data) => {
                let result = CString::new(data.result).unwrap().into_raw();
                UpdateOneResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).result.is_null() {
            let _ = CString::from_raw((*response).result as *mut c_char);
        }
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
    request_id: i32
}
#[repr(C)]
pub struct InsertOrUpdateOneResponseWrapper {
    success: bool,
    result: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn insert_or_update_one(
    client: *mut ClientWrapper,
    options: *mut InsertOrUpdateOneRequestWrapper,
) -> *mut InsertOrUpdateOneResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = InsertOrUpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = InsertOrUpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    debug!("Rust: insert_or_update_one create request");

    trace!("Rust: parse collectionname");
    let collectionname = c_char_to_str(options.collectionname);
    trace!("Rust: parse uniqeness");
    let uniqeness = c_char_to_str(options.uniqeness);
    trace!("Rust: parse item");
    let item = c_char_to_str(options.item);
    trace!("Rust: parse w");
    let w = options.w;
    trace!("Rust: parse j");
    let j = options.j;
    let request = InsertOrUpdateOneRequest {
        collectionname,
        uniqeness,
        item,
        w,
        j
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOrUpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    debug!("Rust: run insert_or_update_one in runtime");
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.insert_or_update_one(request))
    });

    let response = match result {
        Ok(data) => {
            let result = CString::new(data).unwrap().into_raw();
            InsertOrUpdateOneResponseWrapper {
                success: true,
                result,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = InsertOrUpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = InsertOrUpdateOneResponseWrapper {
                success: false,
                result: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    debug!("Rust: insert_or_update_one_async create request");
    let request = InsertOrUpdateOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        uniqeness: c_char_to_str(options.uniqeness),
        item: c_char_to_str(options.item),
        w: options.w,
        j: options.j
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = InsertOrUpdateOneResponseWrapper {
            success: false,
            result: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.insert_or_update_one(request).await;

        let response = match result {
            Ok(data) => {
                let result = CString::new(data).unwrap().into_raw();
                InsertOrUpdateOneResponseWrapper {
                    success: true,
                    result,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).result.is_null() {
            let _ = CString::from_raw((*response).result as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DeleteOneRequestWrapper {
    collectionname: *const c_char,
    id: *const c_char,
    recursive: bool,
    request_id: i32
}
#[repr(C)]
pub struct DeleteOneResponseWrapper {
    success: bool,
    affectedrows: i32,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_one(
    client: *mut ClientWrapper,
    options: *mut DeleteOneRequestWrapper,
) -> *mut DeleteOneResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DeleteOneResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DeleteOneResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DeleteOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        id: c_char_to_str(options.id),
        recursive: options.recursive
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteOneResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.delete_one(request))
    });

    let response = match result {
        Ok(data) => {
            let affectedrows = data;
            DeleteOneResponseWrapper {
                success: true,
                affectedrows,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DeleteOneResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DeleteOneResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DeleteOneRequest {
        collectionname: c_char_to_str(options.collectionname),
        id: c_char_to_str(options.id),
        recursive: options.recursive
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteOneResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.delete_one(request).await;
        let response = match result {
            Ok(data) => {
                let affectedrows = data;
                DeleteOneResponseWrapper {
                    success: true,
                    affectedrows,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DeleteManyRequestWrapper {
    collectionname: *const c_char,
    query: *const c_char,
    recursive: bool,
    ids: *const *const c_char,
    request_id: i32
}
#[repr(C)]
pub struct DeleteManyResponseWrapper {
    success: bool,
    affectedrows: i32,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_many(
    client: *mut ClientWrapper,
    options: *mut DeleteManyRequestWrapper,
) -> *mut DeleteManyResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DeleteManyResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DeleteManyResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DeleteManyRequest {
        collectionname: c_char_to_str(options.collectionname),
        query: c_char_to_str(options.query),
        recursive: options.recursive,
        ids: {
            let mut ids = vec![];
            let mut i = 0;
            loop {
                let id = unsafe { *options.ids.add(i) };
                if id.is_null() {
                    break;
                }
                ids.push(c_char_to_str(id));
                i += 1;
            }
            ids
        }
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteManyResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.delete_many(request))
    });

    let response = match result {
        Ok(data) => {
            let affectedrows = data;
            DeleteManyResponseWrapper {
                success: true,
                affectedrows,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DeleteManyResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DeleteManyResponseWrapper {
                success: false,
                affectedrows: 0,
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DeleteManyRequest {
        collectionname: c_char_to_str(options.collectionname),
        query: c_char_to_str(options.query),
        recursive: options.recursive,
        ids: {
            let mut ids = vec![];
            let mut i = 0;
            loop {
                let id = unsafe { *options.ids.add(i) };
                if id.is_null() {
                    break;
                }
                let id = c_char_to_str(id);
                ids.push(id);
                i += 1;
            }
            ids
        }
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteManyResponseWrapper {
            success: false,
            affectedrows: 0,
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client.delete_many(request).await;
        let response = match result {
            Ok(data) => {
                let affectedrows = data;
                DeleteManyResponseWrapper {
                    success: true,
                    affectedrows,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct DownloadRequestWrapper {
    collectionname: *const c_char,
    id: *const c_char,
    folder: *const c_char,
    filename: *const c_char,
    request_id: i32
}
#[repr(C)]
pub struct DownloadResponseWrapper {
    success: bool,
    filename: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn download(
    client: *mut ClientWrapper,
    options: *mut DownloadRequestWrapper,
) -> *mut DownloadResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DownloadResponseWrapper {
                success: false,
                filename: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DownloadResponseWrapper {
                success: false,
                filename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let folder = c_char_to_str(options.folder);
    let filename = c_char_to_str(options.filename);
    let request = DownloadRequest {
        collectionname: c_char_to_str(options.collectionname),
        filename: c_char_to_str(options.filename),
        id: c_char_to_str(options.id)
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DownloadResponseWrapper {
            success: false,
            filename: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on( client.download(request, Some(&folder), Some(&filename)))
    });

    let response = match result {
        Ok(data) => {
            let filename = CString::new(data.filename).unwrap().into_raw();
            DownloadResponseWrapper {
                success: true,
                filename,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DownloadResponseWrapper {
                success: false,
                filename: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DownloadResponseWrapper {
                success: false,
                filename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let folder = c_char_to_str(options.folder);
    let filename = c_char_to_str(options.filename);
    let request = DownloadRequest {
        collectionname: c_char_to_str(options.collectionname),
        filename: c_char_to_str(options.filename),
        id: c_char_to_str(options.id)
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DownloadResponseWrapper {
            success: false,
            filename: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client
            .download(request, Some(&folder), Some(&filename))
            .await;

        let response = match result {
            Ok(data) => {
                let filename = CString::new(data.filename).unwrap().into_raw();
                DownloadResponseWrapper {
                    success: true,
                    filename,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).filename.is_null() {
            let _ = CString::from_raw((*response).filename as *mut c_char);
        }
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
    request_id: i32
}
#[repr(C)]
pub struct UploadResponseWrapper {
    success: bool,
    id: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn upload(
    client: *mut ClientWrapper,
    options: *mut UploadRequestWrapper,
) -> *mut UploadResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = UploadResponseWrapper {
                success: false,
                id: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UploadResponseWrapper {
                success: false,
                id: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let filepath = c_char_to_str(options.filepath);
    if filepath.is_empty() {
        let error_msg = CString::new("Filepath is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let filepath = filepath.to_string();
    debug!("upload: filepath: {}", filepath);
    let filename = c_char_to_str(options.filename);
    if filename.is_empty() {
        let error_msg = CString::new("Filename is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }

    let request = UploadRequest {
        filename: filename.to_string(),
        mimetype: c_char_to_str(options.mimetype),
        metadata: c_char_to_str(options.metadata),
        collectionname: c_char_to_str(options.collectionname)
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    debug!("upload: runtime.block_on");
    let result = tokio::task::block_in_place(|| {
        // let c = client.as_mut().unwrap();
        // c.upload(request).await
        // client.as_mut().unwrap().upload(request, &filepath).await
        let handle = client.get_runtime_handle();
        handle.block_on( client.upload(request, &filepath))
    });

    let response = match result {
        Ok(data) => {
            let id = CString::new(data.id).unwrap().into_raw();
            UploadResponseWrapper {
                success: true,
                id,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
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
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = UploadResponseWrapper {
                success: false,
                id: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UploadResponseWrapper {
                success: false,
                id: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let filepath = c_char_to_str(options.filepath);
    if filepath.is_empty() {
        let error_msg = CString::new("Filepath is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let filepath = filepath.to_string();
    debug!("upload_async: filepath: {}", filepath);
    let filename = c_char_to_str(options.filename);
    if filename.is_empty() {
        let error_msg = CString::new("Filename is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }

    let request = UploadRequest {
        filename: filename.to_string(),
        mimetype: c_char_to_str(options.mimetype),
        metadata: c_char_to_str(options.metadata),
        collectionname: c_char_to_str(options.collectionname)
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    debug!("upload_async: runtime.spawn");
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        debug!("upload_async: call client.upload");
        let result = client.upload(request, &filepath).await;

        debug!("upload_async: call client.upload done");
        let response = match result {
            Ok(data) => {
                let id = CString::new(data.id).unwrap().into_raw();
                UploadResponseWrapper {
                    success: true,
                    id,
                    error: std::ptr::null(),
                    request_id,
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
                    request_id,
                }
            }
        };
        debug!("upload_async: call callback with response");
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).id.is_null() {
            let _ = CString::from_raw((*response).id as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct WatchRequestWrapper {
    collectionname: *const c_char,
    paths: *const c_char,
    request_id: i32
}
#[repr(C)]
pub struct WatchResponseWrapper {
    success: bool,
    watchid: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn watch(
    client: *mut ClientWrapper,
    options: *mut WatchRequestWrapper,
) -> *mut WatchResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = WatchResponseWrapper {
                success: false,
                watchid: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = WatchResponseWrapper {
                success: false,
                watchid: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    // let events = &client_wrapper.events;
    let paths = c_char_to_str(options.paths);
    let paths = paths.split(",").map(|s| s.to_string()).collect();
    let request = WatchRequest {
        collectionname: c_char_to_str(options.collectionname),
        paths,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = WatchResponseWrapper {
            success: false,
            watchid: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let request_id = options.request_id;
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(
        client
            .watch(
                request,
                Box::new(move |event: WatchEvent| {
                    // convert event to json
                    // let event = serde_json::to_string(&event).unwrap();
                    // let c_event = std::ffi::CString::new(event).unwrap();
                    debug!("watch: event: {:?}", event);
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
        )
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
                request_id,
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
                request_id,
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
    let watchid = c_char_to_str(watchid);
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
                        document,
                        request_id: 0
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
        if !(*response).id.is_null() {
            let _ = CString::from_raw((*response).id as *mut c_char);
        }
        if !(*response).operation.is_null() {
            let _ = CString::from_raw((*response).operation as *mut c_char);
        }
        if !(*response).document.is_null() {
            let _ = CString::from_raw((*response).document as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

type WatchEventCallback = extern "C" fn(*mut WatchEventWrapper);
type WatchCallback = extern "C" fn(wrapper: *mut WatchResponseWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn watch_async_async(
    client: *mut ClientWrapper,
    options: *mut WatchRequestWrapper,
    callback: WatchCallback,
    // event_callback: extern "C" fn(*mut WatchEventWrapper),
    event_callback: WatchEventCallback,
    // event_callback: extern "C" fn(*const c_char),
) {
    debug!("watch_async_async");
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = WatchResponseWrapper {
                success: false,
                watchid: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = WatchResponseWrapper {
                success: false,
                watchid: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let paths = c_char_to_str(options.paths);
    let paths = paths.split(",").map(|s| s.to_string()).collect();
    let request = WatchRequest {
        collectionname: c_char_to_str(options.collectionname),
        paths,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = WatchResponseWrapper {
            success: false,
            watchid: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    debug!("watch_async: runtime.spawn");
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        debug!("watch_async: call client.watch");
        let result = client
            .watch(
                request,
                Box::new(move |_event: WatchEvent| {
                    debug!("watch_async: spawn new task, to call event_callback");
                    trace!("watch_async: call event_callback");
                    let id = CString::new(_event.id).unwrap().into_raw();
                    let operation = CString::new(_event.operation).unwrap().into_raw();
                    let document = CString::new(_event.document).unwrap().into_raw();
                    let event = Box::into_raw(Box::new(WatchEventWrapper {
                        id,
                        operation,
                        document,
                        request_id
                    }));

                    event_callback(event);
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
                    request_id,
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
                    request_id,
                }
            }
        };

        debug!("watch_async: call callback with response");
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).watchid.is_null() {
            let _ = CString::from_raw((*response).watchid as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct UnWatchResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn unwatch(
    client: *mut ClientWrapper,
    watchid: *const c_char
) -> *mut UnWatchResponseWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UnWatchResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let watchid = c_char_to_str(watchid);
    if watchid.is_empty() {
        let error_msg = CString::new("Watchid is required").unwrap().into_raw();
        let response = UnWatchResponseWrapper {
            success: false,
            error: error_msg,
            request_id: 0,
        };
        return Box::into_raw(Box::new(response));
    }
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UnWatchResponseWrapper {
            success: false,
            error: error_msg,
            request_id: 0
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    trace!("watchid: {:?}", watchid);
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on( client.unwatch(&watchid))
    });
    trace!("completed, parsing result");
    match result {
        Ok(_) => {
            let response = UnWatchResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id: 0
            };
            debug!("Unwatch success");
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Unwatch failed: {:?}", e))
                .unwrap()
                .into_raw();
            debug!("Unwatch failed: {:?}", error_msg);
            let response = UnWatchResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0
            };
            Box::into_raw(Box::new(response))
        }
    }    
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn unwatch_async(
    client: *mut ClientWrapper,
    watchid: *const c_char,
    request_id: i32,
    callback: extern "C" fn(*mut UnWatchResponseWrapper),
) {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UnWatchResponseWrapper {
                success: false,
                error: error_msg,
                request_id,
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let watchid = c_char_to_str(watchid);
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UnWatchResponseWrapper {
            success: false,
            error: error_msg,
            request_id,
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    handle.spawn(async move {
        let result = client.unwatch(&watchid).await;
        match result {
            Ok(_) => {
                let response = UnWatchResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                    request_id,
                };
                callback(Box::into_raw(Box::new(response)));
            }
            Err(e) => {
                let error_msg = CString::new(format!("Unwatch failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                let response = UnWatchResponseWrapper {
                    success: false,
                    error: error_msg,
                    request_id,
                };
                callback(Box::into_raw(Box::new(response)));
            }
        }
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_unwatch_response(response: *mut UnWatchResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}


#[repr(C)]
pub struct RegisterQueueRequestWrapper {
    queuename: *const c_char,
    request_id: i32
}
#[repr(C)]
pub struct RegisterQueueResponseWrapper {
    success: bool,
    queuename: *const c_char,
    error: *const c_char,
    request_id: i32
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn register_queue(
    client: *mut ClientWrapper,
    options: *mut RegisterQueueRequestWrapper,
) -> *mut RegisterQueueResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = RegisterQueueResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = RegisterQueueResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    // let events = &client_wrapper.events;
    let request = RegisterQueueRequest {
        queuename: c_char_to_str(options.queuename),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = RegisterQueueResponseWrapper {
            success: false,
            queuename: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(
        client
            .register_queue(
                request,
                Box::new(move |event: QueueEvent| {
                    trace!("queue: event: {:?}", event);
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
        )
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
                request_id: options.request_id,
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
                request_id: options.request_id,
            }
        }
    };
    Box::into_raw(Box::new(response))
}

type QueueEventCallback = extern "C" fn(*mut QueueEventWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn register_queue_async(
    client: *mut ClientWrapper,
    options: *mut RegisterQueueRequestWrapper,
    event_callback: QueueEventCallback,
) -> *mut RegisterQueueResponseWrapper {
    debug!("register_queue_async");
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = RegisterQueueResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response))
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = RegisterQueueResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response))
        }
    };
    let client = client_wrapper.client.clone();
    let request = RegisterQueueRequest {
        queuename: c_char_to_str(options.queuename),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = RegisterQueueResponseWrapper {
            success: false,
            queuename: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response))
    }
    let client = client.unwrap();
    debug!("register_queue_async: runtime.spawn");
    let request_id = options.request_id;
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(
        client
            .register_queue(
                request,
                Box::new(move |event: QueueEvent| {
                    debug!("register_queue_async: spawn new task, to call event_callback");
                    trace!("register_queue_async: call event_callback");
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
                        request_id
                    });
                    event_callback(Box::into_raw(event));
                }),
            )
        )
    });

    debug!("register_queue_async: parse result");
    let response = match result {
        Ok(data) => {
            let queuename = CString::new(data).unwrap().into_raw();
            RegisterQueueResponseWrapper {
                success: true,
                queuename,
                error: std::ptr::null(),
                request_id: options.request_id,
            }
        }
        Err(e) => {
            let error_msg = CString::new(format!("RegisterQueue failed: {:?}", e))
                .unwrap()
                .into_raw();
            RegisterQueueResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).queuename.is_null() {
            let _ = CString::from_raw((*response).queuename as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}


#[repr(C)]
pub struct RegisterExchangeRequestWrapper {
    exchangename: *const c_char,
    algorithm: *const c_char,
    routingkey: *const c_char,
    addqueue: bool,
    request_id: i32
}
#[repr(C)]
pub struct RegisterExchangeResponseWrapper {
    success: bool,
    queuename: *const c_char,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn register_exchange (
    client: *mut ClientWrapper,
    options: *mut RegisterExchangeRequestWrapper,
) -> *mut RegisterExchangeResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = RegisterExchangeResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = RegisterExchangeResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = RegisterExchangeRequest {
        exchangename: c_char_to_str(options.exchangename),
        algorithm: c_char_to_str(options.algorithm),
        routingkey: c_char_to_str(options.routingkey),
        addqueue: options.addqueue,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = RegisterExchangeResponseWrapper {
            success: false,
            queuename: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(
        client
            .register_exchange(request,
                Box::new(move |event: QueueEvent| {
                    trace!("exchange: event: {:?}", event);
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
        )
    });

    let response = match result {
        Ok(data) => {
            let queuename = CString::new(data).unwrap().into_raw();
            RegisterExchangeResponseWrapper {
                success: true,
                queuename,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
            }
        }
    };

    Box::into_raw(Box::new(response))
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn register_exchange_async(
    client: *mut ClientWrapper,
    options: *mut RegisterExchangeRequestWrapper,
    event_callback: QueueEventCallback,
) -> *mut RegisterExchangeResponseWrapper {
    debug!("register_exchange_async");
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = RegisterExchangeResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: 0,
            };
            return Box::into_raw(Box::new(response))
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = RegisterExchangeResponseWrapper {
                success: false,
                queuename: std::ptr::null(),
                error: error_msg,
                request_id: options.request_id,
            };
            return Box::into_raw(Box::new(response))
        }
    };
    let client = client_wrapper.client.clone();
    let request = RegisterExchangeRequest {
        exchangename: c_char_to_str(options.exchangename),
        algorithm: c_char_to_str(options.algorithm),
        routingkey: c_char_to_str(options.routingkey),
        addqueue: options.addqueue,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = RegisterExchangeResponseWrapper {
            success: false,
            queuename: std::ptr::null(),
            error: error_msg,
            request_id: options.request_id,
        };
        return Box::into_raw(Box::new(response))
    }
    let client = client.unwrap();
    debug!("register_exchange_async: runtime.spawn");
    let request_id = options.request_id;
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(
        client
            .register_exchange(request,
                Box::new(move |event: QueueEvent| {
                    debug!("register_exchange_async: spawn new task, to call event_callback");
                    trace!("register_exchange_async: call event_callback");
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
                        request_id
                    });
                    event_callback(Box::into_raw(event));
                }),

            )
        )
    });

    debug!("register_exchange_async: parse result");
    let response = match result {
        Ok(data) => {
            let queuename = CString::new(data).unwrap().into_raw();
            RegisterExchangeResponseWrapper {
                success: true,
                queuename,
                error: std::ptr::null(),
                request_id: options.request_id,
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
                request_id: options.request_id,
            }
        }
    };
    Box::into_raw(Box::new(response))
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_register_exchange_response(response: *mut RegisterExchangeResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).queuename.is_null() {
            let _ = CString::from_raw((*response).queuename as *mut c_char);
        }
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
    request_id: i32,
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
            request_id: 0,
        }
     }
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn next_queue_event (
    queuename: *const c_char,
) -> *mut QueueEventWrapper {
    debug!("unwrap watchid");
    let queuename = c_char_to_str(queuename);
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
                        request_id: 0
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
        if !(*response).queuename.is_null() {
            let _ = CString::from_raw((*response).queuename as *mut c_char);
        }
        if !(*response).correlation_id.is_null() {
            let _ = CString::from_raw((*response).correlation_id as *mut c_char);
        }
        if !(*response).replyto.is_null() {
            let _ = CString::from_raw((*response).replyto as *mut c_char);
        }
        if !(*response).routingkey.is_null() {
            let _ = CString::from_raw((*response).routingkey as *mut c_char);
        }
        if !(*response).exchangename.is_null() {
            let _ = CString::from_raw((*response).exchangename as *mut c_char);
        }
        if !(*response).data.is_null() {
            let _ = CString::from_raw((*response).data as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
pub struct QueueMessageRequestWrapper {
    queuename: *const c_char,
    correlation_id: *const c_char,
    replyto: *const c_char,
    routingkey: *const c_char,
    exchangename: *const c_char,
    data: *const c_char,
    striptoken: bool,
    expiration: i32,
}
#[repr(C)]
pub struct QueueMessageResponseWrapper {
    success: bool,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn queue_message(
    client: *mut ClientWrapper,
    options: *mut QueueMessageRequestWrapper,
) -> *mut QueueMessageResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = QueueMessageResponseWrapper {
                success: false,
                error: error_msg,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = QueueMessageResponseWrapper {
                success: false,
                error: error_msg,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = QueueMessageRequest {
        queuename: c_char_to_str(options.queuename),
        correlation_id: c_char_to_str(options.correlation_id),
        replyto: c_char_to_str(options.replyto),
        routingkey: c_char_to_str(options.routingkey),
        exchangename: c_char_to_str(options.exchangename),
        data: c_char_to_str(options.data),
        striptoken: options.striptoken,
        expiration: options.expiration,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = QueueMessageResponseWrapper {
            success: false,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(
        client
            .queue_message(request)
            )
    });
    match result {
        Ok(_) => {
            let response = QueueMessageResponseWrapper {
                success: true,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
            

        }
        Err(e) => {
            let error_msg = CString::new(format!("Queue message failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = QueueMessageResponseWrapper {
                success: false,
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_queue_message_response(response: *mut QueueMessageResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
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
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UnRegisterQueueResponseWrapper {
                success: false,
                error: error_msg,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let queuename = c_char_to_str(queuename);
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UnRegisterQueueResponseWrapper {
            success: false,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on( client.unregister_queue(&queuename))
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}



#[repr(C)]
#[derive(Debug, Clone)]
pub struct WorkitemFileWrapper {
    filename: *const c_char,
    id: *const c_char,
    compressed: bool,
    // file: vec::Vec<u8>,
}
impl WorkitemFileWrapper {
    pub fn new(filename: &str, id: &str, compressed: bool) -> Self {
        trace!("filename: {:?}", filename);
        let filename = CString::new(filename).unwrap().into_raw();
        trace!("filename: {:?}", filename);
        trace!("id: {:?}", id);
        let id = CString::new(id).unwrap().into_raw();
        trace!("id: {:?}", id);
        // let file: Vec<u8> = Vec::new();
        WorkitemFileWrapper {
            filename,
            id,
            // file,
            compressed,
        }
        // let c_filename = CString::new(filename).unwrap();
        // let c_id = CString::new(id).unwrap();
        
        // WorkitemFileWrapper {
        //     filename: c_filename.as_ptr(),
        //     id: c_id.as_ptr(),
        //     compressed,
        // }
    }
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
    files_len: i32,
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
impl WorkitemWrapper {
    #[tracing::instrument(skip_all)]
    pub fn as_workitem(&self) -> Workitem {
        let files_len = self.files_len;
        let mut files: Vec<WorkitemFile> = vec![];
        if files_len > 0 {
            let _files = unsafe { &*self.files };
            let _files = unsafe { std::slice::from_raw_parts(_files, files_len.try_into().unwrap()) };
            files = _files.iter().map(|f| {
                let file = unsafe { &**f };
                WorkitemFile {
                    filename: c_char_to_str(file.filename),
                    id: c_char_to_str(file.id),
                    ..Default::default()
                    // compressed: file.compressed,
                    // file: file.file.clone(),
                }
            }).collect();
        }
        trace!("nextrun: {:?}", self.nextrun);
        // convert self.nextrun to std::time::SystemTime
        let mut nextrun = Some(Timestamp::from(
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.nextrun)
        ));
        trace!("nextrun: {:?}", nextrun);
        if self.nextrun == 0 {
            nextrun = None;
        }
        trace!("lastrun: {:?}", self.lastrun);
        // convert self.lastrun to std::time::SystemTime
        let mut lastrun = Some(Timestamp::from(
            std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(self.lastrun)
        ));
        trace!("lastrun: {:?}", lastrun);
        if self.lastrun == 0 {
            lastrun = None;
        }    
        Workitem {
            id: c_char_to_str(self.id),
            name: c_char_to_str(self.name),
            payload: c_char_to_str(self.payload),
            priority: self.priority,
            nextrun,
            lastrun,
            files,
            state: c_char_to_str(self.state),
            wiq: c_char_to_str(self.wiq),
            wiqid: c_char_to_str(self.wiqid),
            retries: self.retries,
            username: c_char_to_str(self.username),
            success_wiqid: c_char_to_str(self.success_wiqid),
            failed_wiqid: c_char_to_str(self.failed_wiqid),
            success_wiq: c_char_to_str(self.success_wiq),
            failed_wiq: c_char_to_str(self.failed_wiq),
            errormessage: c_char_to_str(self.errormessage),
            errorsource: c_char_to_str(self.errorsource),
            errortype: c_char_to_str(self.errortype),
        }
    }
}
#[tracing::instrument(skip_all)]
pub fn wrap_workitem(workitem: Workitem ) -> WorkitemWrapper {
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
        // trace!("parse workitem file: {:?}", f);
        // let filename = CString::new(f.filename.clone()).unwrap().into_raw();
        // let id = CString::new(f.id.clone()).unwrap().into_raw();
        // // let file: Vec<u8> = Vec::new();
        // let compressed = f.compressed;
        // let file = Box::into_raw(Box::new(WorkitemFileWrapper {
        //     filename,
        //     id,
        //     // file,
        //     compressed,
        // }));
        let file = Box::into_raw(Box::new(WorkitemFileWrapper::new(&f.filename, &f.id, f.compressed)));
        files.push(file);
    }
    trace!("files: {:?} at {:?}", files.len(), files);
    trace!("read nextrun");
    let nextrun = workitem.nextrun.map(|t| t.seconds as u64).unwrap_or(0);
    trace!("nextrun: {:?}", nextrun);
    let lastrun = workitem.lastrun.map(|t| t.seconds as u64).unwrap_or(0);
    trace!("lastrun: {:?}", lastrun);
    let _files = files.as_ptr();
    trace!("files: {:?}", files);
    let files_len = workitem.files.len() as i32;
    trace!("files_len: {:?}", files_len);
    let workitem = WorkitemWrapper {
        id,
        name,
        payload,
        priority: workitem.priority,
        nextrun,
        lastrun,
        files: _files,
        files_len,
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
    trace!("forget files");
    std::mem::forget(files);
    trace!("return workitem");
    workitem
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
    files_len: i32,
    request_id: i32
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PushWorkitemResponseWrapper {
    success: bool,
    error: *const c_char,
    workitem: *const WorkitemWrapper,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn push_workitem(
    client: *mut ClientWrapper,
    options: *mut PushWorkitemRequestWrapper,
) -> *mut PushWorkitemResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = PushWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: 0
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = PushWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let files_len = options.files_len;
    debug!("files_len: {:?}", files_len);
    let mut files: Vec<WorkitemFile> = vec![];
    if files_len > 0 {
        debug!("get files of options");
        let _files = unsafe { &*options.files };
        debug!("slice files");
        let _files = unsafe { std::slice::from_raw_parts(_files, files_len.try_into().unwrap()) };
        debug!("loop files");
        files = _files.iter().map(|f| {
            debug!("process a file");
            let file = unsafe { &**f };
            debug!("create WorkitemFile instance");
            let filename = c_char_to_str(file.filename);
            trace!("filename: {:?}", filename);
            let id = c_char_to_str(file.id);
            trace!("id: {:?}", id);
            trace!("compressed: {:?}", file.compressed);
            WorkitemFile {
                filename,
                id,
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
    if options.nextrun == 0 {
        nextrun = None;
    }
    let request = PushWorkitemRequest {
        wiq: c_char_to_str(options.wiq),
        wiqid: c_char_to_str(options.wiqid),
        name: c_char_to_str(options.name),
        payload: c_char_to_str(options.payload),
        nextrun,
        success_wiqid: c_char_to_str(options.success_wiqid),
        failed_wiqid: c_char_to_str(options.failed_wiqid),
        success_wiq: c_char_to_str(options.success_wiq),
        failed_wiq: c_char_to_str(options.failed_wiq),
        priority: options.priority,
        files,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PushWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
            request_id: options.request_id
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(client
            .push_workitem(request)
            )
    });

    match result {
        Ok(resp) => {
            Box::into_raw(Box::new(match resp.workitem {
                Some(workitem) => {
                    let workitem = wrap_workitem(workitem);
                    PushWorkitemResponseWrapper {
                        success: true,
                        error: std::ptr::null(),
                        workitem: Box::into_raw(Box::new(workitem)),
                        request_id: options.request_id
                    }
                }
                None => {
                    let error_msg = CString::new("Push workitem failed: workitem not found").unwrap().into_raw();
                    PushWorkitemResponseWrapper {
                        success: false,
                        error: error_msg,
                        workitem: std::ptr::null(),
                        request_id: options.request_id
                    }
                }
            }))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Push workitem failed: {:?}", e))
                .unwrap()
                .into_raw();
            Box::into_raw(Box::new(PushWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            }))
        }
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn push_workitem_async( 
    client: *mut ClientWrapper,
    options: *mut PushWorkitemRequestWrapper,
    callback: extern "C" fn(*mut PushWorkitemResponseWrapper),
) {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = PushWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: 0
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = PushWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let files_len = options.files_len;
    debug!("files_len: {:?}", files_len);
    let mut files: Vec<WorkitemFile> = vec![];
    if files_len > 0 {
        debug!("get files of options");
        let _files = unsafe { &*options.files };
        debug!("slice files");
        let _files = unsafe { std::slice::from_raw_parts(_files, files_len.try_into().unwrap()) };
        debug!("loop files");
        files = _files.iter().map(|f| {
            debug!("process a file");
            let file = unsafe { &**f };
            debug!("create WorkitemFile instance 2");
            let filename = c_char_to_str(file.filename);
            debug!("filename: {:?}", filename);
            let id = c_char_to_str(file.id);
            debug!("id: {:?}", id);
            let compressed = file.compressed;
            debug!("compressed: {:?}", compressed); 
            WorkitemFile {
                filename: c_char_to_str(file.filename),
                id: c_char_to_str(file.id),
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
    if options.nextrun == 0 {
        nextrun = None;
    }
    let request = PushWorkitemRequest {
        wiq: c_char_to_str(options.wiq),
        wiqid: c_char_to_str(options.wiqid),
        name: c_char_to_str(options.name),
        payload: c_char_to_str(options.payload),
        nextrun,
        success_wiqid: c_char_to_str(options.success_wiqid),
        failed_wiqid: c_char_to_str(options.failed_wiqid),
        success_wiq: c_char_to_str(options.success_wiq),
        failed_wiq: c_char_to_str(options.failed_wiq),
        priority: options.priority,
        files,
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PushWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
            request_id: options.request_id
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client
            .push_workitem(request)
            .await;
        let response = match result {
            Ok(resp) => {
                match resp.workitem {
                    Some(workitem) => {
                        let workitem = wrap_workitem(workitem);
                        PushWorkitemResponseWrapper {
                            success: true,
                            error: std::ptr::null(),
                            workitem: Box::into_raw(Box::new(workitem)),
                            request_id,
                        }
                    }
                    None => {
                        let error_msg = CString::new("Push workitem failed: workitem not found").unwrap().into_raw();
                        PushWorkitemResponseWrapper {
                            success: false,
                            error: error_msg,
                            workitem: std::ptr::null(),
                            request_id,
                        }
                    }
                }
            }
            Err(e) => {
                let error_msg = CString::new(format!("Push workitem failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                PushWorkitemResponseWrapper {
                    success: false,
                    error: error_msg,
                    workitem: std::ptr::null(),
                    request_id,
                }
            }
        };
        let response = Box::into_raw(Box::new(response));
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
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).workitem.is_null() {
            free_workitem((*response).workitem as *mut WorkitemWrapper);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PopWorkitemRequestWrapper {
    wiq: *const c_char,
    wiqid: *const c_char,
    request_id: i32
    // includefiles: bool,
    // compressed: bool,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PopWorkitemResponseWrapper {
    success: bool,
    error: *const c_char,
    workitem: *const WorkitemWrapper,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn pop_workitem (
    client: *mut ClientWrapper,
    options: *mut PopWorkitemRequestWrapper,
    downloadfolder: *const c_char,
) -> *mut PopWorkitemResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = PopWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: 0
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = PopWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = PopWorkitemRequest {
        wiq: c_char_to_str(options.wiq),
        wiqid: c_char_to_str(options.wiqid),
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PopWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
            request_id: options.request_id
        };
        return Box::into_raw(Box::new(response));
    }
    let downloadfolder = c_char_to_str(downloadfolder);
    let mut _downloadfolder = Some(downloadfolder.as_str());
    if downloadfolder.is_empty() {
        _downloadfolder = None;
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(client
            .pop_workitem(request, _downloadfolder)
            )
    });
    debug!("pop_workitem completed, parse result");

    match result {
        Ok(data) => {
            let workitem = match data.workitem {
                Some(workitem) => {
                    let workitem = wrap_workitem(workitem);
                    trace!("wrap workitem");
                    Box::into_raw(Box::new(workitem))
                },
                None => {
                    std::ptr::null()
                }
            };
            Box::into_raw(Box::new(PopWorkitemResponseWrapper {
                success: true,
                error: std::ptr::null(),
                workitem,
                request_id: options.request_id
            }))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Pop workitem failed: {:?}", e))
                .unwrap()
                .into_raw();
            Box::into_raw(Box::new(PopWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            }))
        }
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn pop_workitem_async (
    client: *mut ClientWrapper,
    options: *mut PopWorkitemRequestWrapper,
    downloadfolder: *const c_char,
    callback: extern "C" fn(*mut PopWorkitemResponseWrapper),
) {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = PopWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: 0
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = PopWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = PopWorkitemRequest {
        wiq: c_char_to_str(options.wiq),
        wiqid: c_char_to_str(options.wiqid),
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = PopWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
            request_id: options.request_id
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let downloadfolder = c_char_to_str(downloadfolder);
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let mut _downloadfolder = Some(downloadfolder.as_str());
        if downloadfolder.is_empty() {
            _downloadfolder = None;
        }
    
        let result = client
            .pop_workitem(request, _downloadfolder)
            .await;
        let response = match result {
            Ok(data) => {
                let workitem = match data.workitem {
                    Some(workitem) => {
                        let workitem = wrap_workitem(workitem);
                        trace!("wrap workitem");
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
                    request_id: request_id
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
                    request_id: request_id
                };
                Box::into_raw(Box::new(response))
            }
        };
        trace!("callback with result");
        callback(response);
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_pop_workitem_response(response: *mut PopWorkitemResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).workitem.is_null() {
            free_workitem((*response).workitem as *mut WorkitemWrapper);
        }
        let _ = Box::from_raw(response);
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_workitem_file(file: *mut WorkitemFileWrapper) {
    if file.is_null() {
        return;
    }
    unsafe {
        if !(*file).filename.is_null() {
            let _ = CString::from_raw((*file).filename as *mut c_char);
        }
        if !(*file).id.is_null() {
            let _ = CString::from_raw((*file).id as *mut c_char);
        }
        let _ = Box::from_raw(file);
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_workitem(workitem: *mut WorkitemWrapper) {
    if workitem.is_null() {
        return;
    }
    unsafe {
        if !(*workitem).id.is_null() {
            let _ = CString::from_raw((*workitem).id as *mut c_char);
        }
        if !(*workitem).name.is_null() {
            let _ = CString::from_raw((*workitem).name as *mut c_char);
        }
        if !(*workitem).payload.is_null() {
            let _ = CString::from_raw((*workitem).payload as *mut c_char);
        }
        if !(*workitem).state.is_null() {
            let _ = CString::from_raw((*workitem).state as *mut c_char);
        }
        if !(*workitem).wiq.is_null() {
            let _ = CString::from_raw((*workitem).wiq as *mut c_char);
        }
        if !(*workitem).wiqid.is_null() {
            let _ = CString::from_raw((*workitem).wiqid as *mut c_char);
        }
        if !(*workitem).username.is_null() {
            let _ = CString::from_raw((*workitem).username as *mut c_char);
        }
        if !(*workitem).success_wiqid.is_null() {
            let _ = CString::from_raw((*workitem).success_wiqid as *mut c_char);
        }
        if !(*workitem).failed_wiqid.is_null() {
            let _ = CString::from_raw((*workitem).failed_wiqid as *mut c_char);
        }
        if !(*workitem).success_wiq.is_null() {
            let _ = CString::from_raw((*workitem).success_wiq as *mut c_char);
        }
        if !(*workitem).failed_wiq.is_null() {
            let _ = CString::from_raw((*workitem).failed_wiq as *mut c_char);
        }
        if !(*workitem).errormessage.is_null() {
            let _ = CString::from_raw((*workitem).errormessage as *mut c_char);
        }
        if !(*workitem).errorsource.is_null() {
            let _ = CString::from_raw((*workitem).errorsource as *mut c_char);
        }
        if !(*workitem).errortype.is_null() {
            let _ = CString::from_raw((*workitem).errortype as *mut c_char);
        }

        // Free the array of `WorkitemFileWrapper` pointers if it exists
        if !(*workitem).files.is_null() && (*workitem).files_len > 0 {
            for i in 0..(*workitem).files_len as isize {
                let file_ptr = *(*workitem).files.offset(i);
                if !file_ptr.is_null() {
                    free_workitem_file(file_ptr as *mut WorkitemFileWrapper);
                }
            }
            // Free the array of pointers itself
            let _ = Box::from_raw((*workitem).files as *mut *const WorkitemFileWrapper);
        }
        let _ = Box::from_raw(workitem); // Take ownership to deallocate
    }
}


#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn pop_workitem2_async (
    _client: *mut ClientWrapper,
    _options: *mut PopWorkitemRequestWrapper,
    _downloadfolder: *const c_char,
    request_id: i32,
    callback: extern "C" fn(*mut PopWorkitemResponseWrapper),
) {
    callback(Box::into_raw(Box::new(PopWorkitemResponseWrapper {
        success: true,
        error: std::ptr::null(),
        workitem: std::ptr::null(),
        request_id: request_id
    })));
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct UpdateWorkitemRequestWrapper {
    workitem: *const WorkitemWrapper,
    ignoremaxretries: bool,    
    files: *const *const WorkitemFileWrapper,
    files_len: i32,
    request_id: i32
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UpdateWorkitemResponseWrapper {
    success: bool,
    error: *const c_char,
    workitem: *const WorkitemWrapper,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn update_workitem (
    client: *mut ClientWrapper,
    options: *mut UpdateWorkitemRequestWrapper,
) -> *mut UpdateWorkitemResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = UpdateWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: 0
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UpdateWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            };
            return Box::into_raw(Box::new(response));
        }
    };
    trace!("grab references");
    let client = client_wrapper.client.clone();
    let files_len = options.files_len;
    debug!("files_len: {:?}", files_len);
    let mut files: Vec<WorkitemFile> = vec![];
    if files_len > 0 {
        debug!("get files of options");
        let _files = unsafe { &*options.files };
        debug!("slice files");
        let _files = unsafe { std::slice::from_raw_parts(_files, files_len.try_into().unwrap()) };
        debug!("loop files");
        files = _files.iter().map(|f| {
            debug!("process a file");
            let file = unsafe { &**f };
            debug!("create WorkitemFile instance 2");
            let filename = c_char_to_str(file.filename);
            debug!("filename: {:?}", filename);
            let id = c_char_to_str(file.id);
            debug!("id: {:?}", id);
            let compressed = file.compressed;
            debug!("compressed: {:?}", compressed); 
            WorkitemFile {
                filename: c_char_to_str(file.filename),
                id: c_char_to_str(file.id),
                compressed: file.compressed,
                ..Default::default()
                // file: file.file.clone(),
            }
        }).collect();
    }
    debug!("unwrap workitem");
    let workitem = unsafe { &*options.workitem };
    debug!("convert workitem wrapper to workitem");
    let workitem = workitem.as_workitem();
    let request = UpdateWorkitemRequest {
        workitem: Some(workitem),
        ignoremaxretries: options.ignoremaxretries,
        files,
    };
    
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UpdateWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
            request_id: options.request_id
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(client
            .update_workitem(request)
            )
    });

    match result {
        Ok(resp) => {
            Box::into_raw(Box::new(match resp.workitem {
                Some(workitem) => {
                    let workitem = wrap_workitem(workitem);
                    UpdateWorkitemResponseWrapper {
                        success: true,
                        error: std::ptr::null(),
                        workitem: Box::into_raw(Box::new(workitem)),
                        request_id: options.request_id
                    }
                }
                None => {
                    let error_msg = CString::new("Update workitem failed: workitem not found").unwrap().into_raw();
                    UpdateWorkitemResponseWrapper {
                        success: false,
                        error: error_msg,
                        workitem: std::ptr::null(),
                        request_id: options.request_id
                    }
                }
            }))            
        }
        Err(e) => {
            let error_msg = CString::new(format!("Update workitem failed: {:?}", e))
                .unwrap()
                .into_raw();
            Box::into_raw(Box::new(UpdateWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            }))
        }
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn update_workitem_async (
    client: *mut ClientWrapper,
    options: *mut UpdateWorkitemRequestWrapper,
    callback: extern "C" fn(*mut UpdateWorkitemResponseWrapper),
) {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = UpdateWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: 0
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = UpdateWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                workitem: std::ptr::null(),
                request_id: options.request_id
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    trace!("grab references");
    let client = client_wrapper.client.clone();
    let files_len = options.files_len;
    debug!("files_len: {:?}", files_len);
    let mut files: Vec<WorkitemFile> = vec![];
    if files_len > 0 {
        debug!("get files of options");
        let _files = unsafe { &*options.files };
        debug!("slice files");
        let _files = unsafe { std::slice::from_raw_parts(_files, files_len.try_into().unwrap()) };
        debug!("loop files");
        files = _files.iter().map(|f| {
            debug!("process a file");
            let file = unsafe { &**f };
            debug!("create WorkitemFile instance 2");
            let filename = c_char_to_str(file.filename);
            debug!("filename: {:?}", filename);
            let id = c_char_to_str(file.id);
            debug!("id: {:?}", id);
            let compressed = file.compressed;
            debug!("compressed: {:?}", compressed); 
            WorkitemFile {
                filename: c_char_to_str(file.filename),
                id: c_char_to_str(file.id),
                compressed: file.compressed,
                ..Default::default()
                // file: file.file.clone(),
            }
        }).collect();
    }
    debug!("unwrap workitem");
    let workitem = unsafe { &*options
    .workitem };

    debug!("convert workitem wrapper to workitem");
    let workitem = workitem.as_workitem();
    let request = UpdateWorkitemRequest {
        workitem: Some(workitem),
        ignoremaxretries: options.ignoremaxretries,
        files,
    };

    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UpdateWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            workitem: std::ptr::null(),
            request_id: options.request_id
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client
            .update_workitem(request)
            .await;
        let response = match result {
            Ok(resp) => {
                let response = match resp.workitem {
                    Some(workitem) => {
                        let workitem = wrap_workitem(workitem);
                        UpdateWorkitemResponseWrapper {
                            success: true,
                            error: std::ptr::null(),
                            workitem: Box::into_raw(Box::new(workitem)),
                            request_id,
                        }
                    }
                    None => {
                        let error_msg = CString::new("Update workitem failed: workitem not found").unwrap().into_raw();
                        UpdateWorkitemResponseWrapper {
                            success: false,
                            error: error_msg,
                            workitem: std::ptr::null(),
                            request_id,
                        }
                    }
                };
                Box::into_raw(Box::new(response))
            }
            Err(e) => {
                let error_msg = CString::new(format!("Update workitem failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                Box::into_raw(Box::new(UpdateWorkitemResponseWrapper {
                    success: false,
                    error: error_msg,
                    workitem: std::ptr::null(),
                    request_id,
                }))
            }
        };
        callback(response);
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_update_workitem_response(response: *mut UpdateWorkitemResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).workitem.is_null() {
            free_workitem((*response).workitem as *mut WorkitemWrapper);
        }
        let _ = Box::from_raw(response);
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeleteWorkitemRequestWrapper {
    id: *const c_char,
    request_id: i32
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DeleteWorkitemResponseWrapper {
    success: bool,
    error: *const c_char,
    request_id: i32
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_workitem( 
    client: *mut ClientWrapper,
    options: *mut DeleteWorkitemRequestWrapper,
) -> *mut DeleteWorkitemResponseWrapper {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DeleteWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DeleteWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DeleteWorkitemRequest {
        id: c_char_to_str(options.id),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            request_id: options.request_id
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.unwrap();
    let result = tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(client
            .delete_workitem(request)
            )
    });

    let response = match result {
        Ok(_) => {
            let response = DeleteWorkitemResponseWrapper {
                success: true,
                error: std::ptr::null(),
                request_id: options.request_id
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Delete workitem failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = DeleteWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id
            };
            Box::into_raw(Box::new(response))
        }
    };
    debug!("return response {:?}", response);
    response
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn delete_workitem_async( 
    client: *mut ClientWrapper,
    options: *mut DeleteWorkitemRequestWrapper,
    callback: extern "C" fn(*mut DeleteWorkitemResponseWrapper),
) {
    let options = match safe_wrapper(options) {
        Some(options) => options,
        None => {
            let error_msg = CString::new("Invalid options").unwrap().into_raw();
            let response = DeleteWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                request_id: 0
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Client is not connected").unwrap().into_raw();
            let response = DeleteWorkitemResponseWrapper {
                success: false,
                error: error_msg,
                request_id: options.request_id
            };
            return callback(Box::into_raw(Box::new(response)));
        }
    };
    let client = client_wrapper.client.clone();
    let request = DeleteWorkitemRequest {
        id: c_char_to_str(options.id),
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = DeleteWorkitemResponseWrapper {
            success: false,
            error: error_msg,
            request_id: options.request_id
        };
        return callback(Box::into_raw(Box::new(response)));
    }
    let client = client.unwrap();
    let handle = client.get_runtime_handle();
    let request_id = options.request_id;
    handle.spawn(async move {
        let result = client
            .delete_workitem(request)
            .await;
        let response = match result {
            Ok(_) => {
                debug!("success");
                let response = DeleteWorkitemResponseWrapper {
                    success: true,
                    error: std::ptr::null(),
                    request_id,
                };
                Box::into_raw(Box::new(response))
            }
            Err(e) => {
                let error_msg = CString::new(format!("Delete workitem failed: {:?}", e))
                    .unwrap()
                    .into_raw();
                debug!("failed: {:?}", error_msg);
                let response = DeleteWorkitemResponseWrapper {
                    success: false,
                    error: error_msg,
                    request_id,
                };
                Box::into_raw(Box::new(response))
            }
        };
        debug!("callback {:?}", response);
        callback(response);
    });
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_delete_workitem_response(response: *mut DeleteWorkitemResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}





#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClientEventWrapper {
    event: *const c_char,
    reason: *const c_char,
}
impl Default for ClientEventWrapper {
    fn default() -> Self { 
        ClientEventWrapper {
            event: std::ptr::null(),
            reason: std::ptr::null(),
        }
     }
}
#[repr(C)]
pub struct ClientEventResponseWrapper {
    success: bool,
    eventid: *const c_char,
    error: *const c_char,
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn on_client_event(
    client: *mut ClientWrapper
) -> *mut ClientEventResponseWrapper {
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Invalid options, client is None").unwrap().into_raw();
            let response = ClientEventResponseWrapper {
                success: false,
                eventid: std::ptr::null(),
                error: error_msg,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let mut client = client_wrapper.client.clone();
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = ClientEventResponseWrapper {
            success: false,
            eventid: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.as_mut().unwrap();
    let eventid = Client::get_uniqueid();
    let _eventid = eventid.clone();
    tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
        handle.block_on(client.on_event(Box::new({
            move |event: ClientEvent| {
                let clientid = _eventid.clone();
                debug!("client event: {:?}", event);
    
                let mut e = CLIENT_EVENTS.lock().unwrap();
                let queue = e.get_mut(&clientid);
                match queue {
                    Some(q) => {
                        q.push_back(event);
                    }
                    None => {
                        let mut q = std::collections::VecDeque::new();
                        q.push_back(event);
                        e.insert(clientid, q);
                    }
                }
            }
        })));
    });
    
    let mut events = CLIENT_EVENTS.lock().unwrap();
    let _eventid = eventid.clone();
    let queue = events.get_mut(&_eventid);
    if queue.is_none() {
        let q = std::collections::VecDeque::new();
        let k = String::from(&eventid);
        events.insert(k, q);
    };
    let response = ClientEventResponseWrapper {
        success: true,
        eventid: CString::new(eventid).unwrap().into_raw(),
        error: std::ptr::null(),
    };
    return Box::into_raw(Box::new(response));
}
type ClientEventCallback = extern "C" fn(*mut ClientEventWrapper);
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn on_client_event_async(
    client: *mut ClientWrapper,
    event_callback: ClientEventCallback,
)  -> *mut ClientEventResponseWrapper {
    debug!("on_client_event_async::begin");
    let client_wrapper = match safe_wrapper(client) {
        Some(client) => client,
        None => {
            let error_msg = CString::new("Invalid options, client is None").unwrap().into_raw();
            let response = ClientEventResponseWrapper {
                success: false,
                eventid: std::ptr::null(),
                error: error_msg,
            };
            return Box::into_raw(Box::new(response));
        }
    };
    let mut client = client_wrapper.client.clone();
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = ClientEventResponseWrapper {
            success: false,
            eventid: std::ptr::null(),
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }
    let client = client.as_mut().unwrap();
    let eventid = Client::get_uniqueid();
    debug!("on_client_event_async::eventid: {:?}", eventid);
    let _eventid = eventid.clone();
    tokio::task::block_in_place(|| {
        let handle = client.get_runtime_handle();
            handle.block_on(client
            .on_event(
                Box::new(move |event: ClientEvent| {
                    let clientid = _eventid.clone();
                    debug!("client event: {:?}", event);
                    let mut e = CLIENT_EVENTS.lock().unwrap();
                    let queue = e.get_mut(&clientid);
                    match queue {
                        Some(_q) => {
                        }
                        None => {
                            // Stop the event listener if the queue is not found
                            return;
                        }
                    };
                    let event = match event {
                        ClientEvent::Connecting => ClientEventWrapper { event: CString::new("Connecting").unwrap().into_raw(),reason: std::ptr::null() },
                        ClientEvent::Connected => ClientEventWrapper { event: CString::new("Connected").unwrap().into_raw(),reason: std::ptr::null() },
                        ClientEvent::Disconnected(reason) => ClientEventWrapper { event: CString::new("Disconnected").unwrap().into_raw(),reason: CString::new(reason).unwrap().into_raw() },
                        ClientEvent::SignedIn => ClientEventWrapper { event: CString::new("SignedIn").unwrap().into_raw(),reason: std::ptr::null() },
                        // ClientEvent::SignedOut => ClientEventWrapper { event: CString::new("SignedOut").unwrap().into_raw(),reason: std::ptr::null() },
                    };
                    let event = Box::into_raw(Box::new(event));

                    event_callback(event);
                    debug!("client event: {:?}", event);
                }),
            )
        )
    });

    let mut events = CLIENT_EVENTS.lock().unwrap();
    let _eventid = eventid.clone();
    let queue = events.get_mut(&_eventid);
    if queue.is_none() {
        debug!("create event queue, for eventid: {:?}", eventid);
        let q = std::collections::VecDeque::new();
        let k = String::from(&eventid);
        events.insert(k, q);
    };
    debug!("on_client_event_async::end");
    let response = ClientEventResponseWrapper {
        success: true,
        eventid: CString::new(eventid).unwrap().into_raw(),
        error: std::ptr::null(),
    };
    return Box::into_raw(Box::new(response));

}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn next_client_event (
    clientid: *const c_char,    
) -> *mut ClientEventWrapper {
    debug!("unwrap clientid");
    let clientid = c_char_to_str(clientid);
    debug!("clientid {:}", clientid);
    let clientid = clientid.to_string();
    debug!("unwrap events");
    let mut e = CLIENT_EVENTS.lock().unwrap();
    debug!("get queue");
    let queue = e.get_mut(&clientid);
    match queue {
        Some(q) => {
            match q.pop_front() {
                Some(event) => {
                    debug!("got client event");
                    let event = match event {
                        ClientEvent::Connecting => ClientEventWrapper { event: CString::new("Connecting").unwrap().into_raw(),reason: std::ptr::null() },
                        ClientEvent::Connected => ClientEventWrapper { event: CString::new("Connected").unwrap().into_raw(),reason: std::ptr::null() },
                        ClientEvent::Disconnected(reason) => ClientEventWrapper { event: CString::new("Disconnected").unwrap().into_raw(),reason: CString::new(reason).unwrap().into_raw() },
                        ClientEvent::SignedIn => ClientEventWrapper { event: CString::new("SignedIn").unwrap().into_raw(),reason: std::ptr::null() },
                        // ClientEvent::SignedOut => ClientEventWrapper { event: CString::new("SignedOut").unwrap().into_raw(),reason: std::ptr::null() },
                    };
                    Box::into_raw(Box::new(event))
                }
                None => {
                    debug!("No event");
                    Box::into_raw(Box::new(ClientEventWrapper::default())) 
                },
            }
        },
        None => {
            debug!("Queue for {:} not found", clientid);
            Box::into_raw(Box::new(ClientEventWrapper::default())) 
        },
    }
}

#[repr(C)]
pub struct OffClientEventResponseWrapper {
    success: bool,
    error: *const c_char,
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn off_client_event(
    eventid: *const c_char,
) -> *mut OffClientEventResponseWrapper {
    let eventid = c_char_to_str(eventid);
    if eventid.is_empty() {
        let error_msg = CString::new("eventid is required").unwrap().into_raw();
        let response = OffClientEventResponseWrapper {
            success: false,
            error: error_msg,
        };
        return Box::into_raw(Box::new(response));
    }
    trace!("eventid: {:?}", eventid);
    
    let mut e = CLIENT_EVENTS.lock().unwrap();
    let queue = e.get_mut(&eventid);
    if let Some(q) = queue {
        q.clear();
        e.remove(&eventid);
    };
    Box::into_raw(Box::new(OffClientEventResponseWrapper {
        success: true,
        error: std::ptr::null(),
    }))    
}

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_off_event_response(response: *mut OffClientEventResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_event_response(response: *mut ClientEventResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).error.is_null() {
            let _ = CString::from_raw((*response).error as *mut c_char);
        }
        if !(*response).eventid.is_null() {
            let _ = CString::from_raw((*response).eventid as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }    
}
#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn free_client_event(response: *mut ClientEventWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        if !(*response).event.is_null() {
            let _ = CString::from_raw((*response).event as *mut c_char);
        }
        if !(*response).reason.is_null() {
            let _ = CString::from_raw((*response).reason as *mut c_char);
        }
        let _ = Box::from_raw(response);
    }        
}
