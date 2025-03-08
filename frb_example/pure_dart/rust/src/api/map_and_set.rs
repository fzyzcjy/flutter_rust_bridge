use crate::api::enumeration::{EnumSimpleTwinNormal, KitchenSinkTwinNormal};
use crate::auxiliary::sample_types::MySize;
use std::collections::{HashMap, HashSet};

/// flutter_rust_bridge:ignore
#[derive(Clone, Debug, Default)]
pub struct CustomHasherTwinNormal(std::collections::hash_map::RandomState);

impl std::hash::BuildHasher for CustomHasherTwinNormal {
    type Hasher = std::collections::hash_map::DefaultHasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }

    fn hash_one<T>(&self, x: T) -> u64
    where
        T: std::hash::Hash,
        Self: Sized,
        Self::Hasher: std::hash::Hasher,
    {
        self.0.hash_one(x)
    }
}

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

pub fn func_hash_map_string_string_hasher_twin_normal(
    arg: HashMap<String, String, CustomHasherTwinNormal>,
) -> HashMap<String, String, CustomHasherTwinNormal> {
    arg
}

pub fn func_hash_set_string_twin_normal(arg: HashSet<String>) -> HashSet<String> {
    arg
}

pub fn func_hash_set_string_hasher_twin_normal(
    arg: HashSet<String, CustomHasherTwinNormal>,
) -> HashSet<String, CustomHasherTwinNormal> {
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
