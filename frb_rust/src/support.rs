//! Functions that support auto-generated Rust code.
//! These functions are *not* meant to be used by humans directly.
#![doc(hidden)]

use std::{mem, sync::Arc};

pub use crate::ffi::*;
pub use lazy_static::lazy_static;

pub use crate::handler::DefaultHandler;

// ref https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
pub fn new_leak_vec_ptr<T: Clone>(fill: T, length: i32) -> *mut T {
    into_leak_vec_ptr(vec![fill; length as usize]).0
}

pub fn into_leak_vec_ptr<T: Clone>(mut v: Vec<T>) -> (*mut T, i32) {
    v.shrink_to_fit();
    assert!(v.len() == v.capacity());
    let ptr = v.as_mut_ptr();
    let len = v.len() as i32;
    mem::forget(v);
    (ptr, len)
}

/// # Safety
/// Use it in pair with [new_leak_vec_ptr].
pub unsafe fn vec_from_leak_ptr<T>(ptr: *mut T, len: i32) -> Vec<T> {
    Vec::from_raw_parts(ptr, len as usize, len as usize)
}

/// Convert [Vec<T>] to array length `N`.
///
/// # Panics
///
/// Panics if length of [Vec<T>] != `N`.
pub fn from_vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    core::convert::TryInto::try_into(v)
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

// ref: doc of [Box::into_raw]
pub fn new_leak_box_ptr<T>(t: T) -> *mut T {
    let x: Box<T> = Box::new(t);
    Box::into_raw(x)
}

/// # Safety
/// Use it in pair with [new_leak_box_ptr].
pub unsafe fn box_from_leak_ptr<T>(ptr: *mut T) -> Box<T> {
    Box::from_raw(ptr)
}

/// Cast a byte buffer into a boxed slice of the target type without making any copies.
/// Panics if the cast fails.
pub fn slice_from_byte_buffer<T: bytemuck::Pod>(buffer: Vec<u8>) -> Box<[T]> {
    let buf = Box::leak(buffer.into_boxed_slice());
    match bytemuck::try_cast_slice_mut(buf) {
        Ok(buf) => unsafe { Box::from_raw(buf) },
        Err(err) => {
            // clean up before panicking
            unsafe { core::ptr::drop_in_place(buf) }
            panic!("cast error: {}", err);
        }
    }
}

/// NOTE for maintainer: Please keep this struct in sync with `DUMMY_WIRE_CODE_FOR_BINDGEN`
/// in the code generator
#[repr(C)]
#[cfg(not(wasm))]
pub struct WireSyncReturnStruct {
    pub ptr: *mut u8,
    pub len: i32,
    pub success: bool,
}

#[cfg(wasm)]
pub type WireSyncReturnStruct = wasm_bindgen::JsValue;

/// Safe version of [`WireSyncReturnStruct`].
pub struct WireSyncReturnData(pub(crate) Vec<u8>);

impl From<Vec<u8>> for WireSyncReturnData {
    fn from(data: Vec<u8>) -> Self {
        WireSyncReturnData(data)
    }
}

/// Bool will be converted to u8 where 0 stands for false and 1 stands for true.
impl From<bool> for WireSyncReturnData {
    fn from(data: bool) -> Self {
        if data { 1_u8 } else { 0_u8 }.into()
    }
}

/// String will be converted to UTF-8 bytes.
impl From<String> for WireSyncReturnData {
    fn from(data: String) -> Self {
        data.as_bytes().to_vec().into()
    }
}

/// Macro for implementing [`From<Primitive>`] for [`WireSyncReturnData`].
/// This conversion won't fail.
macro_rules! primitive_to_sync_return {
    ($($t:ty),+) => {
        $(impl From<$t> for WireSyncReturnData {
            fn from(data: $t) -> Self {
                data.to_be_bytes().to_vec().into()
            }
        })*
    }
}

/// # Safety
/// This function should never be called manually.
/// Retrieving an opaque pointer from Dart is an implementation detail,
/// so this function is not guaranteed to be API-stable.
pub unsafe fn opaque_from_dart<T>(ptr: *const T) -> Opaque<T> {
    // The raw pointer is the same one created from Arc::into_raw,
    // owned and artificially incremented by Dart.
    Opaque {
        ptr: (!ptr.is_null()).then(|| Arc::from_raw(ptr)),
    }
}

// For simple types, use macro to implement [`From`] trait.
primitive_to_sync_return!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
