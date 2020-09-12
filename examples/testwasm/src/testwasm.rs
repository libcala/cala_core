// TODO: yoyoy

/// Rust-Wrappers around Syscalls
pub mod sys {
    // TODO: yoyoy 2
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

#[no_mangle]
pub extern "C" fn wake(promise: u32, result: u32) {
    let future = start();
}
