use super::dep::{
    DeepCollectionStruct, EnumWithEmptyNamedVariant, ShallowCollectionStruct, Simple,
};

pub fn first_function() {}
pub fn second_function(arg: Simple) {}
pub fn third_function(arg: DeepCollectionStruct) {}
pub fn fourth_function(arg: ShallowCollectionStruct) {}
pub fn fifth_function(arg: EnumWithEmptyNamedVariant) {}
