use flutter_rust_bridge::frb;
use wasm_bindgen;
use wasm_bindgen::prelude::*;

// #[frb(init)]
// pub fn init_app() {
//     flutter_rust_bridge::setup_default_user_utils();
// }

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn f(a: Vec<u64>) -> Vec<u64> {
    a
}

#[wasm_bindgen]
pub fn my_rust_function(
    // message_port: web_sys::MessagePort,
    message_port: flutter_rust_bridge::for_generated::MessagePort,
) {
    flutter_rust_bridge::console_error!("my_rust_function start message_port={:?}", message_port);
    let msg: JsValue = "hello".into();
    message_port.post_message(&msg).unwrap();
    flutter_rust_bridge::console_error!("my_rust_function end");
}
