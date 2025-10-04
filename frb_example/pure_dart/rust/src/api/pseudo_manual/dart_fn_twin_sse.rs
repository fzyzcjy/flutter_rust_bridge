// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_fn.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use flutter_rust_bridge::{DartFnFuture, DartOpaque};
use std::net::Ipv4Addr;

pub struct DemoStructForRustCallDartTwinSse {
    pub name: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_simple_twin_sse(callback: impl Fn() -> DartFnFuture<()>) {
    println!("rust_call_dart_simple before");
    callback().await;
    println!("rust_call_dart_simple after");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_one_arg_twin_sse(callback: impl Fn(String) -> DartFnFuture<()>) {
    callback("a".to_owned()).await;
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_two_args_twin_sse(
    callback: impl Fn(String, DemoStructForRustCallDartTwinSse) -> DartFnFuture<()>,
) {
    callback(
        "a".to_owned(),
        DemoStructForRustCallDartTwinSse {
            name: "b".to_owned(),
        },
    )
    .await;
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_return_twin_sse(callback: impl Fn() -> DartFnFuture<String>) {
    let result = callback().await;
    assert_eq!(&result, "a");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_loopback_twin_sse(
    callback: impl Fn(
        DemoStructForRustCallDartTwinSse,
    ) -> DartFnFuture<DemoStructForRustCallDartTwinSse>,
) {
    let result = callback(DemoStructForRustCallDartTwinSse { name: "a".into() }).await;
    assert_eq!(&result.name, "a");
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_with_dart_opaque_arg_twin_sse(
    input: DartOpaque,
    callback: impl Fn(DartOpaque) -> DartFnFuture<()>,
) {
    callback(input).await
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_with_dart_opaque_result_twin_sse(
    callback: impl Fn() -> DartFnFuture<DartOpaque>,
) -> DartOpaque {
    callback().await
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_multi_times_twin_sse(
    callback: impl Fn() -> DartFnFuture<()>,
    num_times: i32,
) {
    for _ in 0..num_times {
        callback().await;
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_return_result_twin_sse(
    callback: impl Fn(String) -> DartFnFuture<anyhow::Result<String>>,
    expect_output: Option<String>,
) {
    assert_eq!(callback("hi".to_owned()).await.ok(), expect_output);
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn rust_call_dart_using_ipv4_addr_twin_sse(
    callback: impl Fn(Ipv4Addr) -> DartFnFuture<Ipv4Addr>,
) {
    assert_eq!(
        callback(Ipv4Addr::new(127, 0, 0, 1)).await,
        Ipv4Addr::new(127, 0, 0, 255)
    );
}
