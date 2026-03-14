// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_example.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::auxiliary::sample_types::MySize;
use flutter_rust_bridge::frb;
use log::info;

#[derive(Debug, Clone)]
pub struct MyTreeNodeTwinRustAsync {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNodeTwinRustAsync>,
}

pub async fn handle_complex_struct_twin_rust_async(
    s: MyTreeNodeTwinRustAsync,
) -> MyTreeNodeTwinRustAsync {
    // info!("handle_complex_struct({:?})", &s);
    let _s_cloned = s.clone();
    s
}

#[derive(Debug, Clone, Copy)]
pub enum WeekdaysTwinRustAsync {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

//This seems to be a bug in the syn parser (v1), for whoever tries to fix it, after each failed build you need to manually remove all rust generated files (bridge_*)
// pub async fn test_raw_string_item_struct_with_raw_string_in_func_twin_rust_async(r#type: String) -> RawStringItemStruct {
//     RawStringItemStruct { r#type }
// }

pub async fn list_of_primitive_enums_twin_rust_async(
    weekdays: Vec<WeekdaysTwinRustAsync>,
) -> Vec<WeekdaysTwinRustAsync> {
    weekdays
}

#[derive(Debug, Clone)]
pub struct MyNestedStructTwinRustAsync {
    pub tree_node: MyTreeNodeTwinRustAsync,
    pub weekday: WeekdaysTwinRustAsync,
}

pub async fn handle_nested_struct_twin_rust_async(
    s: MyNestedStructTwinRustAsync,
) -> MyNestedStructTwinRustAsync {
    println!("handle_nested_struct({s:?})");
    let _s_cloned = s.clone();
    s
}

pub struct BigBuffersTwinRustAsync {
    pub int64: Vec<i64>,
    pub uint64: Vec<u64>,
}

pub async fn handle_big_buffers_twin_rust_async() -> BigBuffersTwinRustAsync {
    BigBuffersTwinRustAsync {
        int64: vec![i64::MIN, i64::MAX],
        uint64: vec![u64::MAX],
    }
}

pub struct ATwinRustAsync {
    pub a: String,
}

pub struct BTwinRustAsync {
    pub b: i32,
}

pub struct CTwinRustAsync {
    pub c: bool,
}

pub enum AbcTwinRustAsync {
    A(ATwinRustAsync),
    B(BTwinRustAsync),
    C(CTwinRustAsync),
    JustInt(i32),
}

pub async fn test_abc_enum_twin_rust_async(abc: AbcTwinRustAsync) -> AbcTwinRustAsync {
    abc
}

pub struct StructWithEnumTwinRustAsync {
    pub abc1: AbcTwinRustAsync,
    pub abc2: AbcTwinRustAsync,
}

pub async fn test_struct_with_enum_twin_rust_async(
    se: StructWithEnumTwinRustAsync,
) -> StructWithEnumTwinRustAsync {
    StructWithEnumTwinRustAsync {
        abc1: se.abc2,
        abc2: se.abc1,
    }
}

pub async fn handle_string_twin_rust_async(s: String) -> String {
    info!("handle_string({})", &s);
    let s2 = s.clone();
    s + &s2
}

pub async fn handle_char_twin_rust_async(arg: char) -> char {
    arg
}

// to check that `Vec<u8>` can be used as return type
pub async fn handle_vec_u8_twin_rust_async(v: Vec<u8>) -> Vec<u8> {
    info!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    v.repeat(2)
}

pub async fn handle_struct_twin_rust_async(arg: MySize, boxed: Box<MySize>) -> MySize {
    info!("handle_struct({:?}, {:?})", &arg, &boxed);
    MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Debug, Clone)]
pub struct MySizeFreezedTwinRustAsync {
    pub width: i32,
    pub height: i32,
}

// TODO move it to a non-auto-generated test
// #[frb(sync)]
// pub async fn handle_struct_sync_freezed_twin_rust_async(
//     arg: MySizeFreezedTwinRustAsync,
//     boxed: Box<MySizeFreezedTwinRustAsync>,
// ) -> MySizeFreezedTwinRustAsync {
//     info!("handle_struct_sync_freezed({:?}, {:?})", &arg, &boxed);
//     MySizeFreezedTwinRustAsync {
//         width: arg.width + boxed.width,
//         height: arg.height + boxed.height,
//     }
// }

// To test parsing of `pub(super)`
#[allow(dead_code)]
pub(super) fn visibility_restricted_func_twin_rust_async() {}

#[frb(positional)]
pub async fn positional_arguments_twin_rust_async(a: i32, b: i32) -> i32 {
    a + b
}
