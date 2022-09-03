//! Functions that support auto-generated Rust code.
//! These functions are *not* meant to be used by humans directly.
#![doc(hidden)]

use std::mem;

pub use allo_isolate::ffi::DartCObject;
pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
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

/// NOTE for maintainer: Please keep this struct in sync with `DUMMY_WIRE_CODE_FOR_BINDGEN`
/// in the code generator
#[repr(C)]
pub struct WireSyncReturnStruct {
    pub ptr: *mut u8,
    pub len: i32,
    pub success: bool,
}

impl From<WireSyncReturnData> for WireSyncReturnStruct {
    fn from(data: WireSyncReturnData) -> Self {
        let (ptr, len) = into_leak_vec_ptr(data.0);
        WireSyncReturnStruct {
            ptr,
            len,
            success: true,
        }
    }
}

/// Wrapper struct for [`WireSyncReturnStruct`].
pub struct WireSyncReturnData(Vec<u8>);

impl From<Vec<u8>> for WireSyncReturnData {
    fn from(data: Vec<u8>) -> Self {
        WireSyncReturnData(data)
    }
}

/// Bool will be converted to u8 where 0 stands for false and 1 stands for true.
impl From<bool> for WireSyncReturnData {
    fn from(data: bool) -> Self {
        if data { 1_u8 } else { 0_u8 }.to_be_bytes().to_vec().into()
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

// For simple types, use macro to implement [`From`] trait.
primitive_to_sync_return!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);
