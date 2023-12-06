use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use dart_sys::Dart_Handle;
use dart_sys::Dart_NewPersistentHandle_DL;
use log::warn;
use std::thread::ThreadId;

impl From<DartOpaque> for DartAbi {
    fn from(data: DartOpaque) -> Self {
        (new_leak_box_ptr(data) as usize).into_dart()
    }
}

// TODO rename
#[no_mangle]
pub unsafe extern "C" fn get_dart_object(ptr: usize) -> Dart_Handle {
    let value: DartOpaque = box_from_leak_ptr(ptr as _);
    handle.create_dart_handle()
}
