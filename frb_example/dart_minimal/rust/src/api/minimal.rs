use flutter_rust_bridge::frb;
use std::any::Any;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn f(a: Box<dyn Any + Send>) {
    let _ = a;
}
