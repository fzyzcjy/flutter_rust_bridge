//! Functions that support auto-generated Rust code.
//! These functions are *not* meant to be used by humans directly.

use std::mem;

pub use allo_isolate::ffi::DartCObject;
pub use allo_isolate::IntoDart;
pub use lazy_static::lazy_static;

pub use crate::handler::SimpleHandler;

// ref https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
pub fn new_leak_vec_ptr<T: Clone>(fill: T, length: i32) -> *mut T {
    let mut v = vec![fill; length as usize];
    v.shrink_to_fit();
    assert!(v.len() == v.capacity());
    let ptr = v.as_mut_ptr();
    mem::forget(v);
    ptr
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
