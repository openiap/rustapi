use std::os::raw::{c_char};
use std::ffi::CStr;
use client::openiap::{ SigninRequest , QueryRequest};
use tokio::runtime::Runtime;
pub mod client;
use client::Client;
use std::ffi::CString;

#[allow(dead_code)]
#[repr(C)]
pub struct ClientWrapper {
    success: bool,
    error: *const c_char,
    client: Option<Client>,
    runtime: Runtime,
}
#[no_mangle]
pub extern "C" fn client_connect(server_address: *const c_char) -> *mut ClientWrapper {
    let server_address = unsafe { CStr::from_ptr(server_address).to_str().unwrap() };
    let runtime = Runtime::new().unwrap();
    let client = runtime.block_on(Client::connect(server_address));
    if client.is_err() == true {
        let e = client.err().unwrap();
        let error_msg = CString::new(format!("Connaction failed: {:?}", e)).unwrap().into_raw();
        return Box::into_raw(Box::new(ClientWrapper {
            client: None,
            runtime,
            success: false,
            error: error_msg,
        }))
    }
    Box::into_raw(Box::new(ClientWrapper {
        client: Some(client.unwrap()),
        runtime,
        success: true,
        error: std::ptr::null(),
    }))
}
#[no_mangle]
pub extern "C" fn free_client(response: *mut ClientWrapper) {
    println!("free_client 1");
    if response.is_null() { return; }
    println!("free_client 2");
    unsafe {
        println!("free_client 3");
        let _ = Box::from_raw(response);
        println!("free_client 4");
    }
}
// #[no_mangle]
// pub extern "C" fn free_client(response: *mut ClientWrapper) {
//     println!("free_client 1");
//     if response.is_null() {
//         println!("free_client: response is null");
//         return;
//     }
//     println!("free_client 2");
//     unsafe {
//         println!("free_client 3");
//         let response_ref: &ClientWrapper = &*response;
//         if !response_ref.error.is_null() {
//             let error_cstr = CStr::from_ptr(response_ref.error);
//             if let Ok(error_str) = error_cstr.to_str() {
//                 println!("free_client: error = {}", error_str);
//             } else {
//                 println!("free_client: error = <invalid UTF-8>");
//             }
//         } else {
//             println!("free_client: no error message");
//         }
        
//         // Additional debug for client
//         if let Some(client) = &response_ref.client {
//             println!("free_client: client exists, checking inner state");
//             let inner = client.inner.lock().unwrap();
//             println!("free_client: client inner state: {:?}", inner);
//         } else {
//             println!("free_client: no client to free");
//         }
        
//         // Free the client
//         let _client_wrapper: Box<ClientWrapper> = Box::from_raw(response);
//         println!("free_client 4");
//     }
//     println!("free_client 5");
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
#[no_mangle]
pub extern "C" fn client_signin(client: *mut ClientWrapper, options: *mut SigninRequestWrapper) -> *mut SigninResponseWrapper {
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
        let response = SigninResponseWrapper { success: false, jwt: std::ptr::null(), error: error_msg };
        return Box::into_raw(Box::new(response));
    }
    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.signin(request ).await
    });
    match result {
        Ok(data) => {
            let jwt = CString::new(data.jwt).unwrap().into_raw();
            let response = SigninResponseWrapper { success: true, jwt, error: std::ptr::null() };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Signin failed: {:?}", e)).unwrap().into_raw();
            let response = SigninResponseWrapper { success: false, jwt: std::ptr::null(), error: error_msg };
            Box::into_raw(Box::new(response))
        }
    }
}

#[no_mangle]
pub extern "C" fn free_signin_response(response: *mut SigninResponseWrapper) {
    if response.is_null() { return; }
    unsafe {
        let _ = Box::from_raw(response);
    }
}
#[no_mangle]
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
#[no_mangle]
pub extern "C" fn client_query(client: *mut ClientWrapper, options: *mut QueryRequestWrapper) -> *mut QueryResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    let client = &client_wrapper.client;
    let runtime = &client_wrapper.runtime;
    let request = QueryRequest {
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }.to_string(),
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
        let response = QueryResponseWrapper { success: false, results: std::ptr::null(), error: error_msg };
        return Box::into_raw(Box::new(response));
    }
    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.query(request ).await
    });
    match result {
        Ok(data) => {
            let results = CString::new(data.results).unwrap().into_raw();
            let response = QueryResponseWrapper { success: true, results, error: std::ptr::null() };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Query failed: {:?}", e)).unwrap().into_raw();
            let response = QueryResponseWrapper { success: false, results: std::ptr::null(), error: error_msg };
            Box::into_raw(Box::new(response))
        }
    }
}

#[no_mangle]
pub extern "C" fn free_query_response(response: *mut QueryResponseWrapper) {
    if response.is_null() { return; }
    unsafe {
        let _ = Box::from_raw(response);
    }
}