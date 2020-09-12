// cala_core
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Web-Specific APIs

#![allow(unsafe_code)]

use std::{
    marker::PhantomData,
    cell::RefCell,
    collections::HashMap,
    future::Future,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::*;

thread_local! {
    // A map of the resolved promises / ready futures
    static READY: RefCell<HashMap<i32, JsVar>> = RefCell::new(HashMap::new())
}

// Run whenever a Promise resolves.
fn wake_internal(promise: i32, result: i32) {
    // Promise resolving marks the future as "ready" with a value in the map.
    READY.with(|w| w.borrow_mut().insert(promise, JsVar(result)));
    // Promise resolving wakes the thread, so run the executor.
    executor();
}

#[cfg(feature = "wasm-bindgen")]
#[doc = ""]
#[wasm_bindgen]
pub fn wake(promise: i32, result: i32) {
    wake_internal(promise, result);
}

#[cfg(not(feature = "wasm-bindgen"))]
#[no_mangle]
extern "C" fn wake(promise: i32, result: i32) {
    wake_internal(promise, result);
}

/// A JavaScript Promise
#[derive(Debug)]
pub struct JsPromise<T: From<JsVar>>(JsVar, PhantomData::<T>);

impl<T: From<JsVar>> JsPromise<T> {
    /// Poll a JavaScript Promise.
    pub fn poll(&self) -> Poll<T> {
        READY.with(|w| {
            if let Some(value) = w.borrow_mut().remove(&(self.0).0) {
                Poll::Ready(value.into())
            } else {
                Poll::Pending
            }
        })
    }
}

/// A JavaScript variable.
#[derive(Debug)]
pub struct JsVar(i32);

impl JsVar {
    /// Assume the JavaScript variable is a promise, and convert to a JsPromise.
    pub unsafe fn into_promise<T: From<JsVar>>(self) -> JsPromise<T> {
        self.set_waker_internal();
        JsPromise(self, PhantomData)
    }

    /// Assuming the JavaScript variable is a function with two parameters,
    /// convert into a `JsFn`.
    pub unsafe fn into_fn(self) -> JsFn {
        JsFn(self)
    }

    /// Create a new `JsVar` from a Rust integer
    pub fn from_i32(value: i32) -> JsVar {
        Self::from_i32_internal(value)
    }

    #[cfg(feature = "wasm-bindgen")]
    fn from_i32_internal(value: i32) -> Self {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_store_int(idx: i32) -> i32;
        }

        Self(_cala_js_store_int(value))
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    fn from_i32_internal(value: i32) -> Self {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_store_int(idx: i32) -> i32;
        }
        Self(unsafe { _cala_js_store_int(value) })
    }

    /// Get a Rust integer from a `JsVar`.
    pub unsafe fn into_i32(&self) -> i32 {
        Self::into_i32_internal(self)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn into_i32_internal(value: &JsVar) -> i32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_load_int(idx: i32) -> i32;
        }
        _cala_js_load_int(value.0)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn into_i32_internal(value: &JsVar) -> i32 {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_load_int(idx: i32) -> i32;
        }
        _cala_js_load_int(value.0)
    }

    /// Create a new `JsVar` from a Rust float
    pub fn from_f32(value: f32) -> JsVar {
        Self::from_f32_internal(value)
    }

    #[cfg(feature = "wasm-bindgen")]
    fn from_f32_internal(value: f32) -> Self {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_store_float(idx: f32) -> i32;
        }

        Self(_cala_js_store_float(value))
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    fn from_f32_internal(value: f32) -> Self {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_store_float(idx: f32) -> i32;
        }
        Self(unsafe { _cala_js_store_float(value) })
    }

    /// Get a Rust float from a `JsVar`.
    pub unsafe fn into_f32(&self) -> f32 {
        Self::into_f32_internal(self)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn into_f32_internal(value: &JsVar) -> f32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_load_float(idx: i32) -> f32;
        }
        _cala_js_load_float(value.0)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn into_f32_internal(value: &JsVar) -> f32 {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_load_float(idx: i32) -> f32;
        }
        _cala_js_load_float(value.0)
    }

    /// Create a new `JsVar` from a Rust float
    pub fn from_f64(value: f64) -> JsVar {
        Self::from_f64_internal(value)
    }

    #[cfg(feature = "wasm-bindgen")]
    fn from_f64_internal(value: f64) -> Self {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_store_double(idx: f64) -> i32;
        }

        Self(_cala_js_store_double(value))
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    fn from_f64_internal(value: f64) -> Self {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_store_double(idx: f64) -> i32;
        }
        Self(unsafe { _cala_js_store_double(value) })
    }

    /// Get a Rust float from a `JsVar`.
    pub unsafe fn into_f64(&self) -> f64 {
        Self::into_f64_internal(self)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn into_f64_internal(value: &JsVar) -> f64 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_load_double(idx: i32) -> f64;
        }
        _cala_js_load_double(value.0)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn into_f64_internal(value: &JsVar) -> f64 {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_load_double(idx: i32) -> f64;
        }
        _cala_js_load_double(value.0)
    }

    /// Assume the variable is a string and copy into Rust `Vec`.
    pub unsafe fn read_utf16(&self, output: &mut Vec<u16>) {
        let length = self.vecstr(output, 0); // Query size
        output.reserve_exact(length as usize - output.len());
        let written = self.vecstr(output, length); // Write data
        output.set_len(written as usize);
        debug_assert_eq!(length, written);
    }

    /// Assume the variable is an array and copy into Rust `Vec`.
    pub unsafe fn read_bytes(&self, output: &mut Vec<u8>) {
        let length = self.vec8i(output, 0); // Query size
        output.reserve_exact(length as usize - output.len());
        let written = self.vec8i(output, length); // Write data
        output.set_len(written as usize);
        debug_assert_eq!(length, written);
    }

    /// Assume the variable is an array and copy into Rust `Vec`.
    pub unsafe fn read_ints(&self, output: &mut Vec<i32>) {
        let length = self.vec32i(output, 0); // Query size
        output.reserve_exact(length as usize - output.len());
        let written = self.vec32i(output, length); // Write data
        output.set_len(written as usize);
        debug_assert_eq!(length, written);
    }

    /// Assume the variable is an array and copy into Rust `Vec`.
    pub unsafe fn read_floats(&self, output: &mut Vec<f32>) {
        let length = self.vec32f(output, 0); // Query size
        output.reserve_exact(length as usize - output.len());
        let written = self.vec32f(output, length); // Write data
        output.set_len(written as usize);
        debug_assert_eq!(length, written);
    }

    /// Assume the variable is an array and copy into Rust `Vec`.
    pub unsafe fn read_doubles(&self, output: &mut Vec<f64>) {
        let length = self.vec64f(output, 0); // Query size
        output.reserve_exact(length as usize - output.len());
        let written = self.vec64f(output, length); // Write data
        output.set_len(written as usize);
        debug_assert_eq!(length, written);
    }

    /// Assume the variable is an array and copy from Rust slice.
    pub unsafe fn write_bytes(&self, input: &[u8]) {
        self.slice8i(input);
    }

    /// Assume the variable is an array and copy from Rust slice.
    pub unsafe fn write_ints(&self, input: &[i32]) {
        self.slice32i(input);
    }

    /// Assume the variable is an array and copy from Rust slice.
    pub unsafe fn write_floats(&self, input: &[f32]) {
        self.slice32f(input);
    }

    /// Assume the variable is an array and copy from Rust slice.
    pub unsafe fn write_doubles(&self, input: &[f64]) {
        self.slice64f(input);
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec8i(&self, output: &mut Vec<u8>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read_bytes(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_bytes(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn vec8i(&self, output: &mut Vec<u8>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read_bytes(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_bytes(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec32i(&self, output: &mut Vec<i32>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read_ints(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_ints(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn vec32i(&self, output: &mut Vec<i32>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read_ints(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_ints(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec32f(&self, output: &mut Vec<f32>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read_floats(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_floats(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn vec32f(&self, output: &mut Vec<f32>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read_floats(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_floats(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vec64f(&self, output: &mut Vec<f64>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read_doubles(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_doubles(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn vec64f(&self, output: &mut Vec<f64>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read_doubles(j: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_doubles(self.0, output.as_mut_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn slice8i(&self, input: &[u8]) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_write_bytes(j: i32, p: u32, l: u32);
        }
        _cala_js_write_bytes(self.0, input.as_ptr() as u32, input.len() as u32)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn slice8i(&self, input: &[u8]) {
        extern "C" {
            fn _cala_js_write_bytes(j: i32, p: u32, l: u32) -> ();
        }
        _cala_js_write_bytes(self.0, input.as_ptr() as u32, input.len() as u32)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn slice32i(&self, input: &[i32]) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_write_ints(j: i32, p: u32, l: u32);
        }
        _cala_js_write_ints(self.0, input.as_ptr() as u32, input.len() as u32)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn slice32i(&self, input: &[i32]) {
        extern "C" {
            fn _cala_js_write_ints(j: i32, p: u32, l: u32) -> ();
        }
        _cala_js_write_ints(self.0, input.as_ptr() as u32, input.len() as u32)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn slice32f(&self, input: &[f32]) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_write_floats(j: i32, p: u32, l: u32);
        }
        _cala_js_write_floats(self.0, input.as_ptr() as u32, input.len() as u32)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn slice32f(&self, input: &[f32]) {
        extern "C" {
            fn _cala_js_write_floats(j: i32, p: u32, l: u32) -> ();
        }
        _cala_js_write_floats(self.0, input.as_ptr() as u32, input.len() as u32)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn slice64f(&self, input: &[f64]) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_write_doubles(j: i32, p: u32, l: u32);
        }
        _cala_js_write_doubles(
            self.0,
            input.as_ptr() as u32,
            input.len() as u32,
        )
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn slice64f(&self, input: &[f64]) {
        extern "C" {
            fn _cala_js_write_doubles(j: i32, p: u32, l: u32) -> ();
        }
        _cala_js_write_doubles(
            self.0,
            input.as_ptr() as u32,
            input.len() as u32,
        )
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn vecstr<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_read_text(idx: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_text(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn vecstr<T>(&self, output: &mut Vec<T>, length: u32) -> u32 {
        extern "C" {
            fn _cala_js_read_text(idx: i32, p: u32, l: u32) -> u32;
        }
        _cala_js_read_text(self.0, output.as_ptr() as u32, length)
    }

    #[cfg(feature = "wasm-bindgen")]
    unsafe fn set_waker_internal(&self) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_waker(idx: i32);
        }
        _cala_js_waker(self.0)
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    unsafe fn set_waker_internal(&self) {
        extern "C" {
            fn _cala_js_waker(idx: i32);
        }
        _cala_js_waker(self.0)
    }

    // Remove any state associated with ready promises on drop.
    fn drop_internal(&self) {
        READY.with(|w| w.borrow_mut().remove(&self.0));
    }
}

impl Drop for JsVar {
    #[cfg(feature = "wasm-bindgen")]
    fn drop(&mut self) {
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_free(idx: i32);
        }

        self.drop_internal();
        _cala_js_free(self.0);
    }

    #[cfg(not(feature = "wasm-bindgen"))]
    fn drop(&mut self) {
        extern "C" {
            // Free a JavaScript object
            fn _cala_js_free(idx: i32) -> ();
        }

        self.drop_internal();
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
        #[wasm_bindgen]
        extern "C" {
            fn _cala_js_text(p: u32, l: u32) -> i32;
        }

        // around the right amount of memory
        let mut text = Vec::with_capacity(string.len());
        for c in string.encode_utf16() {
            text.push(c);
        }
        JsString(JsVar(_cala_js_text(
            text.as_ptr() as u32,
            text.len() as u32,
        )))
    }

    /// Allocate a new javascript string from a Rust string slice.
    #[cfg(not(feature = "wasm-bindgen"))]
    pub fn new(string: &str) -> JsString {
        extern "C" {
            // Turn Rust UTF-16 String Into JavaScript String.
            fn _cala_js_text(p: u32, l: u32) -> i32;
        }

        // around the right amount of memory
        let mut text = Vec::with_capacity(string.len());
        for c in string.encode_utf16() {
            text.push(c);
        }
        JsString(JsVar(unsafe {
            _cala_js_text(text.as_ptr() as u32, text.len() as u32)
        }))
    }

    /// Get a reference to this JsString as a JsVar
    pub fn as_var(&self) -> &JsVar {
        &self.0
    }
    
    /// Turn a `JsVar` into a `JsString`.  This does no type-checking, therefore
    /// is unsafe.
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
    #[cfg(feature = "wasm-bindgen")]
    pub unsafe fn new(string: &str) -> JsFn {
        #[wasm_bindgen]
        extern "C" {
            // Execute some JavaScript string.
            fn _cala_js_function(idx: i32) -> i32;
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
    #[cfg(not(feature = "wasm-bindgen"))]
    pub unsafe fn new(string: &str) -> JsFn {
        extern "C" {
            // Execute some JavaScript string.
            fn _cala_js_function(idx: i32) -> i32;
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
    #[cfg(feature = "wasm-bindgen")]
    pub unsafe fn call(
        &self,
        a: Option<&JsVar>,
        b: Option<&JsVar>,
    ) -> Option<JsVar> {
        #[wasm_bindgen]
        extern "C" {
            // A generic javascript shim
            fn _cala_js_call(function: i32, param_a: i32, param_b: i32) -> i32;
        }
        let ret = _cala_js_call(
            (self.0).0,
            a.map(|x| x.0).unwrap_or(-1),
            b.map(|x| x.0).unwrap_or(-1),
        );
        if ret == -1 {
            None
        } else {
            Some(JsVar(ret))
        }
    }

    /// Call a JavaScript function.
    #[cfg(not(feature = "wasm-bindgen"))]
    pub unsafe fn call(
        &self,
        a: Option<&JsVar>,
        b: Option<&JsVar>,
    ) -> Option<JsVar> {
        extern "C" {
            // A generic javascript shim
            fn _cala_js_call(function: i32, param_a: i32, param_b: i32) -> i32;
        }
        let ret = _cala_js_call(
            (self.0).0,
            a.map(|x| x.0).unwrap_or(-1),
            b.map(|x| x.0).unwrap_or(-1),
        );
        if ret == -1 {
            None
        } else {
            Some(JsVar(ret))
        }
    }
}

thread_local!(
    static FUTURE: RefCell<Option<Pin<Box<dyn Future<Output = ()>>>>> =
        RefCell::new(None);
);

/// WASM is non-blocking, but after the function returns, the Future will be
/// running on the main (and only) thread inside the JavaScript executor.
pub fn block_on<F: Future<Output = ()> + 'static>(main: F) {
    FUTURE.with(move |future| {
        *future.borrow_mut() = Some(Box::pin(async {
            panic_hook();
            main.await;
        }));
    });
    executor();
}

fn executor() {
    FUTURE.with(|future| {
        if let Some(future) = future.borrow_mut().as_mut() {
            // Create a dummy context, nothing important
            let waker = waker();
            let mut cx = Context::from_waker(&waker);
            // Whether app exits or continues, we don't care.
            let _ = future.as_mut().poll(&mut cx);
        }
    });
}

/// Create a waker for the JavaScript executor - doesn't need any associated
/// state.
#[inline]
fn waker() -> Waker {
    #[inline]
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &RawWakerVTable::new(clone, wake, wake, drop))
    }

    #[inline]
    unsafe fn wake(_data: *const ()) {
        executor();
    }

    #[inline]
    unsafe fn drop(_data: *const ()) {}

    unsafe {
        Waker::from_raw(RawWaker::new(
            std::ptr::null(),
            &RawWakerVTable::new(clone, wake, wake, drop),
        ))
    }
}

fn panic_hook_internal(panic_info: &std::panic::PanicInfo<'_>) {
    let msg = panic_info.to_string();

    let message = JsString::new(&format!("Cala App panicked!: {:?}", msg));
    let eprint = unsafe { JsFn::new("throw new Error(param_a);") };
    unsafe {
        assert!(eprint.call(Some(message.as_var()), None).is_none());
    }
}

/// Set the panic hook for nicely printed `panic!`s.
pub fn panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |p| {
        hook(p);
        panic_hook_internal(p);
    }));
}
