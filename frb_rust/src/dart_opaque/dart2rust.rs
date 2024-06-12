use super::{DartOpaque, GeneralizedDartHandle};
use crate::platform_types::{message_port_to_handle, MessagePort};
#[cfg(wasm)]
use wasm_bindgen::prelude::*;

/// # Safety
///
/// This should never be called manually.
#[cfg(wasm)]
pub unsafe fn cst_decode_dart_opaque(raw: wasm_bindgen::JsValue) -> DartOpaque {
    #[cfg(target_pointer_width = "64")]
    {
        compile_error!("64-bit pointers are not supported.");
    }

    DartOpaque::from_raw((raw.as_f64().unwrap() as usize) as _)
}

/// # Safety
///
/// This should never be called manually.
#[cfg(not(wasm))]
pub unsafe fn cst_decode_dart_opaque(raw: usize) -> DartOpaque {
    DartOpaque::from_raw(raw as _)
}

/// # Safety
///
/// This should never be called manually.
pub unsafe fn sse_decode_dart_opaque(raw: usize) -> DartOpaque {
    DartOpaque::from_raw(raw as _)
}

/// # Safety
///
/// This should never be called manually.
#[cfg(wasm)]
#[wasm_bindgen]
pub unsafe fn dart_opaque_dart2rust_encode(
    handle: GeneralizedDartHandle,
    dart_handler_port: MessagePort,
) -> usize {
    dart_opaque_dart2rust_encode_inner(handle, dart_handler_port) as _
}

/// # Safety
///
/// This should never be called manually.
#[cfg(not(wasm))]
#[no_mangle]
pub unsafe extern "C" fn dart_opaque_dart2rust_encode(
    handle: GeneralizedDartHandle,
    dart_handler_port: MessagePort,
) -> *const std::ffi::c_void {
    dart_opaque_dart2rust_encode_inner(handle, dart_handler_port)
}

unsafe fn dart_opaque_dart2rust_encode_inner(
    handle: GeneralizedDartHandle,
    dart_handler_port: MessagePort,
) -> *const std::ffi::c_void {
    DartOpaque::new(handle, message_port_to_handle(&dart_handler_port)).into_raw()
}
