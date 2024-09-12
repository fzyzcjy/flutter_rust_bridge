use flutter_rust_bridge::{DartFnFuture, frb};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn hi_callback(arg: impl Fn(String, i32) -> DartFnFuture<String>) -> String {
    arg("hi".to_owned(), 42).await
}
