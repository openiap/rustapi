use client::openiap::{Envelope, QueryRequest, SigninRequest, DownloadRequest, UploadRequest};
use std::ffi::CStr;
use std::os::raw::c_char;
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
    runtime: std::sync::Arc<Runtime>,
}
#[no_mangle]
pub extern "C" fn client_connect(server_address: *const c_char) -> *mut ClientWrapper {
    let server_address = unsafe { CStr::from_ptr(server_address).to_str().unwrap() };
    let runtime = std::sync::Arc::new(Runtime::new().unwrap());
    let client = runtime.block_on(Client::connect(server_address));
    if client.is_err() == true {
        let e = client.err().unwrap();
        let error_msg = CString::new(format!("Connaction failed: {:?}", e))
            .unwrap()
            .into_raw();
        return Box::into_raw(Box::new(ClientWrapper {
            client: None,
            runtime,
            success: false,
            error: error_msg,
        }));
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
    if response.is_null() {
        println!("free_client: response is null");
        return;
    }
    unsafe {
        let response_ref: &ClientWrapper = &*response;
        if !response_ref.error.is_null() {
            let error_cstr = CStr::from_ptr(response_ref.error);
            if let Ok(error_str) = error_cstr.to_str() {
                println!("free_client: error = {}", error_str);
            } else {
                println!("free_client: error = <invalid UTF-8>");
            }
        }

        if let Some(client) = &response_ref.client {
            let client_clone = client.clone();
            let runtime = &response_ref.runtime;

            // Ensure that the runtime properly shuts down after the block_on call
            runtime.block_on(async move {
                {
                    let inner = client_clone.inner.lock().await;
                    let mut queries = inner.queries.lock().await;

                    // Cancel pending requests
                    for (id, response_tx) in queries.drain() {
                        println!("free_client: canceling request with id: {:?}", id);
                        let _ = response_tx.send(Envelope {
                            command: "cancelled".to_string(),
                            ..Default::default()
                        });
                    }

                    // println!("free_client: released queries lock");
                } // Ensure locks are dropped before proceeding

                {
                    let inner = client_clone.inner.lock().await;
                    let mut streams = inner.streams.lock().await;
                    let stream_keys = streams.keys().cloned().collect::<Vec<String>>();
                    stream_keys.iter().for_each(|k| {
                        println!("free_client: client inner state: stream: {:?}", k);
                        streams.remove(k.clone().as_str());
                    });
                    // println!("free_client: released streams lock");
                } // Ensure locks are dropped before proceeding
            });
        }
        // Free the client
        // let _client_wrapper: Box<ClientWrapper> = Box::from_raw(response);
        // println!("free_client 5");
    }
    println!("free_client 6");
}

// #[no_mangle]
// pub extern "C" fn free_client(response: *mut ClientWrapper) {
//     println!n!("free_client 1");
//     if response.is_null() { return; }
//     println!n!("free_client 2");
//     unsafe {
//         println!n!("free_client 3");
//         let _ = Box::from_raw(response);
//         println!n!("free_client 4");
//     }
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
pub extern "C" fn client_signin(
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
    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.signin(request).await
    });
    match result {
        Ok(data) => {
            let jwt = CString::new(data.jwt).unwrap().into_raw();
            let response = SigninResponseWrapper {
                success: true,
                jwt,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Signin failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = SigninResponseWrapper {
                success: false,
                jwt: std::ptr::null(),
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    }
}

#[no_mangle]
pub extern "C" fn free_signin_response(response: *mut SigninResponseWrapper) {
    if response.is_null() {
        return;
    }
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
pub extern "C" fn client_query(
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
    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.query(request).await
    });
    match result {
        Ok(data) => {
            let results = CString::new(data.results).unwrap().into_raw();
            let response = QueryResponseWrapper {
                success: true,
                results,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Query failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = QueryResponseWrapper {
                success: false,
                results: std::ptr::null(),
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    }
}

#[no_mangle]
pub extern "C" fn free_query_response(response: *mut QueryResponseWrapper) {
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
pub extern "C" fn client_download(
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
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }.to_string(),
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
    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.download(request, Some(folder), Some(filename)).await
    });
    match result {
        Ok(data) => {
            let filename = CString::new(data.filename).unwrap().into_raw();
            let response = DownloadResponseWrapper {
                success: true,
                filename,
                error: std::ptr::null(),
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Download failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = DownloadResponseWrapper {
                success: false,
                filename: std::ptr::null(),
                error: error_msg,
            };
            Box::into_raw(Box::new(response))
        }
    }
}
#[no_mangle]
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
    error: *const c_char
}
#[no_mangle]
pub extern "C" fn client_upload(
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
            error: error_msg
        };
        return Box::into_raw(Box::new(response));
    }
    let filename = unsafe { CStr::from_ptr(options.filename).to_str().unwrap() };
    if filename.is_empty() {
        let error_msg = CString::new("Filename is required").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg
        };
        return Box::into_raw(Box::new(response));
    }

    let request = UploadRequest {
        filename: filename.to_string(),
        mimetype: unsafe { CStr::from_ptr(options.mimetype).to_str().unwrap() }.to_string(),
        metadata: unsafe { CStr::from_ptr(options.metadata).to_str().unwrap() }.to_string(),
        collectionname: unsafe { CStr::from_ptr(options.collectionname).to_str().unwrap() }.to_string(),
        ..Default::default()
    };
    if client.is_none() {
        let error_msg = CString::new("Client is not connected").unwrap().into_raw();
        let response = UploadResponseWrapper {
            success: false,
            id: std::ptr::null(),
            error: error_msg
        };
        return Box::into_raw(Box::new(response));
    }
    let result = runtime.block_on(async {
        let c = client.as_ref().unwrap();
        c.upload(request, filepath).await
    });
    match result {
        Ok(data) => {
            let id = CString::new(data.id).unwrap().into_raw();
            let response = UploadResponseWrapper {
                success: true,
                id,
                error: std::ptr::null()
            };
            Box::into_raw(Box::new(response))
        }
        Err(e) => {
            let error_msg = CString::new(format!("Upload failed: {:?}", e))
                .unwrap()
                .into_raw();
            let response = UploadResponseWrapper {
                success: false,
                id: std::ptr::null(),
                error: error_msg
            };
            Box::into_raw(Box::new(response))
        }
    }
}
#[no_mangle]
pub extern "C" fn free_upload_response(response: *mut UploadResponseWrapper) {
    if response.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(response);
    }
}