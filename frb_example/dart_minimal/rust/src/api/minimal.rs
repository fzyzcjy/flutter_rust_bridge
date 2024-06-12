use flutter_rust_bridge::frb;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// TODO temp
// #[frb(init)]
// pub fn init_app() {
//     flutter_rust_bridge::setup_default_user_utils();
// }

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug, Clone)]
pub struct MyTreeNodeTwinNormal {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNodeTwinNormal>,
}

pub fn handle_complex_struct_twin_normal(s: MyTreeNodeTwinNormal) -> MyTreeNodeTwinNormal {
    // info!("handle_complex_struct({:?})", &s);
    let _s_cloned = s.clone();
    s
}

// #[wasm_bindgen]
// pub fn rust_wasm_bindgen_func() {
//     flutter_rust_bridge::console_error!("rust_wasm_bindgen_func start");
//     flutter_rust_bridge::console_error!("rust_wasm_bindgen_func end");
// }
