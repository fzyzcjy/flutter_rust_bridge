use flutter_rust_bridge::frb;
use std::rc::Rc;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn minimal_non_send_async() -> i32 {
    let value = Rc::new(42);
    std::future::ready(()).await;
    *value
}
