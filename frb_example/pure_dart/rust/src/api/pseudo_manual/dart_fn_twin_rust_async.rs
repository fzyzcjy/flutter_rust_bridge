// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_fn.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};

pub struct DemoStructForRustCallDartTwinRustAsync {
    pub name: String,
}

pub async fn rust_call_dart_simple_twin_rust_async(callback: impl Fn() -> DartFnFuture<()>) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

pub async fn rust_call_dart_one_arg_twin_rust_async(callback: impl Fn(String) -> DartFnFuture<()>) {
    callback("a".to_owned()).await;
}

pub async fn rust_call_dart_two_args_twin_rust_async(
    callback: impl Fn(String, DemoStructForRustCallDartTwinRustAsync) -> DartFnFuture<()>,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDartTwinRustAsync {
            name: "b".to_owned(),
        },
    )
    .await;
}

pub async fn rust_call_dart_return_twin_rust_async(callback: impl Fn() -> DartFnFuture<String>) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

pub async fn rust_call_dart_loopback_twin_rust_async(
    callback: impl Fn(
        DemoStructForRustCallDartTwinRustAsync,
    ) -> DartFnFuture<DemoStructForRustCallDartTwinRustAsync>,
) {
    let result = callback(DemoStructForRustCallDartTwinRustAsync { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}

pub async fn rust_call_dart_with_dart_opaque_arg_twin_rust_async(
    input: DartOpaque,
    callback: impl Fn(DartOpaque) -> DartFnFuture<()>,
) {
    callback(input).await
}

pub async fn rust_call_dart_with_dart_opaque_result_twin_rust_async(
    callback: impl Fn() -> DartFnFuture<DartOpaque>,
) -> DartOpaque {
    callback().await
}

pub async fn rust_call_dart_multi_times_twin_rust_async(
    callback: impl Fn() -> DartFnFuture<()>,
    num_times: i32,
) {
    for _ in 0..num_times {
        callback().await;
    }
}

pub async fn rust_call_dart_return_result_twin_rust_async(
    callback: impl Fn(String) -> DartFnFuture<anyhow::Result<String>>,
    expect_output: Option<String>,
) {
    assert_eq!(callback("hi".to_owned()).await.ok(), expect_output);
}
