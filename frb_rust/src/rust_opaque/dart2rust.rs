use super::{DartSafe, RustOpaque};
use std::sync::Arc;

/// # Safety
///
/// This function should never be called manually.
/// Retrieving an opaque pointer from Dart is an implementation detail, so this
/// function is not guaranteed to be API-stable.
#[cfg(not(wasm))]
pub unsafe fn cst_decode_rust_opaque<T: DartSafe>(ptr: *const core::ffi::c_void) -> RustOpaque<T> {
    cst_decode_rust_opaque_inner(ptr as _)
}

#[cfg(wasm)]
pub unsafe fn cst_decode_rust_opaque<T: DartSafe>(raw: wasm_bindgen::JsValue) -> RustOpaque<T> {
    #[cfg(target_pointer_width = "64")]
    {
        compile_error!("64-bit pointers are not supported.");
    }

    cst_decode_rust_opaque_inner((raw.as_f64().unwrap() as usize) as _)
}

unsafe fn cst_decode_rust_opaque_inner<T: DartSafe>(ptr: *const T) -> RustOpaque<T> {
    assert!(!ptr.is_null());
    RustOpaque {
        arc: Arc::from_raw(ptr),
    }
}
