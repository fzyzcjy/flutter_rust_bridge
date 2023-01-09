use super::*;

#[wasm_bindgen(start)]
pub fn _register_logger() {
    console_error_panic_hook::set_once();
    _ = console_log::init();
}
