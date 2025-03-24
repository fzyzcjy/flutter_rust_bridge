use crate::for_generated::{into_leak_vec_ptr, new_leak_vec_ptr, vec_from_leak_ptr};

#[cfg(not(target_family = "wasm"))]
mod io;
#[cfg(not(target_family = "wasm"))]
#[allow(unused)]
pub use io::*;

#[cfg(target_family = "wasm")]
mod web;
#[cfg(target_family = "wasm")]
#[allow(unused)]
pub use web::*;

#[no_mangle]
pub unsafe extern "C" fn frb_rust_vec_u8_new(len: i32) -> *mut u8 {
    new_leak_vec_ptr::<u8>(0, len)
}

#[no_mangle]
pub unsafe extern "C" fn frb_rust_vec_u8_resize(
    ptr: *mut u8,
    old_len: i32,
    new_len: i32,
) -> *mut u8 {
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
    // This will stop the whole generator and tell the users, so we do not care about testing it
    // frb-coverage:ignore-start
    debug_assert!(vec.len() == new_len);
    // frb-coverage:ignore-end
}

#[no_mangle]
pub unsafe extern "C" fn frb_rust_vec_u8_free(ptr: *mut u8, len: i32) {
    vec_from_leak_ptr(ptr, len);
}

#[cfg(not(target_family = "wasm"))]
pub mod shutdown {
    use allo_isolate::{
        ffi::{DartCObject, DartPort},
        store_dart_post_cobject,
    };
    use std::ffi::c_void;

    /// Called by Dart's `NativeFinalizer` on isolate group shutdown.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn frb_shutdown_callback(_: *mut c_void) {
        unsafe extern "C" fn devnull(_: DartPort, _: *mut DartCObject) -> bool {
            // Returning true is wrong since message is not enqueued and this
            // might cause memory leaks. But since application is shutting down
            // we don't really care and just want it to die silently without
            // triggering any send errors.
            true
        }

        // So `Dart_PostCObject` won't do anything from now on. We need this
        // cause once shutdown have started `Dart_Cleanup` might be called any
        // moment from now and `Dart_PostCObject` can only be used before
        // `Dart_Cleanup` has been called
        // For more information refer to:
        // https://github.com/dart-lang/native/issues/2079
        store_dart_post_cobject(devnull);
    }
}
