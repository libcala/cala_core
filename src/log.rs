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
    }
}
