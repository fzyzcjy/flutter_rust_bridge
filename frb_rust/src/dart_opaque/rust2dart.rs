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
        (data.into_raw() as usize).into_dart()
    }
}

#[cfg(wasm)]
#[wasm_bindgen]
pub unsafe fn dart_opaque_rust2dart_decode(ptr: usize) -> GeneralizedDartHandle {
    dart_opaque_rust2dart_decode_inner(ptr)
}

#[cfg(not(wasm))]
#[no_mangle]
pub unsafe extern "C" fn dart_opaque_rust2dart_decode(ptr: usize) -> GeneralizedDartHandle {
    dart_opaque_rust2dart_decode_inner(ptr)
}

unsafe fn dart_opaque_rust2dart_decode_inner(ptr: usize) -> GeneralizedDartHandle {
    let opaque = DartOpaque::from_raw(ptr as _);
    opaque.create_dart_handle()
}
