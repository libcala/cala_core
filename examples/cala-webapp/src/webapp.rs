use std::{pin::Pin, future::Future, task::{Poll, Context}};
use cala_core::{
    main,
    os::web::{JsFn, JsVar, JsString},
};

/// A JavaScript timer with a resolution
struct JsTimer {
    promise: JsVar,
}

impl JsTimer {
    pub fn new(res: u32, ms: u32) -> JsTimer {
        let promise = unsafe {
            // Create a function
            let timeout = 
                JsFn::new("return new Promise(function(res, rej) {\
                    setTimeout(function() { res(param_a); }, param_b);
                });");
            // Call the function
            timeout.call(Some(&JsVar::from_u32(res)), Some(&JsVar::from_u32(ms))).unwrap()
        };
        JsTimer {
            promise
        }
    }
}

impl Future for JsTimer {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<u32> {
        if let Some(result) = cala_core::os::web::resolve_promise(&self.promise) {
            Poll::Ready(unsafe { result.into_u32() })
        } else {
            unsafe { self.promise.set_waker(context.waker().clone()) };
            Poll::Pending
        }
    }
}

fn panic_hook(panic_info: &std::panic::PanicInfo) {
    let msg = panic_info.to_string();

    let message = JsString::new(&format!("Cala App panicked!: {:?}", msg));
    let eprint = unsafe { JsFn::new("throw new Error(param_a);") };
    unsafe {
        assert!(eprint.call(Some(message.as_var()), None).is_none());
    }
}

async fn async_main() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |p| {
        hook(p);
        panic_hook(p);
    }));
    
    let message = JsString::new("Hello, world!");
    let alert = unsafe { JsFn::new("alert(param_a);") };

    unsafe {
        assert!(alert.call(Some(message.as_var()), None).is_none());
    }
    
    let result = JsTimer::new(42, 2000).await;
    let message = JsString::new(&format!("Waited 2 seconds to get {}", result));

    unsafe {
        assert!(alert.call(Some(message.as_var()), None).is_none());
    }
}

main!(async_main());
