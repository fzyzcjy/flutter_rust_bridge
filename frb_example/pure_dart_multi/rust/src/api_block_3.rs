use flutter_rust_bridge::SyncReturn;

use crate::block_specific_module::StructOnlyForBlock3;

use crate::shared_type_module::{
    CrossSharedStructInBlock2And3, SharedStructInAllBlocks, SharedStructInBlock2And3,
    SharedStructOnlyForSyncTest, SharedWeekdaysEnumInAllBlocks,
};

#[derive(PartialEq, Debug)]
pub enum EnumDefinedInBlock3 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl EnumDefinedInBlock3 {
    pub fn test_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}

#[derive(Debug)]
pub struct StructDefinedInBlock3 {
    pub name: String,
}
impl StructDefinedInBlock3 {
    pub fn test_method(&self, message: String) -> String {
        message
    }

    pub fn test_static_method(message: String) -> String {
        message
    }
}

pub fn test_inbuilt_type_in_block_3(a: i32, b: f32) -> f32 {
    a as f32 + b
}

pub fn test_string_in_block_3(s: String, i: u64) -> String {
    format!("{}_{}", s, i)
}

pub fn test_shared_struct_only_for_sync_with_no_sync_return_in_block_3(
    name: String,
    score: f64,
) -> SharedStructOnlyForSyncTest {
    SharedStructOnlyForSyncTest { name, score }
}

pub fn test_shared_struct_only_for_sync_as_input_with_no_sync_return_in_block_3(
    obj: SharedStructOnlyForSyncTest,
    default_score: f64,
) -> SharedStructOnlyForSyncTest {
    SharedStructOnlyForSyncTest {
        score: default_score,
        ..obj
    }
}

pub fn test_all_shared_struct_in_block_3(
    custom: Option<SharedStructInAllBlocks>,
    s: String,
    i: i32,
) -> Option<SharedStructInAllBlocks> {
    if let Some(mut obj) = custom {
        obj.name = s;
        obj.id = i;
        Some(obj)
    } else {
        None
    }
}

pub fn test_shared_struct_in_block_3_for_2_and_3(
    mut custom: SharedStructInBlock2And3,
    s: String,
    i: i32,
) -> SharedStructInBlock2And3 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_cross_shared_struct_in_block_3_for_2_and_3(
    name: String,
) -> CrossSharedStructInBlock2And3 {
    CrossSharedStructInBlock2And3 { name }
}

pub fn test_cross_shared_struct_in_sync_in_block_3_for_2_and_3(
    name: String,
) -> SyncReturn<CrossSharedStructInBlock2And3> {
    SyncReturn(CrossSharedStructInBlock2And3 { name })
}

pub fn test_unique_struct_3(
    mut custom: StructOnlyForBlock3,
    s: String,
    i: i64,
) -> StructOnlyForBlock3 {
    custom.name = s;
    custom.id = i;
    custom
}

pub fn test_struct_defined_in_block_3(custom: StructDefinedInBlock3) -> String {
    custom.name
}

pub fn test_enum_defined_in_block_3(custom: EnumDefinedInBlock3) -> String {
    match custom {
        EnumDefinedInBlock3::Quit => "quit".to_string(),
        EnumDefinedInBlock3::Move { x, y } => format!("move_{}_{}", x, y),
        EnumDefinedInBlock3::Write(content) => format!("write_{}", content),
        EnumDefinedInBlock3::ChangeColor(r, g, b) => {
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
/// * `struct_list` - A vector of `StructDefinedInBlock3` items -- the item is NOT a shared type.
/// * `enum_list` - A vector of `EnumDefinedInBlock3` items -- the item is NOT a shared type.
///
/// # Returns
///
/// A string that concatenates the names of each item in the input vectors, separated by underscores.
pub fn test_list_in_block_3(
    shared_structs: Vec<SharedStructInAllBlocks>,
    strings: Vec<String>,
    nums: Vec<i32>,
    weekdays: Vec<SharedWeekdaysEnumInAllBlocks>,
    struct_list: Vec<StructDefinedInBlock3>,
    enum_list: Vec<EnumDefinedInBlock3>,
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
