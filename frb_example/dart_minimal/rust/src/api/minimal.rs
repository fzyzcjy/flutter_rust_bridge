use flutter_rust_bridge::{frb, DartFnFuture};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn minimal_optional_callback(callback: Option<impl Fn(String) -> DartFnFuture<()>>) {
    if let Some(callback) = callback {
        callback("optional".to_owned()).await;
    }
}
