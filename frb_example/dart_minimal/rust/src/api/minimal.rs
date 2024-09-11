use flutter_rust_bridge::{frb, DartFnFuture};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn rust_function(
    dart_callback: impl Fn(String, Vec<u8>, Vec<String>) -> DartFnFuture<String>,
) {
    dart_callback("Tom".to_owned(), vec![], vec![]).await; // Will get `Hello, Tom!`
}
