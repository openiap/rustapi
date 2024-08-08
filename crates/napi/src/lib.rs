use std::thread;

use napi_derive::napi;

 
#[allow(dead_code)]
#[napi]
fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}


use napi::*;
use napi_derive::js_function;

#[js_function(1)]
pub fn test_deferred(ctx: CallContext) -> Result<JsObject> {
  let reject: bool = ctx.get(0)?;
  let (deferred, promise) = ctx.env.create_deferred()?;

  thread::spawn(move || {
    thread::sleep(std::time::Duration::from_millis(10));
    if reject {
      deferred.reject(Error::from_reason("Fail"));
    } else {
      deferred.resolve(|_| Ok(15));
    }
  });

  Ok(promise)
}


