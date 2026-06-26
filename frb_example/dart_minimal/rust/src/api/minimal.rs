use flutter_rust_bridge::frb;
use std::any::Any;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn describe_any(a: Box<dyn Any + Send + Sync + 'static>) -> String {
    if a.is::<String>() {
        "string".to_owned()
    } else {
        "unknown".to_owned()
    }
}
