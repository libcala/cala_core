use std::{pin::Pin, task::{Poll, Context}, future::Future};
use cala_core::os::web::{JsFn, JsVar};

/// A JavaScript timer future that resolves to a predetermined value.
pub(crate) struct JsTimer {
    promise: JsVar,
}

impl JsTimer {
    pub fn new(res: i32, ms: i32) -> JsTimer {
        let promise = unsafe {
            // Create a function
            let timeout = 
                JsFn::new("return new Promise(function(res, rej) {\
                    setTimeout(function() { res(param_a); }, param_b);
                });");
            // Call the function
            timeout.call(Some(&JsVar::from_i32(res)), Some(&JsVar::from_i32(ms))).unwrap()
        };
        JsTimer {
            promise
        }
    }
}

impl Future for JsTimer {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<i32> {
        if let Some(result) = cala_core::os::web::resolve_promise(&self.promise) {
            Poll::Ready(unsafe { result.into_i32() })
        } else {
            unsafe { self.promise.set_waker() };
            Poll::Pending
        }
    }
}
