use std::{pin::Pin, task::{Poll, Context}, future::Future};
use cala_core::os::web::{JsFn, JsVar};

/// A JavaScript timer future that resolves to a predetermined value.
pub(crate) struct JsTimer {
    // Reset the Promise so that it doesn't resolve twice.
    reset_promise: JsFn,
    // Current promise (needs to be reconstructed for every Poll::Ready)
    promise: JsVar,
}

impl JsTimer {
    pub fn new(res: i32, ms: i32) -> JsTimer {
        let reset_promise = unsafe {
            JsFn::new("\
                var resolve;\
                setInterval(function() { resolve(param_a); }, param_b);\
                return function(a, b) {\
                    return new Promise(function(res, rej) { resolve = res; });\
                };\
            ").call(Some(&JsVar::from_i32(res)), Some(&JsVar::from_i32(ms))).unwrap().into_jsfn()
        };
        let promise = unsafe {
            reset_promise.call(None, None).unwrap()
        };
        JsTimer {
            promise, reset_promise
        }
    }
}

impl Future for JsTimer {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<i32> {
        let mut this = self.as_mut();
        if let Poll::Ready(result) = this.promise.poll() {
            this.promise = unsafe { this.reset_promise.call(None, None).unwrap() };
            Poll::Ready(unsafe { result.into_i32() })
        } else {
            unsafe { self.promise.set_waker() };
            Poll::Pending
        }
    }
}
