use super::DartOpaque;
use crate::dart_opaque::DartOpaqueWireType;
use crate::for_generated::new_leak_box_ptr;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use dart_sys::Dart_Handle;
use dart_sys::Dart_NewPersistentHandle_DL;
use log::warn;
use std::thread::ThreadId;

// TODO the api2wire side: just send the object itself, nothing more
pub unsafe fn wire2api_dart_opaque(
    raw: DartOpaqueWireType,
    drop_port: SendableMessagePortHandle,
) -> DartOpaque {
    DartOpaque::new(raw, drop_port)
}

#[no_mangle]
pub unsafe extern "C" fn dart_opaque_dart2rust_api2wire(handle: Dart_Handle) -> usize {
    new_leak_box_ptr(DartOpaque::new(handle, TODO))
}
