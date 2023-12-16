// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_fn.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "syncSse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use std::panic::UnwindSafe;

pub struct DemoStructForRustCallDartTwinRustAsyncSse {
    pub name: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_simple_twin_rust_async_sse(
    callback: impl Fn() -> DartFnFuture<()> + UnwindSafe,
) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_one_arg_twin_rust_async_sse(
    callback: impl Fn(String) -> DartFnFuture<()> + UnwindSafe,
) {
    callback("a".to_owned()).await;
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_two_args_twin_rust_async_sse(
    callback: impl Fn(String, DemoStructForRustCallDartTwinRustAsyncSse) -> DartFnFuture<()>
        + UnwindSafe,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDartTwinRustAsyncSse {
            name: "b".to_owned(),
        },
    )
    .await;
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_return_twin_rust_async_sse(
    callback: impl Fn() -> DartFnFuture<String> + UnwindSafe,
) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_loopback_twin_rust_async_sse(
    callback: impl Fn(
            DemoStructForRustCallDartTwinRustAsyncSse,
        ) -> DartFnFuture<DemoStructForRustCallDartTwinRustAsyncSse>
        + UnwindSafe,
) {
    let result = callback(DemoStructForRustCallDartTwinRustAsyncSse { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_with_dart_opaque_arg_twin_rust_async_sse(
    input: DartOpaque,
    callback: impl Fn(DartOpaque) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(input).await
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_with_dart_opaque_result_twin_rust_async_sse(
    callback: impl Fn() -> DartFnFuture<DartOpaque> + UnwindSafe,
) -> DartOpaque {
    callback().await
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_multi_times_twin_rust_async_sse(
    callback: impl Fn() -> DartFnFuture<()> + UnwindSafe,
    num_times: i32,
) {
    for _ in 0..num_times {
        callback().await;
    }
}
