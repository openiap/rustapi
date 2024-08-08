/*
needs either libuv-sys2 or libuv-sys in Cargo.toml
But fails to compile on anything but linix, so useless :-|
# uv-sys = "0.1.1"
# libuv-sys2 = "1.48.0"
# libuv-sys = "0.1.0"
# libuv = "1.0"

# libc = "0.2.155"

 */



/*
// uv-sys = "0.1.1"
use tracing::debug;
use uv_sys::sys::*;
use std::ops::Add;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn run_async_in_node(callback: extern "C" fn()) {
    debug!("Running the async operation in Rust");
    let (tx, rx) = mpsc::channel::<extern "C" fn()>();

    debug!("Spawning a new thread to simulate async operation");
    thread::spawn(move || {
        let mut i = 0;
        // Simulate some asynchronous operation
        loop {
            debug!("Simulating async operation {}", i);
            thread::sleep(Duration::from_secs(1));
            i = i.add(1);
            if i >= 5 {
                break;
            }
        }
        
        debug!("Sending the callback to the main thread");
        tx.send(callback).unwrap();
        
        unsafe {
            debug!("Triggering the callback in the main thread");
            let default_loop = uv_default_loop();
            debug!("Initializing the async handle");
            let mut handle: uv_async_t = std::mem::zeroed();
            debug!("Setting the data to the receiver");
            uv_async_init(default_loop, &mut handle, Some(trigger_uv_callback));
            debug!("Setting the receiver to the handle");
            handle.data = Box::into_raw(Box::new(rx)) as *mut _;
            debug!("Sending the async handle to the main thread");
            uv_async_send(&mut handle);
            uv_ref(&mut handle as *mut _ as *mut uv_handle_t);
            debug!("Running the UV loop explicitly");
            uv_run(default_loop, uv_run_mode::UV_RUN_NOWAIT);
        }
    });
}

#[tracing::instrument(skip_all)]
extern "C" fn trigger_uv_callback(handle: *mut uv_async_t) {
    unsafe {
        debug!("Triggering the callback from Rust");
        let rx: Box<Receiver<extern "C" fn()>> = Box::from_raw((*handle).data as *mut _);
        if let Ok(callback) = rx.try_recv() {
            debug!("Received the callback from the channel");
            callback(); // Call the original Node.js callback

            debug!("Cleaning up the handle");
            // Clean up the handle to avoid memory leaks
            uv_close(handle as *mut _ , Some(close_handle_cb));
        } else {
            // **Important**: If there's nothing to receive, or in case of an error,
            // You need to put the receiver back, else it will be dropped and cause a memory leak
            debug!("Putting the receiver back");
            (*handle).data = Box::into_raw(rx) as *mut _;
        }
        debug!("Exiting trigger_uv_callback");
    }
}
#[tracing::instrument(skip_all)]
extern "C" fn close_handle_cb(handle: *mut uv_handle_t) {
    unsafe {
        uv_unref(handle);
    }
}
*/


/*
use tracing::debug;

use uv_sys::sys::*;
use std::ops::Add;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

#[no_mangle]
#[tracing::instrument(skip_all)]
pub extern "C" fn run_async_in_node(callback: extern "C" fn()) {
    debug!("Running the async operation in Rust");
    let (tx, rx) = mpsc::channel::<extern "C" fn()>();

    debug!("Spawning a new thread to simulate async operation");
    thread::spawn(move || {
        let mut i = 0;
        // Simulate some asynchronous operation
        loop {
            debug!("Simulating async operation {}", i);
            thread::sleep(Duration::from_secs(1));
            i = i.add(1);
            if i >= 5 {
                break;
            }
        }
        
        debug!("Sending the callback to the main thread");
        tx.send(callback).unwrap();
        
        unsafe {
            debug!("Triggering the callback in the main thread");
            let default_loop = uv_default_loop();
            debug!("Initializing the async handle");
            let mut handle: uv_async_t = std::mem::zeroed();
            debug!("Setting the data to the receiver");
            uv_async_init(default_loop, &mut handle, Some(trigger_uv_callback));
            debug!("Setting the receiver to the handle");
            handle.data = Box::into_raw(Box::new(rx)) as *mut _;
            debug!("Sending the async handle to the main thread");
            uv_async_send(&mut handle);
            uv_ref(&mut handle as *mut _ as *mut uv_handle_t);
            debug!("Running the UV loop explicitly");
            uv_run(default_loop, uv_run_mode::UV_RUN_NOWAIT);
        }
    });
}

#[tracing::instrument(skip_all)]
extern "C" fn trigger_uv_callback(handle: *mut uv_async_t) {
    unsafe {
        debug!("Triggering the callback from Rust");
        let rx: Box<Receiver<extern "C" fn()>> = Box::from_raw((*handle).data as *mut _);
        if let Ok(callback) = rx.try_recv() {
            debug!("Received the callback from the channel");
            callback(); // Call the original Node.js callback

            debug!("Cleaning up the handle");
            // Clean up the handle to avoid memory leaks
            uv_close(handle as *mut _ , Some(close_handle_cb));
        } else {
            // **Important**: If there's nothing to receive, or in case of an error,
            // You need to put the receiver back, else it will be dropped and cause a memory leak
            debug!("Putting the receiver back");
            (*handle).data = Box::into_raw(rx) as *mut _;
        }
        debug!("Exiting trigger_uv_callback");
    }
}
#[tracing::instrument(skip_all)]
extern "C" fn close_handle_cb(handle: *mut uv_handle_t) {
    unsafe {
        uv_unref(handle);
    }
}

*/

// use napi::sys::{napi_make_callback, napi_create_function, napi_get_undefined, napi_get_global, napi_get_null, napi_get_boolean, napi_get_number, napi_get};



// use napi_derive::napi;
// use napi::
 
// #[napi]
// fn fibonacci(n: u32) -> u32 {
//   match n {
//     1 | 2 => 1,
//     _ => fibonacci(n - 1) + fibonacci(n - 2),
//   }
// }


// use napi::*;
// use napi_derive::js_function;

// #[js_function(1)]
// pub fn test_deferred(ctx: CallContext) -> Result<JsObject> {
//   let reject: bool = ctx.get(0)?;
//   let (deferred, promise) = ctx.env.create_deferred()?;

//   thread::spawn(move || {
//     thread::sleep(std::time::Duration::from_millis(10));
//     if reject {
//       deferred.reject(Error::from_reason("Fail"));
//     } else {
//       deferred.resolve(|_| Ok(15));
//     }
//   });

//   Ok(promise)
// }


