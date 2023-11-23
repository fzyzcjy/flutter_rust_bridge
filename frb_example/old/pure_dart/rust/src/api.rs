#![allow(unused_variables)]

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
pub use std::sync::{Mutex, RwLock};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Result};

use backtrace::Backtrace;
use flutter_rust_bridge::*;
use lazy_static::lazy_static;

use crate::data::{EnumAlias, Id, MyEnum, MyStruct, StructAlias, UserIdAlias};
pub use crate::data::{
    FrbOpaqueReturn, FrbOpaqueSyncReturn, HideData, NonCloneData, NonSendHideData,
};
use crate::new_module_system::{use_new_module_system, NewSimpleStruct};
use crate::old_module_system::{use_old_module_system, OldSimpleStruct};
use log::info;

#[cfg(target_family = "wasm")]
mod helpers;

/// Some initialization code to run when the library is first loaded.
#[cfg(not(target_family = "wasm"))]
#[static_init::constructor]
extern "C" fn on_dylib_start() {
    _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .try_init();
}

// TODO after enabling zero-copy by default, this is not needed anymore
// pub struct ZeroCopyVecOfPrimitivePack {
//     pub int8list: ZeroCopyBuffer<Vec<i8>>,
//     pub uint8list: ZeroCopyBuffer<Vec<u8>>,
//     pub int16list: ZeroCopyBuffer<Vec<i16>>,
//     pub uint16list: ZeroCopyBuffer<Vec<u16>>,
//     pub uint32list: ZeroCopyBuffer<Vec<u32>>,
//     pub int32list: ZeroCopyBuffer<Vec<i32>>,
//     pub uint64list: ZeroCopyBuffer<Vec<u64>>,
//     pub int64list: ZeroCopyBuffer<Vec<i64>>,
//     pub float32list: ZeroCopyBuffer<Vec<f32>>,
//     pub float64list: ZeroCopyBuffer<Vec<f64>>,
// }
//
// pub fn handle_zero_copy_vec_of_primitive(n: i32) -> ZeroCopyVecOfPrimitivePack {
//     ZeroCopyVecOfPrimitivePack {
//         int8list: ZeroCopyBuffer(vec![42i8; n as usize]),
//         uint8list: ZeroCopyBuffer(vec![42u8; n as usize]),
//         int16list: ZeroCopyBuffer(vec![42i16; n as usize]),
//         uint16list: ZeroCopyBuffer(vec![42u16; n as usize]),
//         int32list: ZeroCopyBuffer(vec![42i32; n as usize]),
//         uint32list: ZeroCopyBuffer(vec![42u32; n as usize]),
//         int64list: ZeroCopyBuffer(vec![42i64; n as usize]),
//         uint64list: ZeroCopyBuffer(vec![42u64; n as usize]),
//         float32list: ZeroCopyBuffer(vec![42.0f32; n as usize]),
//         float64list: ZeroCopyBuffer(vec![42.0f64; n as usize]),
//     }
// }

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

pub fn handle_string_list(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

// Function that uses imported struct (from within this crate)
pub fn use_imported_struct(my_struct: MyStruct) -> bool {
    my_struct.content
}

// Function that uses imported enum (from within this crate)
pub fn use_imported_enum(my_enum: MyEnum) -> bool {
    match my_enum {
        MyEnum::False => false,
        MyEnum::True => true,
    }
}

// [T; N] example
pub fn get_array() -> [u8; 5] {
    [1, 2, 3, 4, 5]
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub fn get_complex_array() -> [Point; 2] {
    [Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 2.0 }]
}

// usize
pub fn get_usize(u: usize) -> usize {
    u
}

/// Example for @freezed and @meta.immutable
#[frb(dart_metadata=("freezed", "immutable" import "package:meta/meta.dart" as meta))]
pub struct UserId {
    #[frb(default = 0)]
    pub value: u32,
}

pub fn next_user_id(user_id: UserId) -> UserId {
    UserId {
        value: user_id.value + 1,
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub key: u32,
    pub value: u32,
}

pub struct SumWith {
    pub x: u32,
}

impl SumWith {
    pub fn sum(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

pub fn get_sum_struct() -> SumWith {
    SumWith { x: 21 }
}

pub fn get_sum_array(a: u32, b: u32, c: u32) -> [SumWith; 3] {
    [SumWith { x: a }, SumWith { x: b }, SumWith { x: c }]
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Speed {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Distance {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Measure {
    Speed(Box<Speed>),
    Distance(Box<Distance>),
}

pub fn multiply_by_ten(measure: Measure) -> Option<Measure> {
    match measure {
        Measure::Speed(b) => match *b {
            Speed::GPS(v) => Some(Measure::Speed(Box::new(Speed::GPS(v * 10.)))),
            Speed::Unknown => None,
        },
        Measure::Distance(b) => match *b {
            Distance::Map(v) => Some(Measure::Distance(Box::new(Distance::Map(v * 10.)))),
            Distance::Unknown => None,
        },
    }
}

pub fn call_old_module_system() -> OldSimpleStruct {
    use_old_module_system(2)
}
pub fn call_new_module_system() -> NewSimpleStruct {
    use_new_module_system(1)
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

pub struct MessageId(pub [u8; 32]);

pub fn new_msgid(id: [u8; 32]) -> MessageId {
    MessageId(id)
}

pub fn use_msgid(id: MessageId) -> [u8; 32] {
    id.0
}
pub struct Blob(pub [u8; 1600]);
pub fn boxed_blob(blob: Box<[u8; 1600]>) -> Blob {
    Blob(*blob)
}

pub fn use_boxed_blob(blob: Box<Blob>) -> [u8; 1600] {
    blob.0
}

pub struct FeedId(pub [u8; 8]);

pub fn return_boxed_feed_id(id: [u8; 8]) -> Box<FeedId> {
    Box::new(FeedId(id))
}

pub fn return_boxed_raw_feed_id(id: FeedId) -> Box<[u8; 8]> {
    Box::new(id.0)
}

pub struct TestId(pub [i32; 2]);

pub fn test_id(id: TestId) -> TestId {
    id
}

pub fn last_number(array: [f64; 16]) -> f64 {
    array[15]
}

pub fn nested_id(id: [TestId; 4]) -> [TestId; 2] {
    match id {
        [first, .., last] => [first, last],
    }
}

pub fn sync_void() -> SyncReturn<()> {
    SyncReturn(())
}

pub fn handle_type_alias_id(input: Id) -> Id {
    input
}

pub fn handle_type_nest_alias_id(input: UserIdAlias) -> Id {
    input
}
pub struct TestModel {
    pub id: Id,
    pub name: String,
    pub alias_enum: EnumAlias,
    pub alias_struct: MyStruct,
}

pub fn handle_type_alias_model(input: Id) -> TestModel {
    TestModel {
        id: input,
        name: "TestModel".to_owned(),
        alias_enum: EnumAlias::False,
        alias_struct: StructAlias { content: true },
    }
}

#[derive(Debug, Clone)]
pub struct Empty {}

pub fn empty_struct(empty: Empty) -> Empty {
    empty
}

pub fn return_dart_dynamic() -> DartAbi {
    vec!["foo".into_dart()].into_dart()
}

pub struct RawStringItemStruct {
    pub r#type: String,
}

pub fn test_raw_string_item_struct() -> RawStringItemStruct {
    RawStringItemStruct {
        r#type: "test".to_owned(),
    }
}

pub struct MoreThanJustOneRawStringStruct {
    pub regular: String,
    pub r#type: String,
    pub r#async: bool,
    pub another: String,
}

pub fn test_more_than_just_one_raw_string_struct() -> MoreThanJustOneRawStringStruct {
    MoreThanJustOneRawStringStruct {
        regular: "regular".to_owned(),
        r#type: "type".to_owned(),
        r#async: true,
        another: "another".to_owned(),
    }
}

//This seems to be a bug in the syn parser (v1), for whoever tries to fix it, after each failed build you need to manually remove all rust generated files (bridge_*)
// pub fn test_raw_string_item_struct_with_raw_string_in_func(r#type: String) -> RawStringItemStruct {
//     RawStringItemStruct { r#type }
// }

pub fn list_of_primitive_enums(weekdays: Vec<Weekdays>) -> Vec<Weekdays> {
    weekdays
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

pub struct ContainsMirroredSubStruct {
    pub test: RawStringMirrored,
    pub test2: A,
}

pub fn test_contains_mirrored_sub_struct() -> ContainsMirroredSubStruct {
    ContainsMirroredSubStruct {
        test: RawStringMirrored {
            r#value: "test".to_owned(),
        },
        test2: A {
            a: "test".to_owned(),
        },
    }
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
