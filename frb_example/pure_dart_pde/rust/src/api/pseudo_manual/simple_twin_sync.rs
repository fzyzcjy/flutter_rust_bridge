// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `simple.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#[flutter_rust_bridge::frb(sync)]
pub fn simple_adder_twin_sync(a: i32, b: i32) -> i32 {
    a + b
}
