// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::auxiliary::sample_types::MySize;
use log::info;

#[allow(clippy::unused_unit)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn func_return_unit_twin_rust_async_sse() -> () {}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_list_of_struct_twin_rust_async_sse(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_string_list_twin_rust_async_sse(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

#[derive(Debug, Clone)]
pub struct EmptyTwinRustAsyncSse {}

#[flutter_rust_bridge::frb(serialize)]
pub async fn empty_struct_twin_rust_async_sse(
    empty: EmptyTwinRustAsyncSse,
) -> EmptyTwinRustAsyncSse {
    empty
}
