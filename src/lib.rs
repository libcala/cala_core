// cala_core
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Getting Started
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies.cala_core]
//! version = "0.1.0"
//! ```
//!
//! ```rust
//! // TODO
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/libcala/cala_core/master/res/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/libcala/cala_core/master/res/logo.svg",
    html_root_url = "https://docs.rs/cala_core"
)]
#![deny(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

#[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
#[macro_use]
extern crate stdweb;

#[cfg(all(feature = "cala", not(target_arch = "wasm32")))]
#[doc(hidden)]
pub extern crate pasts;

pub mod os;

/// **cala**: Set an asynchronous function as the entry point for the program.
#[cfg(all(feature = "cala", not(target_arch = "wasm32")))]
#[macro_export]
macro_rules! exec {
    ($main:ident) => {
        fn main() {
            use $crate::pasts::Executor;
            static EXECUTOR: $crate::pasts::CvarExec
                = $crate::pasts::CvarExec::new();
            EXECUTOR.block_on($main());
        }
    };
}

/// **cala**: Set an asynchronous function as the entry point for the program.
#[cfg(all(feature = "cala", target_arch = "wasm32"))]
#[macro_export]
macro_rules! exec {
    ($main:ident) => {
        mod main {
            // Web Assembly "main" function
            #[no_mangle]
            extern "C" fn wasm_main() {
                $crate::os::web::panic_hook();
                $crate::os::web::block_on(super::$main());
            }
        }
    };
}

/*/// Set a function as the entry point for the program.
#[cfg(all(target_os = "android", not(target_arch = "wasm32")))]
#[macro_export]
macro_rules! main {
    ($main:expr) => {
        mod __cala_core_macro_generated {
            use super::*;

            /// Called from NativeActivity JNI
            #[no_mangle]
            pub extern "C" fn android_main(
                state: *mut c_void, /*AndroidApp*/
            ) -> () {
                $main($crate::System);
            }
        }
    };
}
/// Set a function as the entry point for the program.
#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
#[macro_export]
macro_rules! main {
    ($main:expr) => {
        mod __cala_core_macro_generated {
            use super::*;

            /// Called from Windows Runtime
            #[no_mangle]
            pub extern "C" fn wWinMain(
                _h_instance: *mut c_void,
                _h_prev_instance: *mut c_void,
                _p_cmd_line: *mut u16,
                _n_cmd_show: c_int,
            ) -> c_int {
                $main($crate::System);
                0
            }
        }
    };
}
/// Set a function as the entry point for the program.
#[cfg(not(any(
    target_arch = "wasm32",
    target_os = "android",
    target_os = "windows",
)))]
#[macro_export]
macro_rules! main {
    ($main:expr) => {
        fn main() {
            $main($crate::System);
        }
    };
}*/
