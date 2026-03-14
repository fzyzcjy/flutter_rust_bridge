// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

pub async fn borrow_string_twin_rust_async(arg: &String) -> String {
    arg.to_owned()
}

pub async fn borrow_str_twin_rust_async(arg: &str) -> String {
    arg.to_owned()
}

pub async fn borrow_i32_twin_rust_async(arg: &i32) -> i32 {
    *arg
}

pub async fn borrow_slice_u8_twin_rust_async(arg: &[u8]) -> Vec<u8> {
    arg.to_owned()
}

pub async fn borrow_slice_string_twin_rust_async(arg: &[String]) -> Vec<String> {
    arg.to_owned()
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
