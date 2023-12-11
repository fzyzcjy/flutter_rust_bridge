// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_fn.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "syncSse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;

pub struct DemoStructForRustCallDartTwinSse {
    pub name: String,
}

pub async fn rust_call_dart_simple_twin_sse(callback: impl Fn() -> DartFnFuture<()> + UnwindSafe) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

pub async fn rust_call_dart_one_arg_twin_sse(
    callback: impl Fn(String) -> DartFnFuture<()> + UnwindSafe,
) {
    callback("a".to_owned()).await;
}

pub async fn rust_call_dart_two_args_twin_sse(
    callback: impl Fn(String, DemoStructForRustCallDartTwinSse) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDartTwinSse {
            name: "b".to_owned(),
        },
    )
    .await;
}

pub async fn rust_call_dart_return_twin_sse(
    callback: impl Fn() -> DartFnFuture<String> + UnwindSafe,
) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

pub async fn rust_call_dart_loopback_twin_sse(
    callback: impl Fn(DemoStructForRustCallDartTwinSse) -> DartFnFuture<DemoStructForRustCallDartTwinSse>
        + UnwindSafe,
) {
    let result = callback(DemoStructForRustCallDartTwinSse { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}

pub async fn rust_call_dart_with_dart_opaque_arg_twin_sse(
    input: DartOpaque,
    callback: impl Fn(DartOpaque) -> DartFnFuture<()> + UnwindSafe,
) {
    callback(input).await
}

pub async fn rust_call_dart_with_dart_opaque_result_twin_sse(
    callback: impl Fn() -> DartFnFuture<DartOpaque> + UnwindSafe,
) -> DartOpaque {
    callback().await
}

pub async fn rust_call_dart_multi_times_twin_sse(
    callback: impl Fn() -> DartFnFuture<()> + UnwindSafe,
    num_times: i32,
) {
    for _ in 0..num_times {
        callback().await;
    }
}
