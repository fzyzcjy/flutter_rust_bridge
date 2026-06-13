use std::collections::{HashMap, HashSet};

pub struct Simple {
    val: u32,
    values: Vec<String>,
}

#[flutter_rust_bridge_macros::frb(dart_collection_deep_equality)]
pub struct DeepCollectionStruct {
    values: Vec<String>,
    map: HashMap<String, String>,
    set: HashSet<String>,
    optional_values: Option<Vec<String>>,
    bytes: Vec<u8>,
    fixed_bytes: [u8; 3],
}

pub struct ShallowCollectionStruct {
    bytes: Vec<u8>,
    fixed_bytes: [u8; 3],
}

pub enum EnumWithEmptyNamedVariant {
    Empty {},
}
