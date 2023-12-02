use flutter_rust_bridge::SyncReturn;

use crate::block_specific_module::StructOnlyForBlock1;
#[allow(unused)]
use crate::fake_module::{self}; // this statement is used to test special import of the module when frb is generating.
use crate::shared_type_module::SharedWeekdaysEnumInAllBlocks;
use crate::shared_type_module::{
    CrossSharedStructInBlock1And2, SharedStructInAllBlocks, SharedStructInBlock1And2,
    SharedStructOnlyForSyncTest,
};

#[derive(PartialEq, Debug)]
pub enum EnumDefinedInBlock1 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl EnumDefinedInBlock1 {
    pub fn test_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}

#[derive(Debug)]
pub struct StructDefinedInBlock1 {
    pub name: String,
    // TODO: delete?
    // pub sub_type: SubTypeForStructDefinedInBlock1,
}
impl StructDefinedInBlock1 {
    pub fn test_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}

pub fn test_inbuilt_type_in_block_1(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_in_block_1(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_string_in_sync_in_block_1(s: String, i: u64) -> SyncReturn<String> {
    SyncReturn(format!("{}_{}", s, i))
}

pub fn test_optional_string_in_block_1(s: Option<String>, i: i32) -> Option<String> {
    s.map(|s| format!("{}{}", s, i))
}

pub fn test_optional_string_in_sync_in_block_1(
    s: Option<String>,
    i: i32,
) -> SyncReturn<Option<String>> {
    match s {
        Some(s) => SyncReturn(Some(format!("{}{}", s, i))),
        None => SyncReturn(None),
    }
}

pub fn test_shared_struct_only_for_sync_with_sync_return_in_block_1(
    name: String,
    score: f64,
) -> SyncReturn<SharedStructOnlyForSyncTest> {
    SyncReturn(SharedStructOnlyForSyncTest { name, score })
}

pub fn test_all_shared_struct_in_block_1(
    mut custom: SharedStructInAllBlocks,
    s: String,
    i: i32,
) -> SharedStructInAllBlocks {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_shared_struct_in_block_1_for_1_and_2(
    mut custom: SharedStructInBlock1And2,
    s: String,
    i: i32,
) -> SharedStructInBlock1And2 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_in_block_1_for_1_and_2(
    custom: CrossSharedStructInBlock1And2,
) -> String {
    custom.name
}

pub fn test_unique_struct_1(
    mut custom: StructOnlyForBlock1,
    s: String,
    i: i8,
) -> StructOnlyForBlock1 {
    custom.name = if s.is_empty() { None } else { Some(s) };
    custom.id = Some(i);
    custom.num = Some(i as f64);
    custom
}

pub fn test_struct_defined_in_block_1(custom: StructDefinedInBlock1) -> String {
    custom.name
}

pub fn test_enum_defined_in_block_1(custom: EnumDefinedInBlock1) -> String {
    match custom {
        EnumDefinedInBlock1::Quit => "quit".to_string(),
        EnumDefinedInBlock1::Move { x, y } => format!("move_{}_{}", x, y),
        EnumDefinedInBlock1::Write(content) => format!("write_{}", content),
        EnumDefinedInBlock1::ChangeColor(r, g, b) => {
            format!("changeColor_{}_{}_{}", r, g, b)
        }
    }
}

/// This API is for testing generating API with list types.
/// To achieve a complete test, it accepts params of various types of lists
/// -- lists of primitive types, strings, and customized structs/enums, some of the inner items of which are
/// shared, and the others are not shared.
///
/// # Arguments
///
/// * `shared_structs` - A vector of `SharedStructInAllBlocks` items -- the item is a shared type.
/// * `strings` - A vector of `String` items -- the item is a shared type.
/// * `nums` - A vector of `i32` items -- the item is a shared type.
/// * `weekdays` - A vector of `SharedWeekdaysEnumInAllBlocks` items -- the item is a shared type.
/// * `struct_list` - A vector of `StructDefinedInBlock1` items -- the item is NOT a shared type.
/// * `enum_list` - A vector of `EnumDefinedInBlock1` items -- the item is NOT a shared type.
///
/// # Returns
///
/// A string that concatenates the names of each item in the input vectors, separated by underscores.
pub fn test_list_in_block_1(
    shared_structs: Vec<SharedStructInAllBlocks>,
    strings: Vec<String>,
    nums: Vec<i32>,
    weekdays: Vec<SharedWeekdaysEnumInAllBlocks>,
    struct_list: Vec<StructDefinedInBlock1>,
    enum_list: Vec<EnumDefinedInBlock1>,
) -> String {
    //join name of each item in shared_structs
    let s = shared_structs
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<String>>()
        .join("_");
    let mut s = format!("{}_{}", s, strings.join("_"));

    // put each num in `nums` to stirng `s`
    for num in nums {
        s.push_str(&format!("_{}", num));
    }

    // put each weekday in `weekdays` to string `s`
    for weekday in weekdays {
        s.push_str(&format!("_{}", weekday.print_weekday()));
    }

    // join name of each item in struct_list
    let struct_names = struct_list
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<String>>()
        .join("_");
    s = format!("{}_{}", s, struct_names);

    // join name of each item in enum_list
    let enum_names = enum_list
        .iter()
        .map(|e| format!("{:?}", e))
        .collect::<Vec<String>>()
        .join("_");
    s = format!("{}_{}", s, enum_names);

    s
}
