use crate::for_generated::{into_leak_vec_ptr, new_leak_vec_ptr, vec_from_leak_ptr};

#[cfg(not(wasm))]
mod io;
#[cfg(not(wasm))]
pub use io::*;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[no_mangle]
pub unsafe extern "C" fn rust_vec_u8_new(len: i32) -> *mut u8 {
    new_leak_vec_ptr::<u8>(0, len)
}

#[no_mangle]
pub unsafe extern "C" fn rust_vec_u8_resize(ptr: *mut u8, old_len: i32, new_len: i32) -> *mut u8 {
    let mut vec = vec_from_leak_ptr(ptr, old_len);
    vec_resize(&mut vec, new_len);
    into_leak_vec_ptr(vec).0
}

fn vec_resize(vec: &mut Vec<u8>, new_len: i32) {
    let new_len = new_len as usize;
    if new_len > vec.len() {
        vec.reserve_exact(new_len - vec.len());
    }
    vec.resize(new_len, 0);
    debug_assert!(vec.len() == new_len);
}

#[no_mangle]
pub unsafe extern "C" fn rust_vec_u8_free(ptr: *mut u8, len: i32) {
    vec_from_leak_ptr(ptr, len);
}
