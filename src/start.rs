#![allow(unsafe_code)]

use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, RawWaker, RawWakerVTable, Waker};

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
use wasm_bindgen::prelude::*;

type PinFut = Pin<Box<dyn Future<Output = ()>>>;

static mut FUTURE: MaybeUninit<PinFut> = MaybeUninit::uninit();
static mut WAKER: MaybeUninit<Waker> = MaybeUninit::uninit();
static mut CONTEXT: MaybeUninit<Context<'_>> = MaybeUninit::uninit();

/// Macro to define task executed at the start of the program.
#[macro_export]
macro_rules! start {
    () => {
        mod _cala_core {
            #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
            #[no_mangle]
            extern "C" fn main() {
                // FIXME: Block on
                todo!()
            }
            
            #[cfg(any(target_arch = "wasm32", target_os = "android"))]
            #[no_mangle]
            extern "C" fn start() {
                $crate::_macro::start(Box::pin(super::start()))
            }
        }
    }
}

#[cfg(target_os = "android")]
extern "C" fn android_main() {
    // FIXME: Block on
    todo!()
}

pub(crate) unsafe fn start(start: PinFut) {
    FUTURE = MaybeUninit::new(start);
    WAKER = MaybeUninit::new(waker());
    CONTEXT = MaybeUninit::new(Context::from_waker(&*WAKER.as_ptr()));
    wake();
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
#[allow(unreachable_pub)]
#[doc = ""]
#[wasm_bindgen]
pub fn wake() {
    let _ = unsafe {
        (*FUTURE.as_mut_ptr())
            .as_mut()
            .poll(&mut *CONTEXT.as_mut_ptr())
    };
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm-bindgen")))]
#[no_mangle]
unsafe extern "C" fn wake() {
    let _ = (*FUTURE.as_mut_ptr())
        .as_mut()
        .poll(&mut *CONTEXT.as_mut_ptr());
}

/// Create a waker for the executor - doesn't need any associated state.
#[inline(always)]
unsafe fn waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &RawWakerVTable::new(clone, wake_by, wake_by, drop))
    }

    unsafe fn wake_by(_data: *const ()) {
        wake();
    }

    unsafe fn drop(_data: *const ()) {}

    Waker::from_raw(RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(clone, wake_by, wake_by, drop),
    ))
}
