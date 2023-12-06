use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::Channel;
use crate::generalized_isolate::PortLike;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::BroadcastChannel;

pub type GeneralizedAutoDropDartPersistentHandle = wasm_bindgen::JsValue;

// TODO things below not migrated yet --------------------------------------------------------

// TODO rm?
// pub type GeneralizedDartPersistentHandle = wasm_bindgen::JsValue;
//
// #[derive(Debug)]
// pub struct DartOpaqueBase {
//     inner: JsValue,
// }
//
// impl DartOpaqueBase {
//     pub fn new(handle: JsValue) -> Self {
//         Self { inner: handle }
//     }
//
//     pub fn unwrap(self) -> JsValue {
//         self.inner
//     }
//
//     pub fn into_raw(self) -> *mut JsValue {
//         new_leak_box_ptr(self.inner)
//     }
// }

/// # Safety
///
/// Only for generated code. Never call manually.
#[wasm_bindgen]
pub unsafe fn get_dart_object(ptr: usize) -> JsValue {
    *box_from_leak_ptr(ptr as _)
}

/// # Safety
///
/// Only for generated code. Never call manually.
#[wasm_bindgen]
pub unsafe fn drop_dart_object(ptr: usize) {
    drop(box_from_leak_ptr::<JsValue>(ptr as _));
}
