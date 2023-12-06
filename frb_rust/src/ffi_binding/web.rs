use crate::for_generated::box_from_leak_ptr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[cfg(feature = "wasm-start")]
#[wasm_bindgen(start)]
pub fn wasm_start_callback() {
    console_error_panic_hook::set_once();
}
