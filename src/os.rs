// cala_core
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Operating System Specific APIs

#![allow(non_camel_case_types, trivial_numeric_casts, clippy::useless_conversion)]

#[cfg(all(unix, not(target_arch = "wasm32")))]
pub use std::os::unix::prelude::*;

use std::{convert::TryFrom, os::{raw}};

#[cfg(any(feature = "docs-rs", target_arch = "wasm32"))]
pub mod web;

/// File Descriptor (Unix), Socket (Windows) or Promise (Web)
#[repr(transparent)]
#[derive(Debug)]
pub struct Connection(
    #[cfg(all(unix, not(target_arch = "wasm32")))]
    RawFd,
    #[cfg(all(windows, not(target_arch = "wasm32")))]
    os::windows::io::RawSocket,
    #[cfg(target_arch = "wasm32")]
    u32,
);

#[cfg(all(unix, not(target_arch = "wasm32")))]
impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl Drop for Connection {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        #[cfg(all(unix, not(target_arch = "wasm32")))]
        {
            extern "C" {
                fn close(fd: Connection) -> c_sint;
            }
            let _ = unsafe {
                close(Self(self.0))
            };
        }
        #[cfg(all(windows, not(target_arch = "wasm32")))]
        {
            todo!()
        }
        #[cfg(target_arch = "wasm32")]
        {
            todo!()
        }
    }
}

/// C `void`
#[repr(transparent)]
#[derive(Debug)]
pub struct c_void(raw::c_void);

/// C `char` (no sign)
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_char(raw::c_char);
/// C `signed char`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_schar(raw::c_schar);
/// C `unsigned char`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_uchar(raw::c_uchar);

/// C `short`, `signed short`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_sshort(raw::c_short);
/// C `unsigned short`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_ushort(raw::c_ushort);

/// C `int`, `signed int`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_sint(raw::c_int);
/// C `unsigned int`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_uint(raw::c_uint);

/// C `long`, `signed long`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_slong(raw::c_long);
/// C `unsigned long`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_ulong(raw::c_ulong);

/// C `long`, `signed long long`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_slonglong(raw::c_longlong);
/// C `unsigned long long`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_ulonglong(raw::c_ulonglong);

/// C `size_t`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_usize(usize);
/// C `ssize_t`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_ssize(isize);

/// C `float`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_float(raw::c_float);
/// C `double`
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct c_double(raw::c_double);

/// C `const T*`.  Doesn't implement `Send` or `Sync`.
#[repr(transparent)]
#[derive(Debug)]
pub struct c_ref<T>(*const T);

impl<T> c_ref<T> {
    /// Wrap a raw C pointer in a newtype.
    pub fn from_raw(pointer: *const T) -> Self {
        Self(pointer)
    }

    /// Get an immutable reference from the pointer.
    #[allow(unsafe_code)]
    pub fn get(&self) -> &T {
        unsafe { &*self.0 }
    }

    /// Use a closure to free the memory pointer.
    pub fn free<F: FnOnce(*const T)>(self, f: F) {
        f(self.0);
        let mut this = self;
        this.0 = std::ptr::null_mut();
    }

    /// Run closure with a reference to the data in the pointer.
    #[allow(unsafe_code)]
    pub fn with<R, F: FnOnce(&T) -> R>(&self, f: F) -> R {
        f(unsafe { &*self.0 })
    }
}

/// C `T*` that must be free'd.  Doesn't implement `Send` or `Sync`.
#[repr(transparent)]
#[derive(Debug)]
pub struct c_ptr<T>(*mut T);

impl<T> c_ptr<T> {
    /// Wrap a raw C pointer in a newtype.
    pub fn from_raw(pointer: *mut T) -> Self {
        Self(pointer)
    }

    /// Get an immutable reference from the pointer.
    #[allow(unsafe_code)]
    pub fn get(&self) -> &T {
        unsafe { &*self.0 }
    }

    /// Get a mutable reference from the pointer.
    #[allow(unsafe_code)]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.0 }
    }

    /// Use a closure to free the memory pointer.
    pub fn free<F: FnOnce(*mut T)>(self, f: F) {
        f(self.0);
        let mut this = self;
        this.0 = std::ptr::null_mut();
    }

    /// Run closure with a reference to the data in the pointer.
    #[allow(unsafe_code)]
    pub fn with<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        f(unsafe { &mut *self.0 })
    }
}

impl<T> Drop for c_ptr<T> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            panic!("Raw pointer has been leaked!");
        }
    }
}

impl Default for c_char {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_schar {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_uchar {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_sshort {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_ushort {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_sint {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_uint {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_slong {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_ulong {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_slonglong {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_ulonglong {
    fn default() -> Self {
        Self(0)
    }
}

impl Default for c_float {
    fn default() -> Self {
        Self(0.0)
    }
}

impl Default for c_double {
    fn default() -> Self {
        Self(0.0)
    }
}

impl From<u8> for c_uchar {
    fn from(other: u8) -> Self {
        Self(other as _)
    }
}

impl From<i8> for c_schar {
    fn from(other: i8) -> Self {
        Self(other as _)
    }
}

impl From<u16> for c_ushort {
    fn from(other: u16) -> Self {
        Self(other as _)
    }
}

impl From<i16> for c_sshort {
    fn from(other: i16) -> Self {
        Self(other as _)
    }
}

impl From<u16> for c_uint {
    fn from(other: u16) -> Self {
        Self(other as _)
    }
}

impl From<i16> for c_sint {
    fn from(other: i16) -> Self {
        Self(other as _)
    }
}

impl From<u32> for c_ulong {
    fn from(other: u32) -> Self {
        Self(other as _)
    }
}

impl From<i32> for c_slong {
    fn from(other: i32) -> Self {
        Self(other as _)
    }
}

impl From<u64> for c_ulonglong {
    fn from(other: u64) -> Self {
        Self(other as _)
    }
}

impl From<i64> for c_slonglong {
    fn from(other: i64) -> Self {
        Self(other as _)
    }
}

impl TryFrom<c_uchar> for u8 {
    type Error = ();

    fn try_from(other: c_uchar) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_schar> for i8 {
    type Error = ();

    fn try_from(other: c_schar) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_ushort> for u16 {
    type Error = ();

    fn try_from(other: c_ushort) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_sshort> for i16 {
    type Error = ();

    fn try_from(other: c_sshort) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_uint> for u32 {
    type Error = ();

    fn try_from(other: c_uint) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_sint> for i32 {
    type Error = ();

    fn try_from(other: c_sint) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_ulong> for u64 {
    type Error = ();

    fn try_from(other: c_ulong) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_slong> for i64 {
    type Error = ();

    fn try_from(other: c_slong) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_ulonglong> for u128 {
    type Error = ();

    fn try_from(other: c_ulonglong) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<c_slonglong> for i128 {
    type Error = ();

    fn try_from(other: c_slonglong) -> Result<Self, ()> {
        Self::try_from(other.0).map_err(|_| ())
    }
}

impl TryFrom<u32> for c_uint {
    type Error = ();

    fn try_from(other: u32) -> Result<Self, ()> {
        Ok(Self(raw::c_uint::try_from(other).map_err(|_| ())?))
    }
}

impl TryFrom<i32> for c_sint {
    type Error = ();

    fn try_from(other: i32) -> Result<Self, ()> {
        Ok(Self(raw::c_int::try_from(other).map_err(|_| ())?))
    }
}

impl TryFrom<f32> for c_float {
    type Error = ();

    fn try_from(other: f32) -> Result<Self, ()> {
        Ok(Self(raw::c_float::try_from(other).map_err(|_| ())?))
    }
}

impl TryFrom<f64> for c_double {
    type Error = ();

    fn try_from(other: f64) -> Result<Self, ()> {
        Ok(Self(raw::c_double::try_from(other).map_err(|_| ())?))
    }
}

impl TryFrom<c_float> for f32 {
    type Error = ();

    fn try_from(other: c_float) -> Result<Self, ()> {
        Ok(f32::try_from(other.0).map_err(|_| ())?)
    }
}

impl TryFrom<c_double> for f64 {
    type Error = ();

    fn try_from(other: c_double) -> Result<Self, ()> {
        Ok(f64::try_from(other.0).map_err(|_| ())?)
    }
}

impl From<usize> for c_usize {
    fn from(other: usize) -> Self {
        Self(other)
    }
}

impl From<isize> for c_ssize {
    fn from(other: isize) -> Self {
        Self(other)
    }
}

impl From<c_usize> for usize {
    fn from(other: c_usize) -> Self {
        other.0
    }
}

impl From<c_ssize> for isize {
    fn from(other: c_ssize) -> Self {
        other.0
    }
}
