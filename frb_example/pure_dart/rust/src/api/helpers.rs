use super::*;

// wasm-bindgen only allows one "start" function,
// so disable flutter_rust_bridge's and inject our own.
#[cfg(not(feature = "wasm-start"))]
#[wasm_bindgen(start)]
pub fn _register_logger() {
    console_error_panic_hook::set_once();
    _ = console_log::init();
}
