use std::mem;

// ref https://stackoverflow.com/questions/39224904/how-to-expose-a-rust-vect-to-ffi
pub fn new_leak_vec_ptr<T: Clone>(fill: T, length: i32) -> *mut T {
    into_leak_vec_ptr(vec![fill; length as usize]).0
}

pub fn into_leak_vec_ptr<T: Clone>(mut v: Vec<T>) -> (*mut T, i32) {
    v.shrink_to_fit();
    assert_eq!(v.len(), v.capacity());
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

/// Convert `Vec<T>` to array length `N`.
///
/// # Panics
///
/// Panics if length of `Vec<T>` != `N`.
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
