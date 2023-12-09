// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `async_misc.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

pub async fn func_async_void() {}

pub async fn func_async_simple_add(a: i32, b: i32) -> i32 {
    a + b
}
