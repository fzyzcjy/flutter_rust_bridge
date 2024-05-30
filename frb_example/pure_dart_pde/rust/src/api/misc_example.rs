// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::auxiliary::sample_types::MySize;
use flutter_rust_bridge::frb;
use log::info;

#[derive(Debug, Clone)]
pub struct MyTreeNodeTwinNormal {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNodeTwinNormal>,
}

pub fn handle_complex_struct_twin_normal(s: MyTreeNodeTwinNormal) -> MyTreeNodeTwinNormal {
    // info!("handle_complex_struct({:?})", &s);
    let _s_cloned = s.clone();
    s
}

#[derive(Debug, Clone, Copy)]
pub enum WeekdaysTwinNormal {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

//This seems to be a bug in the syn parser (v1), for whoever tries to fix it, after each failed build you need to manually remove all rust generated files (bridge_*)
// pub fn test_raw_string_item_struct_with_raw_string_in_func_twin_normal(r#type: String) -> RawStringItemStruct {
//     RawStringItemStruct { r#type }
// }

pub fn list_of_primitive_enums_twin_normal(
    weekdays: Vec<WeekdaysTwinNormal>,
) -> Vec<WeekdaysTwinNormal> {
    weekdays
}

#[derive(Debug, Clone)]
pub struct MyNestedStructTwinNormal {
    pub tree_node: MyTreeNodeTwinNormal,
    pub weekday: WeekdaysTwinNormal,
}

pub fn handle_nested_struct_twin_normal(s: MyNestedStructTwinNormal) -> MyNestedStructTwinNormal {
    println!("handle_nested_struct({s:?})");
    let _s_cloned = s.clone();
    s
}

pub struct BigBuffersTwinNormal {
    pub int64: Vec<i64>,
    pub uint64: Vec<u64>,
}

pub fn handle_big_buffers_twin_normal() -> BigBuffersTwinNormal {
    BigBuffersTwinNormal {
        int64: vec![i64::MIN, i64::MAX],
        uint64: vec![u64::MAX],
    }
}

pub struct ATwinNormal {
    pub a: String,
}

pub struct BTwinNormal {
    pub b: i32,
}

pub struct CTwinNormal {
    pub c: bool,
}

pub enum AbcTwinNormal {
    A(ATwinNormal),
    B(BTwinNormal),
    C(CTwinNormal),
    JustInt(i32),
}

pub fn test_abc_enum_twin_normal(abc: AbcTwinNormal) -> AbcTwinNormal {
    abc
}

pub struct StructWithEnumTwinNormal {
    pub abc1: AbcTwinNormal,
    pub abc2: AbcTwinNormal,
}

pub fn test_struct_with_enum_twin_normal(se: StructWithEnumTwinNormal) -> StructWithEnumTwinNormal {
    StructWithEnumTwinNormal {
        abc1: se.abc2,
        abc2: se.abc1,
    }
}

pub fn handle_string_twin_normal(s: String) -> String {
    info!("handle_string({})", &s);
    let s2 = s.clone();
    s + &s2
}

pub fn handle_char_twin_normal(arg: char) -> char {
    arg
}

// to check that `Vec<u8>` can be used as return type
pub fn handle_vec_u8_twin_normal(v: Vec<u8>) -> Vec<u8> {
    info!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    v.repeat(2)
}

pub fn handle_struct_twin_normal(arg: MySize, boxed: Box<MySize>) -> MySize {
    info!("handle_struct({:?}, {:?})", &arg, &boxed);
    MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Debug, Clone)]
pub struct MySizeFreezedTwinNormal {
    pub width: i32,
    pub height: i32,
}

// TODO move it to a non-auto-generated test
// #[frb(sync)]
// pub fn handle_struct_sync_freezed_twin_normal(
//     arg: MySizeFreezedTwinNormal,
//     boxed: Box<MySizeFreezedTwinNormal>,
// ) -> MySizeFreezedTwinNormal {
//     info!("handle_struct_sync_freezed({:?}, {:?})", &arg, &boxed);
//     MySizeFreezedTwinNormal {
//         width: arg.width + boxed.width,
//         height: arg.height + boxed.height,
//     }
// }

// To test parsing of `pub(super)`
#[allow(dead_code)]
pub(super) fn visibility_restricted_func_twin_normal() {}

#[frb(positional)]
pub fn positional_arguments_twin_normal(a: i32, b: i32) -> i32 {
    a + b
}
