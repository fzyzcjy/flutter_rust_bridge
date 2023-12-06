use super::{DartOpaque, GeneralizedDartHandle};
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use log::warn;
use std::thread::ThreadId;
#[cfg(wasm)]
use wasm_bindgen::prelude::*;

impl From<DartOpaque> for DartAbi {
    fn from(data: DartOpaque) -> Self {
        (new_leak_box_ptr(data) as usize).into_dart()
    }
}

// TODO old name: `get_dart_object`, rename all users
#[wasm_bindgen]
#[no_mangle]
pub unsafe extern "C" fn dart_opaque_rust2dart_wire2api(ptr: usize) -> GeneralizedDartHandle {
    let value: DartOpaque = *box_from_leak_ptr(ptr as _);
    value.create_dart_handle()
}
