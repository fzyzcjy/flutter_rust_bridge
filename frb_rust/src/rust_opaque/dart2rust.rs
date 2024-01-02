use super::RustOpaque;
use std::sync::Arc;

/// # Safety
///
/// This function should never be called manually.
/// Retrieving an opaque pointer from Dart is an implementation detail, so this
/// function is not guaranteed to be API-stable.
#[cfg(not(wasm))]
pub unsafe fn cst_decode_rust_opaque<T>(ptr: *const core::ffi::c_void) -> RustOpaque<T> {
    decode_rust_opaque_inner(ptr as _)
}

/// # Safety
///
/// This should never be called manually.
#[cfg(wasm)]
pub unsafe fn cst_decode_rust_opaque<T>(raw: wasm_bindgen::JsValue) -> RustOpaque<T> {
    #[cfg(target_pointer_width = "64")]
    {
        compile_error!("64-bit pointers are not supported.");
    }

    decode_rust_opaque_inner((raw.as_f64().unwrap() as usize) as _)
}

/// # Safety
///
/// This should never be called manually.
pub unsafe fn sse_decode_rust_opaque<T>(ptr: usize) -> RustOpaque<T> {
    decode_rust_opaque_inner(ptr as _)
}

/// # Safety
///
/// This should never be called manually.
unsafe fn decode_rust_opaque_inner<T>(ptr: *const T) -> RustOpaque<T> {
    assert!(!ptr.is_null());
    RustOpaque {
        arc: Arc::from_raw(ptr),
    }
}
