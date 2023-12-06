use super::DartOpaque;
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use crate::Handler;
use dart_sys::Dart_Handle;
use log::warn;
use std::thread::ThreadId;

pub unsafe fn wire2api_dart_opaque(raw: *const std::ffi::c_void) -> DartOpaque {
    *box_from_leak_ptr(raw as _)
}

pub unsafe fn dart_opaque_dart2rust_api2wire<H: Handler>(
    handler: &H,
    handle: Dart_Handle,
) -> *const std::ffi::c_void {
    let drop_port = handler.dart_opaque_drop_port();
    new_leak_box_ptr(DartOpaque::new(handle, drop_port)) as _
}
