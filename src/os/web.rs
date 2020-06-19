// cala_core
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Web-Specific APIs

use std::{
    cell::RefCell,
    collections::HashMap,
    future::Future,
    mem::MaybeUninit,
    pin::Pin,
    task::{Context, RawWaker, RawWakerVTable, Waker},
    thread::LocalKey,
};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

#[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
use stdweb::unstable::TryInto;

enum Promise {
    None,
    Waker(Waker),
    Value(JsVar),
}

impl Promise {
    fn take(&mut self) -> Promise {
        let mut ret = Promise::None;
        std::mem::swap(self, &mut ret);
        ret
    }
}

thread_local!(static WAKERS: RefCell<HashMap<u32, Promise>> = RefCell::new(HashMap::new()));

// Implementation of waking of `Future`s.
#[allow(unsafe_code)]
fn wake_internal(promise: u32, result: u32) {
    let result = JsVar(result);
    WAKERS.with(|w| {
        if let Some(wakers) = w.borrow_mut().get_mut(&promise) {
            if let Promise::Waker(_waker) = wakers.take() {
                *wakers = Promise::Value(result);
            }
        }
    });
}

/// Get the result of a JavaScript Promise
pub fn resolve_promise(promise: &JsVar) -> Option<JsVar> {
    WAKERS.with(|w| {
        if let Some(wakers) = w.borrow_mut().get_mut(&promise.0) {
            if let Promise::Value(value) = wakers.take() {
                Some(value)
            } else {
                None
            }
        } else {
            None
        }
    })
}

#[cfg(feature = "wasm-bindgen")]
#[wasm_bindgen]
pub fn wake(promise: u32, result: u32) {
    wake_internal(promise, result)
}

#[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
#[js_export]
fn wake(promise: u32, result: u32) {
    wake_internal(promise, result)
}

#[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
#[no_mangle]
#[doc = ""]
pub extern "C" fn wake(promise: u32, result: u32) {
    wake_internal(promise, result)
}

/// A JavaScript variable.
#[derive(Debug)]
pub struct JsVar(u32);

impl JsVar {
    #![allow(unsafe_code)]

    /// Create a new `JsVar` from a Rust integer
    pub fn from_u32(value: u32) -> JsVar {
        Self::from_u32_internal(value)
    }

    #[cfg(feature = "wasm-bindgen")]
    fn from_u32_internal(value: u32) -> Self {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_u32(idx: u32) -> u32;
        }

        Self(_cala_js_u32(value))
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    fn from_u32_internal(value: u32) -> Self {
        Self(
            js! {
                _cala_js_u32(@{value})
            }
            .try_into()
            .unwrap(),
        )
    }

    #[allow(unsafe_code)]
    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    fn from_u32_internal(value: u32) -> Self {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_u32(idx: u32) -> u32;
        }
        Self(unsafe { _cala_js_u32(value) })
    }

    /// Create a new `JsVar` from a Rust integer
    pub unsafe fn into_u32(&self) -> u32 {
        Self::into_u32_internal(self)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn into_u32_internal(value: &JsVar) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read_u32(idx: u32) -> u32;
        }
        _cala_js_read_u32(value.0)
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    unsafe fn into_u32_internal(value: &JsVar) -> u32 {
        let ret = js! {
            return _cala_js_read_u32(@{value.0});
        };
        ret.try_into().unwrap()
    }

    #[allow(unsafe_code)]
    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    unsafe fn into_u32_internal(value: &JsVar) -> u32 {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_read_u32(idx: u32) -> u32;
        }
        _cala_js_read_u32(value.0)
    }

    /// Assume the variable is a string and copy into Rust `Vec`.
    pub unsafe fn as_str<T>(&self) -> Vec<T> {
        match std::mem::size_of::<T>() {
            2 => {
                let mut output = Vec::new();
                let length = self.vec16(&mut output, 0); // Query size
                output.reserve_exact(length as usize);
                let written = self.vecstr(&mut output, length); // Write data
                output.set_len(written as usize);
                debug_assert_eq!(length, written);
                output
            }
            s => panic!("Bad data size ({}), want 2 (UTF-16)", s),
        }
    }

    /// Assume the variable is an array and copy into Rust `Vec`.
    pub unsafe fn as_vec<T>(&self) -> Vec<T> {
        match std::mem::size_of::<T>() {
            1 => {
                let mut output = Vec::new();
                let length = self.vec8(&mut output, 0); // Query size
                output.reserve_exact(length as usize);
                let written = self.vec8(&mut output, length); // Write data
                output.set_len(written as usize);
                debug_assert_eq!(length, written);
                output
            }
            2 => {
                let mut output = Vec::new();
                let length = self.vec16(&mut output, 0); // Query size
                output.reserve_exact(length as usize);
                let written = self.vec16(&mut output, length); // Write data
                output.set_len(written as usize);
                debug_assert_eq!(length, written);
                output
            }
            4 => {
                let mut output = Vec::new();
                let length = self.vec32(&mut output, 0); // Query size
                output.reserve_exact(length as usize);
                let written = self.vec32(&mut output, length); // Write data
                output.set_len(written as usize);
                debug_assert_eq!(length, written);
                output
            }
            s => panic!("Bad data size ({}), want 1, 2, or 4", s),
        }
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec8<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read8(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_read8(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    unsafe fn vec8<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        let ret = js! {
            return _cala_js_read8(
                @{self.0},
                @{output.as_ptr() as u32},
                @{length}
            );
        };
        ret.try_into().unwrap()
    }

    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    unsafe fn vec8<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read8(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_read8(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec16<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read16(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_read16(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    unsafe fn vec16<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        let ret = js! {
            return _cala_js_read16(
                @{self.0},
                @{output.as_ptr() as u32},
                @{length}
            );
        };
        ret.try_into().unwrap()
    }

    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    unsafe fn vec16<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read16(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_read16(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec32<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read32(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_read32(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    unsafe fn vec32<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        let ret = js! {
            return _cala_js_read32(
                @{self.0},
                @{output.as_ptr() as u32},
                @{length}
            );
        };
        ret.try_into().unwrap()
    }

    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    unsafe fn vec32<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read32(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_read32(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vecstr<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_readstr(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_readstr(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    unsafe fn vecstr<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        let ret = js! {
            return _cala_js_readstr(
                @{self.0},
                @{output.as_ptr() as u32},
                @{length}
            );
        };
        ret.try_into().unwrap()
    }

    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    unsafe fn vecstr<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_readstr(idx: u32, p: u32, l: u32) -> u32;
        }
        _cala_js_readstr(self.0, output.as_ptr() as u32, length)
    }

    /// Attach a `Waker` to this JavaScript `Promise`.  Marked unsafe because
    /// there's no guarantee it's actually a `Promise`.
    pub unsafe fn set_waker(&self, waker: Waker) {
        WAKERS.with(|w| w.borrow_mut().insert(self.0, Promise::Waker(waker)));
        self.set_waker_internal();
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn set_waker_internal(&self) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_waker(idx: u32);
        }
        _cala_js_waker(self.0)
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    unsafe fn set_waker_internal(&self) {
        let ret = js! { _cala_js_waker(@{self.0}); };
        ret.try_into().unwrap()
    }

    #[cfg(not(any(feature = "stdweb", feature = "wasm-bindgen")))]
    unsafe fn set_waker_internal(&self) {
        extern "C" {
            fn _cala_js_waker(idx: u32);
        }
        _cala_js_waker(self.0)
    }
}

impl Drop for JsVar {
    #[cfg(feature = "wasm-bindgen")]
    fn drop(&mut self) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_free(idx: u32);
        }

        _cala_js_free(self.0);
    }

    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
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
    #[cfg(feature = "wasm-bindgen")]
    pub fn new(string: &str) -> JsString {
        // around the right amount of memory
        let mut utf16 = Vec::with_capacity(string.len());
        for c in string.encode_utf16() {
            utf16.push(c);
        }
        //
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_string(p: u32, l: u32) -> u32;
        }
        let string = _cala_js_string(utf16.as_ptr() as u32, utf16.len() as u32);
        JsString(JsVar(string))
    }

    /// Allocate a new javascript string from a Rust string slice.
    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
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
pub struct JsFn(JsVar);

impl JsFn {
    /// Define a function (two parameters param_a: u32, and param_b: u32,
    /// returns a u32)
    #[allow(unsafe_code)]
    #[cfg(feature = "wasm-bindgen")]
    pub unsafe fn new(string: &str) -> JsFn {
        #[wasm_bindgen]
        extern "C" {
            // Execute some JavaScript string.
            fn _cala_js_function(idx: u32) -> u32;
        }

        let javascript = format!(
            "\
            \"use strict\";\
            return function(param_a, param_b) {{ {} }};\
        ",
            string
        );

        let string = JsString::new(&javascript);
        let func = _cala_js_function(string.as_var().0);

        JsFn(JsVar(func))
    }

    /// Define a function (two parameters param_a: u32, and param_b: u32,
    /// returns a u32)
    #[allow(unsafe_code)]
    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    pub unsafe fn new(string: &str) -> JsFn {
        let javascript = format!(
            "\
            \"use strict\";\
            return function(param_a, param_b) {{ {} }};\
        ",
            string
        );

        let string = JsString::new(&javascript);
        let func = js! {
            return _cala_js_function(@{string.as_var().0});
        };

        JsFn(JsVar(func.try_into().unwrap()))
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

        let javascript = format!(
            "\
            \"use strict\";\
            return function(param_a, param_b) {{ {} }};\
        ",
            string
        );

        let string = JsString::new(&javascript);
        let func = _cala_js_function(string.as_var().0);

        JsFn(JsVar(func))
    }

    /// Call a JavaScript function.
    #[allow(unsafe_code)]
    #[cfg(feature = "wasm-bindgen")]
    pub unsafe fn call(
        &self,
        a: Option<&JsVar>,
        b: Option<&JsVar>,
    ) -> Option<JsVar> {
        #[wasm_bindgen]
        extern "C" {
            // A generic javascript shim
            fn _cala_js_call(function: u32, param_a: u32, param_b: u32) -> u32;
        }
        let ret = _cala_js_call(
            (self.0).0,
            a.map(|x| x.0).unwrap_or(u32::MAX),
            b.map(|x| x.0).unwrap_or(u32::MAX),
        );
        if ret == u32::MAX {
            None
        } else {
            Some(JsVar(ret))
        }
    }

    /// Call a JavaScript function.
    #[allow(unsafe_code)]
    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    pub unsafe fn call(
        &self,
        a: Option<&JsVar>,
        b: Option<&JsVar>,
    ) -> Option<JsVar> {
        let ret = js! {
            return _cala_js_call(@{(self.0).0}, @{a.map(|x| x.0).unwrap_or(u32::MAX)}, @{b.map(|x| x.0).unwrap_or(u32::MAX)});
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
            fn _cala_js_call(function: u32, param_a: u32, param_b: u32) -> u32;
        }
        let ret = _cala_js_call(
            (self.0).0,
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

#[derive(Debug)]
struct JsExecState(
    &'static LocalKey<RefCell<Pin<Box<dyn Future<Output = ()>>>>>,
    MaybeUninit<Waker>,
);

/// A JavaScript Executor.
#[derive(Debug)]
pub struct JsExec {
    state: JsExecState,
}

impl JsExec {
    /// Create a new JavaScript executor for a specific future in thread local
    /// storage.
    pub fn new(
        future: &'static LocalKey<RefCell<Pin<Box<dyn Future<Output = ()>>>>>,
    ) -> JsExec {
        let mut state = JsExecState(future, MaybeUninit::uninit());
        let waker = waker(&state);
        state.1 = MaybeUninit::new(waker);
        JsExec { state }
    }
}

impl JsExec {
    /// Wake the executor in order to poll the Future.
    pub fn wake(&self) {
        self.state.trigger_event()
    }
}

impl JsExecState {
    #[allow(unsafe_code)]
    fn trigger_event(&self) {
        let mut cx = Context::from_waker(unsafe { &*self.1.as_ptr() });
        self.0.with(|future| {
            // Whether app exits or continues, we don't care.
            let _ = future.borrow_mut().as_mut().poll(&mut cx);
        });
    }
}

/// Create a waker for the JavaScript executor.
#[inline]
#[allow(unsafe_code)]
fn waker(executor: *const JsExecState) -> Waker {
    #[inline]
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &RawWakerVTable::new(clone, wake, wake, drop))
    }

    #[inline]
    unsafe fn wake(data: *const ()) {
        JsExecState::trigger_event(&*(data as *const JsExecState));
    }

    #[inline]
    unsafe fn drop(_data: *const ()) {}

    unsafe {
        Waker::from_raw(RawWaker::new(
            executor as *const (),
            &RawWakerVTable::new(clone, wake, wake, drop),
        ))
    }
}
