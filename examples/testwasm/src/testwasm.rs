use std::pin::Pin;
use std::future::Future;
use std::task::{RawWaker, RawWakerVTable, Waker, Context};
use std::mem::MaybeUninit;

/// Rust-Wrappers around Syscalls
pub mod sys {
    /// Rynvei Syscalls
    mod ffi {
        extern "C" {
            pub fn say(size: u32, array: u32);
        }
    }

    pub fn say(text: &str) {
        unsafe {
            ffi::say(text.len() as _, text.as_ptr() as _);
        }
    }
}

/// Function called at the start of the progam.
async fn start() {
    sys::say("Hello, world! ☺");
}

type PinFut = Pin<Box<dyn Future<Output = ()>>>;

static mut FUTURE: MaybeUninit<PinFut> = MaybeUninit::uninit();
static mut WAKER: MaybeUninit<Waker> = MaybeUninit::uninit();
static mut CONTEXT: MaybeUninit<Context> = MaybeUninit::uninit();

#[no_mangle]
pub unsafe extern "C" fn init() {
    FUTURE = MaybeUninit::new(Box::pin(start()));
    WAKER = MaybeUninit::new(waker());
    CONTEXT = MaybeUninit::new(Context::from_waker(&*WAKER.as_ptr()));
    wake(0, 0);
}

#[no_mangle]
pub unsafe extern "C" fn wake(promise: u32, result: u32) {
    let _ = (*FUTURE.as_mut_ptr()).as_mut().poll(&mut *CONTEXT.as_mut_ptr());
}

/// Create a waker for the executor - doesn't need any associated state.
#[inline(always)]
unsafe fn waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &RawWakerVTable::new(clone, wake_by, wake_by, drop))
    }

    unsafe fn wake_by(_data: *const ()) {
        wake(0, 0);
    }

    unsafe fn drop(_data: *const ()) {}

    Waker::from_raw(RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(clone, wake_by, wake_by, drop),
    ))
}
