// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use flutter_rust_bridge::frb;
use std::collections::{HashMap, HashSet};

#[frb(dart_collection_deep_equality)]
pub struct StructWithDeepCollectionEqualityTwinNormal {
    pub list: Vec<String>,
    pub map: HashMap<String, String>,
    pub set_values: HashSet<String>,
    pub optional_list: Option<Vec<String>>,
    pub bytes: Vec<u8>,
}

pub struct StructWithShallowCollectionEqualityTwinNormal {
    pub list: Vec<String>,
    pub map: HashMap<String, String>,
    pub set_values: HashSet<String>,
    pub optional_list: Option<Vec<String>>,
    pub bytes: Vec<u8>,
}

pub fn echo_struct_with_deep_collection_equality_twin_normal(
    value: StructWithDeepCollectionEqualityTwinNormal,
) -> StructWithDeepCollectionEqualityTwinNormal {
    value
}

pub fn echo_struct_with_shallow_collection_equality_twin_normal(
    value: StructWithShallowCollectionEqualityTwinNormal,
) -> StructWithShallowCollectionEqualityTwinNormal {
    value
}
