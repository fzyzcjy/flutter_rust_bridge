// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `map_and_set.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::enumeration_twin_rust_async_sse::{
    EnumSimpleTwinRustAsyncSse, KitchenSinkTwinRustAsyncSse,
};
use crate::auxiliary::sample_types::MySize;
use std::collections::{HashMap, HashSet};

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_map_i32_i32_twin_rust_async_sse(
    arg: HashMap<i32, i32>,
) -> HashMap<i32, i32> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_set_i32_twin_rust_async_sse(arg: HashSet<i32>) -> HashSet<i32> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_map_string_string_twin_rust_async_sse(
    arg: HashMap<String, String>,
) -> HashMap<String, String> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_set_string_twin_rust_async_sse(arg: HashSet<String>) -> HashSet<String> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_map_string_bytes_twin_rust_async_sse(
    arg: HashMap<String, Vec<u8>>,
) -> HashMap<String, Vec<u8>> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_map_string_struct_twin_rust_async_sse(
    arg: HashMap<String, MySize>,
) -> HashMap<String, MySize> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_map_string_simple_enum_twin_rust_async_sse(
    arg: HashMap<String, EnumSimpleTwinRustAsyncSse>,
) -> HashMap<String, EnumSimpleTwinRustAsyncSse> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_hash_map_string_complex_enum_twin_rust_async_sse(
    arg: HashMap<String, KitchenSinkTwinRustAsyncSse>,
) -> HashMap<String, KitchenSinkTwinRustAsyncSse> {
    arg
}
