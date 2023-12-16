use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-start")]
#[wasm_bindgen(start)]
pub fn wasm_start_callback() {
    console_error_panic_hook::set_once();
}
