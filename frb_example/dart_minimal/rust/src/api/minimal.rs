use flutter_rust_bridge::{frb, DartFnFuture};
use std::sync::Arc;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn subscribe_to_values(dart_callback: impl Fn(String) -> DartFnFuture<String>) {
    let dart_callback = Arc::new(dart_callback);
    tokio::task::spawn(async move {
        dart_callback("Hello from Rust!".to_owned()).await;
    });
}
