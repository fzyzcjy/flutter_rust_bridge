// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `misc_example.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::auxiliary::sample_types::MySize;
use flutter_rust_bridge::frb;
use log::info;

#[derive(Debug, Clone)]
pub struct MyTreeNodeTwinSse {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNodeTwinSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_complex_struct_twin_sse(s: MyTreeNodeTwinSse) -> MyTreeNodeTwinSse {
    // info!("handle_complex_struct({:?})", &s);
    let _s_cloned = s.clone();
    s
}

#[derive(Debug, Clone, Copy)]
pub enum WeekdaysTwinSse {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

//This seems to be a bug in the syn parser (v1), for whoever tries to fix it, after each failed build you need to manually remove all rust generated files (bridge_*)
// #[flutter_rust_bridge::frb(serialize)] pub fn test_raw_string_item_struct_with_raw_string_in_func_twin_sse(r#type: String) -> RawStringItemStruct {
//     RawStringItemStruct { r#type }
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn list_of_primitive_enums_twin_sse(weekdays: Vec<WeekdaysTwinSse>) -> Vec<WeekdaysTwinSse> {
    weekdays
}

#[derive(Debug, Clone)]
pub struct MyNestedStructTwinSse {
    pub tree_node: MyTreeNodeTwinSse,
    pub weekday: WeekdaysTwinSse,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_nested_struct_twin_sse(s: MyNestedStructTwinSse) -> MyNestedStructTwinSse {
    println!("handle_nested_struct({s:?})");
    let _s_cloned = s.clone();
    s
}

pub struct BigBuffersTwinSse {
    pub int64: Vec<i64>,
    pub uint64: Vec<u64>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_big_buffers_twin_sse() -> BigBuffersTwinSse {
    BigBuffersTwinSse {
        int64: vec![i64::MIN, i64::MAX],
        uint64: vec![u64::MAX],
    }
}

pub struct ATwinSse {
    pub a: String,
}

pub struct BTwinSse {
    pub b: i32,
}

pub struct CTwinSse {
    pub c: bool,
}

pub enum AbcTwinSse {
    A(ATwinSse),
    B(BTwinSse),
    C(CTwinSse),
    JustInt(i32),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn test_abc_enum_twin_sse(abc: AbcTwinSse) -> AbcTwinSse {
    abc
}

pub struct StructWithEnumTwinSse {
    pub abc1: AbcTwinSse,
    pub abc2: AbcTwinSse,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn test_struct_with_enum_twin_sse(se: StructWithEnumTwinSse) -> StructWithEnumTwinSse {
    StructWithEnumTwinSse {
        abc1: se.abc2,
        abc2: se.abc1,
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_string_twin_sse(s: String) -> String {
    info!("handle_string({})", &s);
    let s2 = s.clone();
    s + &s2
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_char_twin_sse(arg: char) -> char {
    arg
}

// to check that `Vec<u8>` can be used as return type
#[flutter_rust_bridge::frb(serialize)]
pub fn handle_vec_u8_twin_sse(v: Vec<u8>) -> Vec<u8> {
    info!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    v.repeat(2)
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_struct_twin_sse(arg: MySize, boxed: Box<MySize>) -> MySize {
    info!("handle_struct({:?}, {:?})", &arg, &boxed);
    MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Debug, Clone)]
pub struct MySizeFreezedTwinSse {
    pub width: i32,
    pub height: i32,
}

// TODO move it to a non-auto-generated test
// #[frb(sync)]
// #[flutter_rust_bridge::frb(serialize)] pub fn handle_struct_sync_freezed_twin_sse(
//     arg: MySizeFreezedTwinSse,
//     boxed: Box<MySizeFreezedTwinSse>,
// ) -> MySizeFreezedTwinSse {
//     info!("handle_struct_sync_freezed({:?}, {:?})", &arg, &boxed);
//     MySizeFreezedTwinSse {
//         width: arg.width + boxed.width,
//         height: arg.height + boxed.height,
//     }
// }

// To test parsing of `pub(super)`
#[allow(dead_code)]
pub(super) fn visibility_restricted_func_twin_sse() {}

#[frb(positional)]
#[flutter_rust_bridge::frb(serialize)]
pub fn positional_arguments_twin_sse(a: i32, b: i32) -> i32 {
    a + b
}
