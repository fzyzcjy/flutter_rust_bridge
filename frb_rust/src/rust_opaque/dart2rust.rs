use super::{DartSafe, RustOpaque};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;
use std::{mem, ops};

/// # Safety
///
/// This function should never be called manually.
/// Retrieving an opaque pointer from Dart is an implementation detail, so this
/// function is not guaranteed to be API-stable.
#[cfg(not(wasm))]
pub unsafe fn wire2api_rust_opaque<T: DartSafe>(ptr: *const core::ffi::c_void) -> RustOpaque<T> {
    wire2api_rust_opaque_inner(ptr as _)
}

#[cfg(wasm)]
pub unsafe fn wire2api_rust_opaque<T: DartSafe>(raw: wasm_bindgen::JsValue) -> RustOpaque<T> {
    #[cfg(target_pointer_width = "64")]
    {
        compile_error!("64-bit pointers are not supported.");
    }

    wire2api_rust_opaque_inner((raw.as_f64().unwrap() as usize) as _)
}

unsafe fn wire2api_rust_opaque_inner<T: DartSafe>(ptr: *const T) -> RustOpaque<T> {
    // The raw pointer is the same one created from Arc::into_raw,
    // owned and artificially incremented by Dart.
    RustOpaque {
        ptr: (!ptr.is_null()).then(|| Arc::from_raw(ptr)),
    }
}

pub unsafe fn rust_opaque_arc_incr_count<T>(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    Arc::<T>::increment_strong_count(ptr as _);
    ptr
}

pub unsafe fn rust_opaque_arc_decr_count<T>(ptr: *const std::ffi::c_void) {
    Arc::<T>::decrement_strong_count(ptr as _);
}
