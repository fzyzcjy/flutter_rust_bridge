// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `benchmark_api.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(unused_variables)]

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_void_twin_rust_async_sse() {}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_input_bytes_twin_rust_async_sse(bytes: Vec<u8>) -> i32 {
    bytes.into_iter().map(|x| x as i32).sum()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn benchmark_output_bytes_twin_rust_async_sse(size: i32) -> Vec<u8> {
    vec![0; size as usize]
}
