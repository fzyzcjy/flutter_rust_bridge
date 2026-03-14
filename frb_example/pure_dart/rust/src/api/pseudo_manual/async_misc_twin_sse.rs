// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `async_misc.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_async_void_twin_sse() {}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_async_simple_add_twin_sse(a: i32, b: i32) -> i32 {
    a + b
}
