// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

pub async fn borrow_string_twin_rust_async(arg: &String) -> String {
    arg.to_owned()
}

pub async fn borrow_str_twin_rust_async(arg: &str) -> String {
    arg.to_owned()
}

pub async fn borrow_i32_twin_rust_async(arg: &i32) -> i32 {
    *arg
}

#[derive(Clone)]
pub struct SimpleStructForBorrowTwinRustAsync {
    pub one: String,
}

pub async fn borrow_struct_twin_rust_async(
    arg: &SimpleStructForBorrowTwinRustAsync,
) -> SimpleStructForBorrowTwinRustAsync {
    arg.clone()
}
