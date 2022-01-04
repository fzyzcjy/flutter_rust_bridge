//! Functions that support auto-generated Rust code.
//! These functions are *not* meant to be used by humans directly.

use std::ffi::c_void;
use std::mem::{self, ManuallyDrop};

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

/// NOTE for maintainer: Please keep this struct in sync with [DUMMY_WIRE_CODE_FOR_BINDGEN]
/// in the code generator
#[repr(C)]
pub struct WireSyncReturnStruct {
    pub ptr: *mut u8,
    pub len: i32,
    pub success: bool,
}

/// A wrapper to transfer ownership of T to Dart's side.
pub struct Opaque<T> {
    pub ptr: *mut T,
    pub drop: unsafe extern "C" fn(*mut T),
}

impl<T> Opaque<T> {
    #[inline]
    fn is_valid(&self) -> bool {
        !self.ptr.is_null()
    }
    #[inline]
    pub fn as_ref(&self) -> Option<&T> {
        self.is_valid().then(|| unsafe { &*self.ptr })
    }
    /// ## Safety
    /// It is instant UB to drop the underlying memory,
    /// e.g. via `std::mem::drop_in_place`, as it is still
    /// being held by Dart.
    #[inline]
    pub unsafe fn as_mut(&mut self) -> Option<&mut T> {
        self.is_valid().then(|| &mut *self.ptr)
    }
    pub fn new(value: T) -> Self {
        let mut val = ManuallyDrop::new(value);
        let ptr: &mut T = &mut *val;
        unsafe extern "C" fn drop<T>(ptr: *mut T) {
            let ptr: &mut T = &mut *ptr;
            core::ptr::drop_in_place(ptr);
        }
        Self { ptr, drop }
    }
    /// ## Safety
    /// This function should not be called manually.
    /// The method of receiving an opaque pointer from Dart
    /// is an implementation detail, so this signature is not API-stable.
    pub unsafe fn from_dart(ptr: *mut T, drop: *const unsafe extern "C" fn(*mut T)) -> Self {
        Self { ptr, drop: *drop }
    }
}

impl<T> IntoDart for Opaque<T> {
    fn into_dart(self) -> DartCObject {
        let drop: *const _ = &self.drop;
        vec![self.ptr.into_dart(), drop.into_dart()].into_dart()
    }
}
