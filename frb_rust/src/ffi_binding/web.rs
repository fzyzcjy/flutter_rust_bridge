use crate::for_generated::box_from_leak_ptr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

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

#[cfg(feature = "wasm-start")]
#[wasm_bindgen(start)]
pub fn wasm_start_callback() {
    console_error_panic_hook::set_once();
}
