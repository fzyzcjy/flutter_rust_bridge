use crate::auxiliary::sample_types::MySize;
use flutter_rust_bridge::frb;
use log::info;

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> MyTreeNode {
    // info!("handle_complex_struct({:?})", &s);
    let _s_cloned = s.clone();
    s
}

#[derive(Debug, Clone, Copy)]
pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

//This seems to be a bug in the syn parser (v1), for whoever tries to fix it, after each failed build you need to manually remove all rust generated files (bridge_*)
// pub fn test_raw_string_item_struct_with_raw_string_in_func(r#type: String) -> RawStringItemStruct {
//     RawStringItemStruct { r#type }
// }

pub fn list_of_primitive_enums(weekdays: Vec<Weekdays>) -> Vec<Weekdays> {
    weekdays
}

#[derive(Debug, Clone)]
pub struct MyNestedStruct {
    pub tree_node: MyTreeNode,
    pub weekday: Weekdays,
}

pub fn handle_nested_struct(s: MyNestedStruct) -> MyNestedStruct {
    println!("handle_nested_struct({s:?})");
    let _s_cloned = s.clone();
    s
}

pub struct BigBuffers {
    pub int64: Vec<i64>,
    pub uint64: Vec<u64>,
}

pub fn handle_big_buffers() -> BigBuffers {
    BigBuffers {
        int64: vec![i64::MIN, i64::MAX],
        uint64: vec![u64::MAX],
    }
}

pub struct A {
    pub a: String,
}

pub struct B {
    pub b: i32,
}

pub struct C {
    pub c: bool,
}

pub enum Abc {
    A(A),
    B(B),
    C(C),
    JustInt(i32),
}

pub fn test_abc_enum(abc: Abc) -> Abc {
    abc
}

pub struct StructWithEnum {
    pub abc1: Abc,
    pub abc2: Abc,
}

pub fn test_struct_with_enum(se: StructWithEnum) -> StructWithEnum {
    StructWithEnum {
        abc1: se.abc2,
        abc2: se.abc1,
    }
}

pub fn handle_string(s: String) -> String {
    info!("handle_string({})", &s);
    let s2 = s.clone();
    s + &s2
}

// to check that `Vec<u8>` can be used as return type
pub fn handle_vec_u8(v: Vec<u8>) -> Vec<u8> {
    info!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    v.repeat(2)
}

pub fn handle_struct(arg: MySize, boxed: Box<MySize>) -> MySize {
    info!("handle_struct({:?}, {:?})", &arg, &boxed);
    MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Debug, Clone)]
pub struct MySizeFreezed {
    pub width: i32,
    pub height: i32,
}

#[frb(sync)]
pub fn handle_struct_sync_freezed(arg: MySizeFreezed, boxed: Box<MySizeFreezed>) -> MySizeFreezed {
    info!("handle_struct_sync_freezed({:?}, {:?})", &arg, &boxed);
    MySizeFreezed {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}
