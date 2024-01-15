// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_type.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::sample_types::MySize;
use log::info;

#[allow(clippy::unused_unit)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_return_unit_twin_sync() -> () {}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_list_of_struct_twin_sync(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_string_list_twin_sync(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

#[derive(Debug, Clone)]
pub struct EmptyTwinSync {}

#[flutter_rust_bridge::frb(sync)]
pub fn empty_struct_twin_sync(empty: EmptyTwinSync) -> EmptyTwinSync {
    empty
}
