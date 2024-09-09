use super::{DartOpaque, GeneralizedDartHandle};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
#[cfg(wasm)]
use wasm_bindgen::prelude::*;

impl From<DartOpaque> for DartAbi {
    fn from(data: DartOpaque) -> Self {
        data.encode().into_dart()
    }
}

impl DartOpaque {
    pub fn encode(self) -> usize {
        self.into_raw() as usize
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
