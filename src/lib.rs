use std::os::raw::{c_char};
use std::ffi::CStr;
use client::openiap::SigninRequest;
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
    if response.is_null() { return; }
    unsafe {
        let _ = Box::from_raw(response);
    }
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
pub extern "C" fn client_signin(client: *mut ClientWrapper, options: *mut SigninRequestWrapper) -> *mut SigninResponseWrapper {
    let options = unsafe { &*options };
    let client_wrapper = unsafe { &mut *client };
    // let username_cstr = unsafe { CStr::from_ptr(options.username) };
    // match username_cstr.to_str() {
    //     Ok(username_str) => {
    //     }
    //     Err(e) => {
    //         let error_msg = CString::new(format!("Invalid username: {:?}", e)).unwrap().into_raw();
    //         let response = SigninResponseWrapper { success: false, jwt: std::ptr::null(), error: error_msg };
    //         return Box::into_raw(Box::new(response));
    //     }
    // }
    // let username_str = username_cstr.to_str().unwrap();

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

