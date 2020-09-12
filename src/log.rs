//! Terminal logs.

////////////////////
//// Wasm32 Web ////
////////////////////

#[cfg(all(target_arch = "wasm32", not(feature = "wasm-bindgen")))]
mod ffi {
    extern "C" {
        pub(super) fn say(size: u32, array: u32);
    }
}

#[cfg(all(target_arch = "wasm32", not(feature = "wasm-bindgen")))]
#[allow(unsafe_code)]
pub(crate) fn say(text: &str) {
    unsafe {
        ffi::say(text.len() as _, text.as_ptr() as _);
    }
}

/////////////////////////
//// Wasm32 Wasmpack ////
/////////////////////////

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
mod ffi {
    #[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = info)]
        pub fn say(data: &str);
    }
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
pub(crate) fn say(text: &str) {
    ffi::say(text);
}

////////////
//// PC ////
////////////

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
mod ffi {
    use crate::os::*;

    extern "C" {
        pub(super) fn write(
            fd: c_sint,
            buf: *const c_void,
            count: c_usize,
        ) -> c_ssize;
    }
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
#[allow(unsafe_code)]
pub(crate) fn say(text: &str) {
    unsafe {
        ffi::write(0i16.into(), text.as_ptr().cast(), text.len().into());
        ffi::write(0i16.into(), b"\n".as_ptr().cast(), 1.into());
    }
}

/////////////
//// End ////
/////////////

/// Tag for log entries.  First element in tuple is the display name, second is
/// whether or not to print it.
pub type Tag = (&'static str, bool);

/// Write out debug information to the log.
#[macro_export]
macro_rules! log {
    ($text:literal) => {
        $crate::_macro::say($text)
    };

    ($text:literal, $($d:expr),+, (,)?) => {
        let mut string = String::new($text);
        $(
            string.write_fmt(format_args!("{}", self))
                .expect("a Display impl returned an error unexpectedly");
        )+
        $crate::_macro::say($text)
    };

    ($tag:ident, $($d:expr),+, (,)?) => {
        if tag.1 {
            let mut string = String::new($tag.0);
            $(
                string.write_fmt(format_args!("{}", self))
                    .expect("a Display impl returned an error unexpectedly");
            )+
            $crate::_macro::say($text)
        }
    };
}
