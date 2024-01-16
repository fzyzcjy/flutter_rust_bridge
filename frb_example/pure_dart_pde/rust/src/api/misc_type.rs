// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::sample_types::MySize;
use log::info;

#[allow(clippy::unused_unit)]
pub fn func_return_unit_twin_normal() -> () {}

pub fn handle_list_of_struct_twin_normal(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

pub fn handle_string_list_twin_normal(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

#[derive(Debug, Clone)]
pub struct EmptyTwinNormal {}

pub fn empty_struct_twin_normal(empty: EmptyTwinNormal) -> EmptyTwinNormal {
    empty
}
