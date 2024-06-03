// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#[flutter_rust_bridge::frb(sync)]
pub fn borrow_string_twin_sync(arg: &String) -> String {
    arg.to_owned()
}

#[flutter_rust_bridge::frb(sync)]
pub fn borrow_str_twin_sync(arg: &str) -> String {
    arg.to_owned()
}

#[flutter_rust_bridge::frb(sync)]
pub fn borrow_i32_twin_sync(arg: &i32) -> i32 {
    *arg
}

#[flutter_rust_bridge::frb(sync)]
pub fn borrow_slice_u8_twin_sync(arg: &[u8]) -> Vec<u8> {
    arg.to_owned()
}

#[flutter_rust_bridge::frb(sync)]
pub fn borrow_slice_string_twin_sync(arg: &[String]) -> Vec<String> {
    arg.to_owned()
}

#[derive(Clone)]
pub struct SimpleStructForBorrowTwinSync {
    pub one: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn borrow_struct_twin_sync(
    arg: &SimpleStructForBorrowTwinSync,
) -> SimpleStructForBorrowTwinSync {
    arg.clone()
}
