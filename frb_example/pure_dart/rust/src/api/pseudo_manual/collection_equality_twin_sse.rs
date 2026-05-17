// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `collection_equality.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use flutter_rust_bridge::frb;
use std::collections::{HashMap, HashSet};

#[frb(dart_collection_deep_equality)]
pub struct StructWithDeepCollectionEqualityTwinSse {
    pub list: Vec<String>,
    pub map: HashMap<String, String>,
    pub set_values: HashSet<String>,
    pub optional_list: Option<Vec<String>>,
    pub bytes: Vec<u8>,
    pub fixed_bytes: [u8; 3],
}

pub struct StructWithShallowCollectionEqualityTwinSse {
    pub list: Vec<String>,
    pub map: HashMap<String, String>,
    pub set_values: HashSet<String>,
    pub optional_list: Option<Vec<String>>,
    pub bytes: Vec<u8>,
    pub fixed_bytes: [u8; 3],
}

#[flutter_rust_bridge::frb(serialize)]
pub fn echo_struct_with_deep_collection_equality_twin_sse(
    value: StructWithDeepCollectionEqualityTwinSse,
) -> StructWithDeepCollectionEqualityTwinSse {
    value
}

#[flutter_rust_bridge::frb(serialize)]
pub fn echo_struct_with_shallow_collection_equality_twin_sse(
    value: StructWithShallowCollectionEqualityTwinSse,
) -> StructWithShallowCollectionEqualityTwinSse {
    value
}
