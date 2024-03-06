// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::api::enumeration::{EnumSimpleTwinNormal, KitchenSinkTwinNormal};
use crate::auxiliary::sample_types::MySize;
use std::collections::{HashMap, HashSet};

pub fn func_hash_map_i32_i32_twin_normal(arg: HashMap<i32, i32>) -> HashMap<i32, i32> {
    arg
}

pub fn func_hash_set_i32_twin_normal(arg: HashSet<i32>) -> HashSet<i32> {
    arg
}

pub fn func_hash_map_string_string_twin_normal(
    arg: HashMap<String, String>,
) -> HashMap<String, String> {
    arg
}

pub fn func_hash_set_string_twin_normal(arg: HashSet<String>) -> HashSet<String> {
    arg
}

pub fn func_hash_map_string_bytes_twin_normal(
    arg: HashMap<String, Vec<u8>>,
) -> HashMap<String, Vec<u8>> {
    arg
}

pub fn func_hash_map_string_struct_twin_normal(
    arg: HashMap<String, MySize>,
) -> HashMap<String, MySize> {
    arg
}

pub fn func_hash_map_string_simple_enum_twin_normal(
    arg: HashMap<String, EnumSimpleTwinNormal>,
) -> HashMap<String, EnumSimpleTwinNormal> {
    arg
}

pub fn func_hash_map_string_complex_enum_twin_normal(
    arg: HashMap<String, KitchenSinkTwinNormal>,
) -> HashMap<String, KitchenSinkTwinNormal> {
    arg
}
