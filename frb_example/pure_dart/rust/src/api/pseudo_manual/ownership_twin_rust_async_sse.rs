// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#[flutter_rust_bridge::frb(serialize)]
pub async fn borrow_string_twin_rust_async_sse(arg: &String) -> String {
    arg.to_owned()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn borrow_str_twin_rust_async_sse(arg: &str) -> String {
    arg.to_owned()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn borrow_i32_twin_rust_async_sse(arg: &i32) -> i32 {
    *arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn borrow_slice_u8_twin_rust_async_sse(arg: &[u8]) -> Vec<u8> {
    arg.to_owned()
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn borrow_slice_string_twin_rust_async_sse(arg: &[String]) -> Vec<String> {
    arg.to_owned()
}

#[derive(Clone)]
pub struct SimpleStructForBorrowTwinRustAsyncSse {
    pub one: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn borrow_struct_twin_rust_async_sse(
    arg: &SimpleStructForBorrowTwinRustAsyncSse,
) -> SimpleStructForBorrowTwinRustAsyncSse {
    arg.clone()
}
