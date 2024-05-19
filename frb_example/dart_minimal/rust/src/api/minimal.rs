use flutter_rust_bridge::{frb, DartFnFuture};

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn rust_call_dart_return_result_twin_normal(
    callback: impl Fn(String) -> DartFnFuture<anyhow::Result<String>>,
    expect_output: Option<String>,
) {
    assert_eq!(callback("hi".to_owned()).await.ok(), expect_output);
}
