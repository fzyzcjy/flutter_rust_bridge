use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::Channel;
use crate::generalized_isolate::PortLike;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::BroadcastChannel;

#[derive(Debug, Clone)]
pub struct GeneralizedAutoDropDartPersistentHandle(wasm_bindgen::JsValue);
/// cbindgen:ignore
pub type GeneralizedDartHandle = wasm_bindgen::JsValue;

impl GeneralizedAutoDropDartPersistentHandle {
    pub fn new_from_non_persistent_handle(non_persistent_handle: GeneralizedDartHandle) -> Self {
        Self(non_persistent_handle)
    }

    pub fn create_dart_handle(&self) -> GeneralizedDartHandle {
        self.0.clone()
    }
}
