// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `collection_equality.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;
use std::collections::{HashMap, HashSet};

#[frb(dart_collection_deep_equality)]
pub struct StructWithDeepCollectionEqualityTwinRustAsync {
    pub list: Vec<String>,
    pub map: HashMap<String, String>,
    pub set_values: HashSet<String>,
    pub optional_list: Option<Vec<String>>,
    pub bytes: Vec<u8>,
    pub fixed_bytes: [u8; 3],
}

pub struct StructWithShallowCollectionEqualityTwinRustAsync {
    pub list: Vec<String>,
    pub map: HashMap<String, String>,
    pub set_values: HashSet<String>,
    pub optional_list: Option<Vec<String>>,
    pub bytes: Vec<u8>,
    pub fixed_bytes: [u8; 3],
}

pub async fn echo_struct_with_deep_collection_equality_twin_rust_async(
    value: StructWithDeepCollectionEqualityTwinRustAsync,
) -> StructWithDeepCollectionEqualityTwinRustAsync {
    value
}

pub async fn echo_struct_with_shallow_collection_equality_twin_rust_async(
    value: StructWithShallowCollectionEqualityTwinRustAsync,
) -> StructWithShallowCollectionEqualityTwinRustAsync {
    value
}
