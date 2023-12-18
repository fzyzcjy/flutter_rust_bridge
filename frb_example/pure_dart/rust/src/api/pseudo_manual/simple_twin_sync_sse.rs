// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `simple.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn simple_adder_twin_sync_sse(a: i32, b: i32) -> i32 {
    a + b
}
