// cala_core
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Web-Specific APIs

#[cfg(feature = "stdweb")]
use stdweb::unstable::TryInto;

/// A JavaScript variable.
#[derive(Debug)]
pub struct JsVar(u32);

impl Drop for JsVar {
    #[cfg(feature = "stdweb")]
    fn drop(&mut self) {
        js! {
            _cala_js_free(@{self.0});
        }
    }

    #[allow(unsafe_code)]
    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    fn drop(&mut self) {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_free(idx: u32) -> ();
        }
        unsafe {
            _cala_js_free(self.0);
        }
    }
}

/// A JavaScript String
#[derive(Debug)]
pub struct JsString(JsVar);

impl JsString {
    /// Allocate a new javascript string from a Rust string slice.
    #[cfg(feature = "stdweb")]
    pub fn new(string: &str) -> JsString {
        // around the right amount of memory
        let mut utf16 = Vec::with_capacity(string.len());
        for c in string.encode_utf16() {
            utf16.push(c);
        }
        //
        let string = js! {
            return _cala_js_string(@{utf16.as_ptr() as u32}, @{utf16.len() as u32});
        };
        JsString(JsVar(string.try_into().unwrap()))
    }

    /// Allocate a new javascript string from a Rust string slice.
    #[allow(unsafe_code)]
    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    pub fn new(string: &str) -> JsString {
        // around the right amount of memory
        let mut utf16 = Vec::with_capacity(string.len());
        for c in string.encode_utf16() {
            utf16.push(c);
        }
        //
        extern "C" {
            // Turn Rust UTF-16 String Into JavaScript String.
            fn _cala_js_string(p: u32, l: u32) -> u32;
        }
        let string = unsafe {
            _cala_js_string(utf16.as_ptr() as u32, utf16.len() as u32)
        };
        JsString(JsVar(string))
    }

    /// Get a reference to this JsString as a JsVar
    pub fn as_var(&self) -> &JsVar {
        &self.0
    }

    /// Turn a `JsVar` into a `JsString`.  This does no type-checking, therefore
    /// is unsafe.
    #[allow(unsafe_code)]
    pub unsafe fn from_var(var: JsVar) -> JsString {
        JsString(var)
    }
}

/// A JavaScript Function.
#[derive(Debug)]
pub struct JsFn(u32);

impl JsFn {
    /// Define a function (two parameters param_a: u32, and param_b: u32,
    /// returns a u32)
    #[allow(unsafe_code)]
    #[cfg(feature = "stdweb")]
    pub unsafe fn new(string: &str) -> JsFn {
        let javascript = format!("\
            \"use strict\";\
            let offset = _cala_stack.length;
            _cala_stack.push(function(param_a, param_b) {{ {} return 4294967295; }});\
            return offset;\
        ", string);

        let string = JsString::new(&javascript);
        let func = js! {
            return _cala_js_function(@{string.as_var().0});
        };

        JsFn(func.try_into().unwrap())
    }

    /// Define a function (two parameters param_a: u32, and param_b: u32,
    /// returns a u32)
    #[allow(unsafe_code)]
    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    pub unsafe fn new(string: &str) -> JsFn {
        extern "C" {
            // Execute some JavaScript string.
            fn _cala_js_function(idx: u32) -> u32;
        }

        let javascript = format!("\
            \"use strict\";\
            let offset = _cala_stack.length;
            _cala_stack.push(function(param_a, param_b) {{ {} return 4294967295; }});\
            return offset;\
        ", string);

        let string = JsString::new(&javascript);
        let func = _cala_js_function(string.as_var().0);

        JsFn(func)
    }

    /// Call a JavaScript function.
    #[allow(unsafe_code)]
    #[cfg(feature = "stdweb")]
    pub unsafe fn call(
        &self,
        a: Option<&JsVar>,
        b: Option<&JsVar>,
    ) -> Option<JsVar> {
        let ret = js! {
            return _cala_js_call(@{self.0}, @{a.map(|x| x.0).unwrap_or(u32::MAX)}, @{b.map(|x| x.0).unwrap_or(u32::MAX)});
        };
        if ret == u32::MAX {
            None
        } else {
            Some(JsVar(ret.try_into().unwrap()))
        }
    }

    /// Call a JavaScript function.
    #[allow(unsafe_code)]
    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    pub unsafe fn call(
        &self,
        a: Option<&JsVar>,
        b: Option<&JsVar>,
    ) -> Option<JsVar> {
        extern "C" {
            // A generic javascript shim
            fn _cala_js_call(function: u32, idx: u32) -> u32;
        }
        let ret = _cala_js_call(
            a.map(|x| x.0).unwrap_or(u32::MAX),
            b.map(|x| x.0).unwrap_or(u32::MAX),
        );
        if ret == u32::MAX {
            None
        } else {
            Some(JsVar(ret))
        }
    }
}

// So that `JsFn` doesn't accidentally implement `Copy`
impl Drop for JsFn {
    fn drop(&mut self) {}
}

/*/// Inject JavaScript into the DOM.
pub fn inject_js(code: &str) {
    #[cfg(feature = "stdweb")] {
        use stdweb::web;

        stdweb::initialize();

        js! {
            let offset = _cala_stack.length;
            _cala_stack.push(function(p) { alert("Hello, World!"); return -1; });
            return offset;
        }

        web::alert(&format!("Cargo Web: {}", code));
    }

    #[cfg(feature = "wasm-bindgen")] {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern "C" {
            fn alert(s: &str);
        }

        alert(&format!("Wasm Bindgen: {}", code));
    }

    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    #[allow(unsafe_code)]
    {
        extern "C" {
            // Turn Rust UTF-16 String Into JavaScript String.
            fn _cala_js_string(p: u32, l: u32) -> u32;
            // Execute some JavaScript string.
            fn _cala_function(idx: u32) -> u32;
            // Free a JavaScript object
            fn _cala_js_free(idx: u32) -> ();
            // A generic javascript shim
            fn _cala_call(function: u32, idx: u32) -> u32;
        }

        unsafe {
            // Load library into memory
            let javascript = "\
                \"use strict\";\
                let offset = _cala_stack.length;
                _cala_stack.push(function(p) { alert('Hello, World!'); return -1; });\
                return offset;\
            ";
            let mut utf16 = Vec::with_capacity(javascript.len()); // around the right amount of memory
            for c in javascript.encode_utf16() {
                utf16.push(c);
            }
            let string = _cala_js_string(utf16.as_ptr() as u32, utf16.len() as u32);
            let offset = _cala_function(string);
            _cala_js_free(string);

            // Call the library function.
            _cala_call(offset, u32::MAX);
        }
    }
}*/
