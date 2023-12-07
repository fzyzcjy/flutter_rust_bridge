use super::{DartOpaque, GeneralizedDartHandle};
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use crate::Handler;
use log::warn;
use std::thread::ThreadId;

#[cfg(wasm)]
pub unsafe fn cst_decode_dart_opaque<H: Handler>(
    handler: &H,
    raw: wasm_bindgen::JsValue,
) -> DartOpaque {
    let drop_port = handler.dart_opaque_drop_port();
    DartOpaque::new(raw, drop_port)
}

#[cfg(not(wasm))]
pub unsafe fn cst_decode_dart_opaque(raw: *const std::ffi::c_void) -> DartOpaque {
    DartOpaque::from_raw(raw)
}

#[cfg(not(wasm))]
pub unsafe fn dart_opaque_dart2rust_encode<H: Handler>(
    handler: &H,
    handle: dart_sys::Dart_Handle,
) -> *const std::ffi::c_void {
    let drop_port = handler.dart_opaque_drop_port();
    DartOpaque::new(handle, drop_port).into_raw()
}
