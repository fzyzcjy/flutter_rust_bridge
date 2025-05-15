// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `map_and_set.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::enumeration_twin_sync_sse::{
    EnumSimpleTwinSyncSse, KitchenSinkTwinSyncSse,
};
use crate::auxiliary::sample_types::MySize;
use std::collections::{HashMap, HashSet};

/// flutter_rust_bridge:ignore
#[derive(Clone, Debug, Default)]
pub struct CustomHasherTwinSyncSse(std::collections::hash_map::RandomState);

impl std::hash::BuildHasher for CustomHasherTwinSyncSse {
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

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_i32_i32_twin_sync_sse(arg: HashMap<i32, i32>) -> HashMap<i32, i32> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_set_i32_twin_sync_sse(arg: HashSet<i32>) -> HashSet<i32> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_string_string_twin_sync_sse(
    arg: HashMap<String, String>,
) -> HashMap<String, String> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_string_string_hasher_twin_sync_sse(
    arg: HashMap<String, String, CustomHasherTwinSyncSse>,
) -> HashMap<String, String, CustomHasherTwinSyncSse> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_set_string_twin_sync_sse(arg: HashSet<String>) -> HashSet<String> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_set_string_hasher_twin_sync_sse(
    arg: HashSet<String, CustomHasherTwinSyncSse>,
) -> HashSet<String, CustomHasherTwinSyncSse> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_string_bytes_twin_sync_sse(
    arg: HashMap<String, Vec<u8>>,
) -> HashMap<String, Vec<u8>> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_string_struct_twin_sync_sse(
    arg: HashMap<String, MySize>,
) -> HashMap<String, MySize> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_string_simple_enum_twin_sync_sse(
    arg: HashMap<String, EnumSimpleTwinSyncSse>,
) -> HashMap<String, EnumSimpleTwinSyncSse> {
    arg
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_hash_map_string_complex_enum_twin_sync_sse(
    arg: HashMap<String, KitchenSinkTwinSyncSse>,
) -> HashMap<String, KitchenSinkTwinSyncSse> {
    arg
}
