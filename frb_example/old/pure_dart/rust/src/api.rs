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

/// Documentation on a simple adder function.
pub fn simple_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn simple_adder_sync(a: i32, b: i32) -> SyncReturn<i32> {
    SyncReturn(a + b)
}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.
 Newlines are preserved.
*/
pub fn primitive_types(my_i32: i32, my_i64: i64, my_f64: f64, my_bool: bool) -> i32 {
    info!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    42
}

pub fn primitive_optional_types(
    my_i32: Option<i32>,
    my_i64: Option<i64>,
    my_f64: Option<f64>,
    my_bool: Option<bool>,
) -> Option<i32> {
    info!(
        "primitive_optional_types({}, {}, {}, {})",
        my_i32.unwrap_or_default(),
        my_i64.unwrap_or_default(),
        my_f64.unwrap_or_default(),
        my_bool.unwrap_or_default()
    );
    Some(
        my_i32.is_some() as i32
            + my_i64.is_some() as i32
            + my_f64.is_some() as i32
            + my_bool.is_some() as i32,
    )
}

pub fn primitive_types_sync(
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) -> SyncReturn<i32> {
    info!(
        "primitive_types_sync({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    SyncReturn(42)
}

pub fn primitive_u32(my_u32: u32) -> u32 {
    info!("primitive_u32({})", my_u32);
    assert_eq!(my_u32, 0xff112233);
    let ret = 0xfe112233;
    info!("returning {}", ret);
    ret
}

pub fn primitive_u32_sync(my_u32: u32) -> SyncReturn<u32> {
    info!("primitive_u32_sync({})", my_u32);
    assert_eq!(my_u32, 0xff112233);
    let ret = 0xfe112233;
    info!("returning {}", ret);
    SyncReturn(ret)
}

pub fn handle_string(s: String) -> String {
    info!("handle_string({})", &s);
    let s2 = s.clone();
    s + &s2
}

pub fn handle_string_sync(s: String) -> SyncReturn<String> {
    info!("handle_string_sync({})", &s);
    let s2 = s.clone();
    SyncReturn(s + &s2)
}

#[allow(clippy::unused_unit)]
pub fn handle_return_unit() -> () {
    info!("handle_return_unit()");
}

pub fn handle_return_unit_sync() -> SyncReturn<()> {
    info!("handle_return_unit_sync()");
    SyncReturn(())
}

// to check that `Vec<u8>` can be used as return type
pub fn handle_vec_u8(v: Vec<u8>) -> Vec<u8> {
    info!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    v.repeat(2)
}

pub fn handle_vec_u8_sync(v: Vec<u8>) -> SyncReturn<Vec<u8>> {
    info!("handle_vec_u8_sync(first few elements: {:?})", &v[..5]);
    SyncReturn(v.repeat(2))
}

pub struct VecOfPrimitivePack {
    pub int8list: Vec<i8>,
    pub uint8list: Vec<u8>,
    pub int16list: Vec<i16>,
    pub uint16list: Vec<u16>,
    pub uint32list: Vec<u32>,
    pub int32list: Vec<i32>,
    pub uint64list: Vec<u64>,
    pub int64list: Vec<i64>,
    pub float32list: Vec<f32>,
    pub float64list: Vec<f64>,
    pub bool_list: Vec<bool>,
}

pub fn handle_vec_of_primitive(n: i32) -> VecOfPrimitivePack {
    VecOfPrimitivePack {
        int8list: vec![42i8; n as usize],
        uint8list: vec![42u8; n as usize],
        int16list: vec![42i16; n as usize],
        uint16list: vec![42u16; n as usize],
        int32list: vec![42i32; n as usize],
        uint32list: vec![42u32; n as usize],
        int64list: vec![42i64; n as usize],
        uint64list: vec![42u64; n as usize],
        float32list: vec![42.0f32; n as usize],
        float64list: vec![42.0f64; n as usize],
        bool_list: vec![true; n as usize],
    }
}

pub fn handle_vec_of_primitive_sync(n: i32) -> SyncReturn<VecOfPrimitivePack> {
    SyncReturn(VecOfPrimitivePack {
        int8list: vec![42i8; n as usize],
        uint8list: vec![42u8; n as usize],
        int16list: vec![42i16; n as usize],
        uint16list: vec![42u16; n as usize],
        int32list: vec![42i32; n as usize],
        uint32list: vec![42u32; n as usize],
        int64list: vec![42i64; n as usize],
        uint64list: vec![42u64; n as usize],
        float32list: vec![42.0f32; n as usize],
        float64list: vec![42.0f64; n as usize],
        bool_list: vec![true; n as usize],
    })
}

pub struct ZeroCopyVecOfPrimitivePack {
    pub int8list: ZeroCopyBuffer<Vec<i8>>,
    pub uint8list: ZeroCopyBuffer<Vec<u8>>,
    pub int16list: ZeroCopyBuffer<Vec<i16>>,
    pub uint16list: ZeroCopyBuffer<Vec<u16>>,
    pub uint32list: ZeroCopyBuffer<Vec<u32>>,
    pub int32list: ZeroCopyBuffer<Vec<i32>>,
    pub uint64list: ZeroCopyBuffer<Vec<u64>>,
    pub int64list: ZeroCopyBuffer<Vec<i64>>,
    pub float32list: ZeroCopyBuffer<Vec<f32>>,
    pub float64list: ZeroCopyBuffer<Vec<f64>>,
}

pub fn handle_zero_copy_vec_of_primitive(n: i32) -> ZeroCopyVecOfPrimitivePack {
    ZeroCopyVecOfPrimitivePack {
        int8list: ZeroCopyBuffer(vec![42i8; n as usize]),
        uint8list: ZeroCopyBuffer(vec![42u8; n as usize]),
        int16list: ZeroCopyBuffer(vec![42i16; n as usize]),
        uint16list: ZeroCopyBuffer(vec![42u16; n as usize]),
        int32list: ZeroCopyBuffer(vec![42i32; n as usize]),
        uint32list: ZeroCopyBuffer(vec![42u32; n as usize]),
        int64list: ZeroCopyBuffer(vec![42i64; n as usize]),
        uint64list: ZeroCopyBuffer(vec![42u64; n as usize]),
        float32list: ZeroCopyBuffer(vec![42.0f32; n as usize]),
        float64list: ZeroCopyBuffer(vec![42.0f64; n as usize]),
    }
}

pub fn handle_zero_copy_vec_of_primitive_sync(n: i32) -> SyncReturn<ZeroCopyVecOfPrimitivePack> {
    SyncReturn(ZeroCopyVecOfPrimitivePack {
        int8list: ZeroCopyBuffer(vec![42i8; n as usize]),
        uint8list: ZeroCopyBuffer(vec![42u8; n as usize]),
        int16list: ZeroCopyBuffer(vec![42i16; n as usize]),
        uint16list: ZeroCopyBuffer(vec![42u16; n as usize]),
        int32list: ZeroCopyBuffer(vec![42i32; n as usize]),
        uint32list: ZeroCopyBuffer(vec![42u32; n as usize]),
        int64list: ZeroCopyBuffer(vec![42i64; n as usize]),
        uint64list: ZeroCopyBuffer(vec![42u64; n as usize]),
        float32list: ZeroCopyBuffer(vec![42.0f32; n as usize]),
        float64list: ZeroCopyBuffer(vec![42.0f64; n as usize]),
    })
}

#[derive(Debug, Clone)]
pub struct MySize {
    pub width: i32,
    pub height: i32,
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Debug, Clone)]
pub struct MySizeFreezed {
    pub width: i32,
    pub height: i32,
}

pub fn handle_struct(arg: MySize, boxed: Box<MySize>) -> MySize {
    info!("handle_struct({:?}, {:?})", &arg, &boxed);
    MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}

pub fn handle_struct_sync(arg: MySize, boxed: Box<MySize>) -> SyncReturn<MySize> {
    info!("handle_struct_sync({:?}, {:?})", &arg, &boxed);
    SyncReturn(MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    })
}

pub fn handle_struct_sync_freezed(
    arg: MySizeFreezed,
    boxed: Box<MySizeFreezed>,
) -> SyncReturn<MySizeFreezed> {
    info!("handle_struct_sync_freezed({:?}, {:?})", &arg, &boxed);
    SyncReturn(MySizeFreezed {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    })
}

#[derive(Debug)]
pub struct NewTypeInt(pub i64);

pub fn handle_newtype(arg: NewTypeInt) -> NewTypeInt {
    info!("handle_newtype({:?})", &arg);
    NewTypeInt(arg.0 * 2)
}

pub fn handle_newtype_sync(arg: NewTypeInt) -> SyncReturn<NewTypeInt> {
    info!("handle_newtype_sync({:?})", &arg);
    SyncReturn(NewTypeInt(arg.0 * 2))
}

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

pub fn handle_list_of_struct_sync(mut l: Vec<MySize>) -> SyncReturn<Vec<MySize>> {
    info!("handle_list_of_struct_sync({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    SyncReturn(ans)
}

pub fn handle_string_list(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

pub fn handle_string_list_sync(names: Vec<String>) -> SyncReturn<Vec<String>> {
    for name in &names {
        info!("Hello, {}", name);
    }
    SyncReturn(names)
}

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> MyTreeNode {
    info!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    s
}

pub fn handle_complex_struct_sync(s: MyTreeNode) -> SyncReturn<MyTreeNode> {
    info!("handle_complex_struct_sync({:?})", &s);
    let s_cloned = s.clone();
    SyncReturn(s)
}

#[derive(Debug, Clone)]
pub struct MyNestedStruct {
    pub tree_node: MyTreeNode,
    pub weekday: Weekdays,
}

pub fn handle_nested_struct(s: MyNestedStruct) -> MyNestedStruct {
    println!("handle_nested_struct({s:?})");
    let s_cloned = s.clone();
    s
}

// Test if sync return is working as expected by using Vec<u8> as return value.
pub fn handle_sync_return(mode: String) -> Result<SyncReturn<Vec<u8>>> {
    match &mode[..] {
        "NORMAL" => Ok(SyncReturn(vec![42u8; 100])),
        "RESULT_ERR" => Err(anyhow!("deliberate error in handle_sync_return_err")),
        "PANIC" => panic!("deliberate panic in handle_sync_return_panic"),
        _ => panic!("unknown mode"),
    }
}

pub fn handle_stream(sink: StreamSink<String>, arg: String) {
    info!("handle_stream arg={}", arg);

    let cnt = Arc::new(AtomicI32::new(0));

    // just to show that, you can send data to sink even in other threads
    let cnt2 = cnt.clone();
    let sink2 = sink.clone();

    spawn!(|| {
        for i in 0..5 {
            let old_cnt = cnt2.fetch_add(1, Ordering::Relaxed);
            let msg = format!("(thread=child, i={i}, old_cnt={old_cnt})");
            format!("send data to sink msg={msg}");
            let _ = sink2.add(msg);
            sleep(Duration::from_millis(100));
        }
        sink2.close();
    });

    for i in 0..5 {
        let old_cnt = cnt.fetch_add(1, Ordering::Relaxed);
        let msg = format!("(thread=normal, i={i}, old_cnt={old_cnt})");
        format!("send data to sink msg={msg}");
        let _ = sink.add(msg);
        sleep(Duration::from_millis(50));
    }
}

pub struct MyStreamEntry {
    pub hello: String,
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub fn handle_stream_of_struct(sink: StreamSink<MyStreamEntry>) {
    // Ok(())
}

pub fn return_err() -> Result<i32> {
    Err(anyhow!(
        "return_err() is called, thus deliberately return Err"
    ))
}

pub fn return_panic() -> i32 {
    panic!("return_panic() is called, thus deliberately panic")
}

pub fn handle_optional_return(left: f64, right: f64) -> Option<f64> {
    if right == 0. {
        None
    } else {
        Some(left / right)
    }
}

#[derive(Default, Debug, Clone)]
pub struct Element {
    pub tag: Option<String>,
    pub text: Option<String>,
    pub attributes: Option<Vec<Attribute>>,
    pub children: Option<Vec<Element>>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

pub fn handle_optional_struct(document: Option<String>) -> Option<Element> {
    document.map(|inner| Element {
        tag: Some("div".to_owned()),
        attributes: Some(vec![Attribute {
            key: "id".to_owned(),
            value: "root".to_owned(),
        }]),
        children: Some(vec![Element {
            tag: Some("p".to_owned()),
            children: Some(vec![Element {
                text: Some(inner),
                ..Default::default()
            }]),
            ..Default::default()
        }]),
        ..Default::default()
    })
}

#[derive(Debug)]
pub struct ExoticOptionals {
    pub int32: Option<i32>,
    pub int64: Option<i64>,
    pub float64: Option<f64>,
    pub boolean: Option<bool>,
    pub zerocopy: Option<ZeroCopyBuffer<Vec<u8>>>,
    pub int8list: Option<Vec<i8>>,
    pub uint8list: Option<Vec<u8>>,
    pub int32list: Option<Vec<i32>>,
    pub float32list: Option<Vec<f32>>,
    pub float64list: Option<Vec<f64>>,
    pub attributes: Option<Vec<Attribute>>,
    pub attributes_nullable: Vec<Option<Attribute>>,
    pub nullable_attributes: Option<Vec<Option<Attribute>>>,
    pub newtypeint: Option<NewTypeInt>,
}

pub fn handle_optional_increment(opt: Option<ExoticOptionals>) -> Option<ExoticOptionals> {
    fn manipulate_list<T>(src: Option<Vec<T>>, push_value: T) -> Option<Vec<T>> {
        let mut list = src.unwrap_or_default();
        list.push(push_value);
        Some(list)
    }

    opt.map(|mut opt| ExoticOptionals {
        int32: Some(opt.int32.unwrap_or(0) + 1),
        int64: Some(opt.int64.unwrap_or(0) + 1),
        float64: Some(opt.float64.unwrap_or(0.) + 1.),
        boolean: Some(!opt.boolean.unwrap_or(false)),
        int8list: manipulate_list(opt.int8list, 0),
        uint8list: manipulate_list(opt.uint8list, 0),
        int32list: manipulate_list(opt.int32list, 0),
        float32list: manipulate_list(opt.float32list, 0.),
        float64list: manipulate_list(opt.float64list, 0.),
        attributes: Some({
            let mut list = opt.attributes.unwrap_or_default();
            list.push(Attribute {
                key: "some-attrib".to_owned(),
                value: "some-value".to_owned(),
            });
            list
        }),
        nullable_attributes: Some({
            let mut list = opt.nullable_attributes.unwrap_or_default();
            list.push(None);
            list
        }),
        newtypeint: Some({
            let mut val = opt.newtypeint.unwrap_or(NewTypeInt(0));
            val.0 += 1;
            val
        }),
        attributes_nullable: {
            opt.attributes_nullable.push(None);
            opt.attributes_nullable
        },
        zerocopy: Some({
            let mut list = opt.zerocopy.unwrap_or_else(|| ZeroCopyBuffer(vec![]));
            list.0.push(0);
            list
        }),
    })
}

pub fn handle_increment_boxed_optional(opt: Option<Box<f64>>) -> f64 {
    match opt {
        Some(e) => *e + 1.,
        None => 42.,
    }
}

pub struct OptVecs {
    pub i32: Vec<Option<i32>>,
    pub enums: Vec<Option<Weekdays>>,
    pub strings: Vec<Option<String>>,
    pub buffers: Vec<Option<Vec<i32>>>,
}

pub fn handle_vec_of_opts(opt: OptVecs) -> OptVecs {
    fn handle<T>(mut opts: Vec<Option<T>>) -> Vec<Option<T>> {
        opts.push(None);
        opts
    }
    OptVecs {
        i32: handle(opt.i32),
        enums: handle(opt.enums),
        strings: handle(opt.strings),
        buffers: handle(opt.buffers),
    }
}

// Option<Box<T>> can't be sent to Dart,
// but instead can be received by Rust.
pub fn handle_option_box_arguments(
    i8box: Option<Box<i8>>,
    u8box: Option<Box<u8>>,
    i32box: Option<Box<i32>>,
    i64box: Option<Box<i64>>,
    f64box: Option<Box<f64>>,
    boolbox: Option<Box<bool>>,
    structbox: Option<Box<ExoticOptionals>>,
) -> String {
    format!(
        "handle_option_box_arguments({:?})",
        (i8box, u8box, i32box, i64box, f64box, boolbox, structbox)
    )
}

/// Simple enums.
#[derive(Debug, Clone, Copy)]
pub enum Weekdays {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    /// Best day of the week.
    Saturday,
    Sunday,
}

#[frb]
#[derive(Debug)]
pub struct Note {
    #[frb(default = "Weekdays.Sunday")]
    pub day: Box<Weekdays>,
    pub body: String,
}

pub fn print_note(note: Note) -> ZeroCopyBuffer<Vec<u8>> {
    info!("{:#?}", note);
    ZeroCopyBuffer(vec![1, 2, 3])
}

pub fn handle_return_enum(input: String) -> Option<Weekdays> {
    match input.as_str() {
        "Monday" => Some(Weekdays::Monday),
        "Tuesday" => Some(Weekdays::Tuesday),
        "Wednesday" => Some(Weekdays::Wednesday),
        "Thursday" => Some(Weekdays::Thursday),
        "Friday" => Some(Weekdays::Friday),
        "Saturday" => Some(Weekdays::Saturday),
        "Sunday" => Some(Weekdays::Sunday),
        _ => None,
    }
}

pub fn handle_enum_parameter(weekday: Weekdays) -> Weekdays {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug)]
pub enum MyEnumFreezed {
    A(i32),
    B(String),
}

pub fn handle_enum_sync_freezed(value: MyEnumFreezed) -> SyncReturn<MyEnumFreezed> {
    info!("handle_struct_sync_enum_freezed({:?})", &value);
    SyncReturn(MyEnumFreezed::B("hello".to_string()))
}

#[frb]
#[derive(Debug, Clone)]
pub struct Customized {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub fn handle_customized_struct(val: Customized) {
    info!("{:#?}", val);
}

#[frb]
#[derive(Debug)]
pub enum KitchenSink {
    /// Comment on variant
    Empty,
    #[frb(unimpl_variant_attr)]
    Primitives {
        /// Dart field comment
        #[frb(default = -1)]
        int32: i32,
        #[frb(unimpl_deprecated)]
        float64: f64,
        boolean: bool,
    },
    Nested(
        i32,
        #[frb(default = "KitchenSink.empty()")] Box<KitchenSink>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(#[frb(default = "Weekdays.Sunday")] Weekdays),
}

#[frb(unimpl_fn_attr)]
pub fn handle_enum_struct(val: KitchenSink) -> KitchenSink {
    use KitchenSink::*;
    use Weekdays::*;
    let inc = |x| x + 1;
    match val {
        Primitives {
            int32,
            float64,
            boolean,
        } => Primitives {
            int32: int32 + 1,
            float64: float64 + 1.,
            boolean: !boolean,
        },
        Nested(val, nested) => Nested(inc(val), Box::new(handle_enum_struct(*nested))),
        Optional(a, b) => Optional(a.map(inc), b.map(inc)),
        Buffer(ZeroCopyBuffer(mut buf)) => {
            buf.push(1);
            Buffer(ZeroCopyBuffer(buf))
        }
        Enums(day) => Enums(match day {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }),
        _ => val,
    }
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

// Mirroring example:
// The goal of mirroring is to use external objects without needing to convert them with an intermediate type
// In this case, the struct ApplicationSettings is defined in another crate (called external-lib)

// To use an external type with mirroring, it MUST be imported publicly (aka. re-export)
pub use external_lib::{
    ApplicationEnv, ApplicationEnvVar, ApplicationMessage, ApplicationMode, ApplicationSettings,
    ListOfNestedRawStringMirrored, NestedRawStringMirrored, Numbers, RawStringEnumMirrored,
    RawStringMirrored, Sequences,
};

// To mirror an external struct, you need to define a placeholder type with the same definition
#[frb(mirror(ApplicationSettings))]
pub struct _ApplicationSettings {
    pub name: String,
    pub version: String,
    pub mode: ApplicationMode,
    pub env: Box<ApplicationEnv>,
    pub env_optional: Option<ApplicationEnv>,
}

#[frb(mirror(ApplicationMode))]
pub enum _ApplicationMode {
    Standalone,
    Embedded,
}

#[frb(mirror(ApplicationEnvVar))]
pub struct _ApplicationEnvVar(pub String, pub bool);

#[frb(mirror(ApplicationEnv))]
pub struct _ApplicationEnv {
    pub vars: Vec<ApplicationEnvVar>,
}

// This function can directly return an object of the external type ApplicationSettings because it has a mirror
pub fn get_app_settings() -> ApplicationSettings {
    external_lib::get_app_settings()
}

// This function can return a Result, that includes an object of the external type ApplicationSettings because it has a mirror
pub fn get_fallible_app_settings() -> anyhow::Result<ApplicationSettings> {
    Ok(external_lib::get_app_settings())
}

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
pub fn is_app_embedded(app_settings: ApplicationSettings) -> bool {
    // info!("env: {:?}", app_settings.env.vars);
    matches!(app_settings.mode, ApplicationMode::Embedded)
}

// use a stream of a mirrored type
pub fn app_settings_stream(sink: StreamSink<ApplicationSettings>) {
    let app_settings = external_lib::get_app_settings();
    sink.add(app_settings);
    sink.close();
}

// use a stream of a vec of mirrored type
pub fn app_settings_vec_stream(sink: StreamSink<Vec<ApplicationSettings>>) {
    let app_settings = vec![
        external_lib::get_app_settings(),
        external_lib::get_app_settings(),
    ];
    sink.add(app_settings);
    sink.close();
}

pub struct MirrorStruct {
    pub a: ApplicationSettings,
    pub b: MyStruct,
    pub c: Vec<MyEnum>,
    pub d: Vec<ApplicationSettings>,
}

// use a Struct consisting of mirror types as argument to a Stream
pub fn mirror_struct_stream(sink: StreamSink<MirrorStruct>) {
    let val = MirrorStruct {
        a: external_lib::get_app_settings(),
        b: MyStruct { content: true },
        c: vec![MyEnum::True, MyEnum::False],
        d: vec![
            external_lib::get_app_settings(),
            external_lib::get_app_settings(),
        ],
    };
    sink.add(val);
    sink.close();
}

// usa a tuple of Mirror types for a StreamSink
pub fn mirror_tuple_stream(sink: StreamSink<(ApplicationSettings, RawStringEnumMirrored)>) {
    let tuple = (
        external_lib::get_app_settings(),
        RawStringEnumMirrored::Raw(RawStringMirrored {
            value: String::from("test"),
        }),
    );
    sink.add(tuple);
    sink.close();
}

#[frb(mirror(ApplicationMessage))]
pub enum _ApplicationMessage {
    DisplayMessage(String),
    RenderPixel { x: i32, y: i32 },
    Exit,
}

pub fn get_message() -> ApplicationMessage {
    external_lib::poll_messages()[1].clone()
}

#[frb(mirror(Numbers, Sequences))]
pub struct _Numbers(pub Vec<i32>);

pub fn repeat_number(num: i32, times: usize) -> Numbers {
    external_lib::repeat_number(num, times)
}

pub fn repeat_sequence(seq: i32, times: usize) -> Sequences {
    external_lib::repeat_sequences(seq, times)
}

pub fn first_number(nums: Numbers) -> Option<i32> {
    nums.0.first().copied()
}

pub fn first_sequence(seqs: Sequences) -> Option<i32> {
    seqs.0.first().copied()
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

// event listener test

lazy_static! {
    static ref EVENTS: Mutex<Option<StreamSink<Event>>> = Default::default();
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Clone)]
pub struct Event {
    pub address: String,
    pub payload: String,
}

impl Event {
    pub fn as_string(&self) -> String {
        format!("{}: {}", self.address, self.payload)
    }
}

pub fn register_event_listener(listener: StreamSink<Event>) -> Result<()> {
    match EVENTS.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

pub fn close_event_listener() {
    if let Ok(Some(sink)) = EVENTS.lock().map(|mut guard| guard.take()) {
        sink.close();
    }
}

pub fn create_event(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(Event { address, payload });
        }
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub key: u32,
    pub value: u32,
}

pub fn handle_stream_sink_at_1(key: u32, max: u32, sink: StreamSink<Log>) {
    spawn!(|| {
        for i in 0..max {
            let _ = sink.add(Log { key, value: i });
        }
        sink.close();
    });
}

pub fn handle_stream_sink_at_2(key: u32, sink: StreamSink<Log>, max: u32) {
    handle_stream_sink_at_1(key, max, sink)
}

pub fn handle_stream_sink_at_3(sink: StreamSink<Log>, key: u32, max: u32) {
    handle_stream_sink_at_1(key, max, sink)
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

pub struct ConcatenateWith {
    pub a: String,
}

#[derive(Debug, Clone)]
pub struct Log2 {
    pub key: u32,
    pub value: String,
}

impl ConcatenateWith {
    /// Documentation on a static method
    pub fn new(a: String) -> ConcatenateWith {
        ConcatenateWith { a }
    }
    /// Documentation on an instance method
    pub fn concatenate(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }
    pub fn concatenate_static(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    pub fn handle_some_stream_sink(&self, key: u32, max: u32, sink: StreamSink<Log2>) {
        let a = self.a.clone();
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: format!("{a}{i}"),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_stream_sink_at_1(&self, sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink(key: u32, max: u32, sink: StreamSink<Log2>) {
        spawn!(|| {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: i.to_string(),
                });
            }
            sink.close();
        });
    }

    pub fn handle_some_static_stream_sink_single_arg(sink: StreamSink<u32>) {
        spawn!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
    }
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

pub fn datetime_utc(d: chrono::DateTime<chrono::Utc>) -> chrono::DateTime<chrono::Utc> {
    use chrono::Datelike;
    use chrono::Timelike;
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    assert_eq!(&d.hour(), &20);
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&d.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&d.nanosecond(), &123_456_000);
    d
}

pub fn datetime_local(d: chrono::DateTime<chrono::Local>) -> chrono::DateTime<chrono::Local> {
    use chrono::Datelike;
    use chrono::Timelike;
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    if cfg!(target_arch = "wasm32") {
        assert_eq!(&d.nanosecond(), &123_000_000);
    } else {
        assert_eq!(&d.hour(), &20);
        assert_eq!(&d.nanosecond(), &123_456_000);
    }
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    d
}

pub fn naivedatetime(d: chrono::NaiveDateTime) -> chrono::NaiveDateTime {
    use chrono::{Datelike, Timelike};
    assert_eq!(&d.year(), &2022);
    assert_eq!(&d.month(), &9);
    assert_eq!(&d.day(), &10);
    assert_eq!(&d.hour(), &20);
    assert_eq!(&d.minute(), &48);
    assert_eq!(&d.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&d.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&d.nanosecond(), &123_456_000);
    d
}

pub fn optional_empty_datetime_utc(
    d: Option<chrono::DateTime<chrono::Utc>>,
) -> Option<chrono::DateTime<chrono::Utc>> {
    assert_eq!(&d, &None);
    d
}

pub fn duration(d: chrono::Duration) -> chrono::Duration {
    assert_eq!(&d.num_hours(), &4);
    d
}

pub fn handle_timestamps(
    timestamps: Vec<chrono::NaiveDateTime>,
    epoch: chrono::NaiveDateTime,
) -> Vec<chrono::Duration> {
    timestamps
        .into_iter()
        .map(|ts| epoch.signed_duration_since(ts))
        .collect()
}

pub fn handle_durations(
    durations: Vec<chrono::Duration>,
    since: chrono::DateTime<chrono::Local>,
) -> Vec<chrono::DateTime<chrono::Local>> {
    durations.into_iter().map(|dur| since - dur).collect()
}

pub struct TestChrono {
    pub dt: Option<chrono::DateTime<chrono::Utc>>,
    pub dt2: Option<chrono::NaiveDateTime>,
    pub du: Option<chrono::Duration>,
}

pub fn test_chrono() -> TestChrono {
    TestChrono {
        dt: Some(chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(1631297333, 0).unwrap(),
            chrono::Utc,
        )),
        dt2: Some(chrono::NaiveDateTime::from_timestamp_opt(1631297333, 0).unwrap()),
        du: Some(chrono::Duration::hours(4)),
    }
}

pub fn test_precise_chrono() -> TestChrono {
    TestChrono {
        dt: Some(chrono::DateTime::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(1014466435, 0).unwrap(),
            chrono::Utc,
        )),
        dt2: Some(chrono::NaiveDateTime::from_timestamp_opt(-5362715015, 0).unwrap()),
        du: Some(chrono::Duration::hours(4)),
    }
}

#[derive(Debug, Clone)]
pub struct FeatureChrono {
    pub utc: chrono::DateTime<chrono::Utc>,
    pub local: chrono::DateTime<chrono::Local>,
    pub duration: chrono::Duration,
    pub naive: chrono::NaiveDateTime,
}

pub fn how_long_does_it_take(mine: FeatureChrono) -> anyhow::Result<chrono::Duration> {
    use chrono::{Datelike, Timelike};
    let difference: chrono::Duration = chrono::Utc::now() - mine.utc;
    assert_eq!(&mine.duration.num_hours(), &4);
    assert_eq!(&mine.naive.year(), &2022);
    assert_eq!(&mine.naive.month(), &9);
    assert_eq!(&mine.naive.day(), &10);
    assert_eq!(&mine.naive.hour(), &20);
    assert_eq!(&mine.naive.minute(), &48);
    assert_eq!(&mine.naive.second(), &53);
    #[cfg(target_arch = "wasm32")]
    assert_eq!(&mine.naive.nanosecond(), &123_000_000);
    #[cfg(not(target_arch = "wasm32"))]
    assert_eq!(&mine.naive.nanosecond(), &123_456_000);
    Ok(difference)
}

#[derive(Debug, Clone)]
pub struct FeatureUuid {
    pub one: uuid::Uuid,
    pub many: Vec<uuid::Uuid>,
}

pub fn handle_uuid(id: uuid::Uuid) -> anyhow::Result<uuid::Uuid> {
    Ok(id)
}

pub fn handle_uuids(ids: Vec<uuid::Uuid>) -> anyhow::Result<Vec<uuid::Uuid>> {
    Ok(ids)
}

pub fn handle_nested_uuids(ids: FeatureUuid) -> anyhow::Result<FeatureUuid> {
    Ok(ids)
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

pub fn sync_accept_dart_opaque(opaque: DartOpaque) -> SyncReturn<String> {
    drop(opaque);
    SyncReturn("test".to_owned())
}

pub fn async_accept_dart_opaque(opaque: DartOpaque) -> String {
    drop(opaque);
    "async test".to_owned()
}

pub fn loop_back(opaque: DartOpaque) -> DartOpaque {
    opaque
}

pub fn loop_back_option(opaque: DartOpaque) -> Option<DartOpaque> {
    Some(opaque)
}

pub fn loop_back_array(opaque: DartOpaque) -> [DartOpaque; 1] {
    [opaque]
}

pub fn loop_back_vec(opaque: DartOpaque) -> Vec<DartOpaque> {
    vec![opaque]
}

pub fn loop_back_option_get(opaque: Option<DartOpaque>) {}

pub fn loop_back_array_get(opaque: [DartOpaque; 1]) {}

pub fn loop_back_vec_get(opaque: Vec<DartOpaque>) {}

/// [DartWrapObject] can be safely retrieved on a dart thread.
pub fn unwrap_dart_opaque(opaque: DartOpaque) -> SyncReturn<String> {
    let handle = opaque.try_unwrap().unwrap();
    SyncReturn("Test".to_owned())
}

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
pub fn panic_unwrap_dart_opaque(opaque: DartOpaque) {
    let handle = opaque.try_unwrap().unwrap();
}

/// Opaque types
pub trait DartDebug: DartSafe + Debug + Send + Sync {}
impl<T: DartSafe + Debug + Send + Sync> DartDebug for T {}

pub enum EnumOpaque {
    Struct(RustOpaque<HideData>),
    Primitive(RustOpaque<i32>),
    TraitObj(RustOpaque<Box<dyn DartDebug>>),
    Mutex(RustOpaque<Mutex<HideData>>),
    RwLock(RustOpaque<RwLock<HideData>>),
}

pub enum EnumDartOpaque {
    Primitive(i32),
    Opaque(DartOpaque),
}

/// [`HideData`] has private fields.
pub struct OpaqueNested {
    pub first: RustOpaque<HideData>,
    pub second: RustOpaque<HideData>,
}

pub struct DartOpaqueNested {
    pub first: DartOpaque,
    pub second: DartOpaque,
}

pub fn create_opaque() -> RustOpaque<HideData> {
    RustOpaque::new(HideData::new())
}

pub fn create_option_opaque(opaque: Option<RustOpaque<HideData>>) -> Option<RustOpaque<HideData>> {
    opaque
}

pub fn sync_create_opaque() -> SyncReturn<RustOpaque<HideData>> {
    SyncReturn(RustOpaque::new(HideData::new()))
}

pub fn create_array_opaque_enum() -> [EnumOpaque; 5] {
    [
        EnumOpaque::Struct(RustOpaque::new(HideData::new())),
        EnumOpaque::Primitive(RustOpaque::new(42)),
        EnumOpaque::TraitObj(opaque_dyn!("String")),
        EnumOpaque::Mutex(RustOpaque::new(Mutex::new(HideData::new()))),
        EnumOpaque::RwLock(RustOpaque::new(RwLock::new(HideData::new()))),
    ]
}

pub fn run_enum_opaque(opaque: EnumOpaque) -> String {
    match opaque {
        EnumOpaque::Struct(s) => run_opaque(s),
        EnumOpaque::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaque::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaque::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaque::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    opaque.hide_data()
}

pub fn run_opaque_with_delay(opaque: RustOpaque<HideData>) -> String {
    sleep(Duration::from_millis(1000));
    opaque.hide_data()
}

pub fn opaque_array() -> [RustOpaque<HideData>; 2] {
    [
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

pub fn sync_create_non_clone() -> SyncReturn<RustOpaque<NonCloneData>> {
    SyncReturn(RustOpaque::new(NonCloneData::new()))
}

#[allow(clippy::redundant_clone)]
pub fn run_non_clone(clone: RustOpaque<NonCloneData>) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

pub fn create_sync_opaque() -> RustOpaque<NonSendHideData> {
    RustOpaque::new(NonSendHideData::new())
}

pub fn sync_create_sync_opaque() -> SyncReturn<RustOpaque<NonSendHideData>> {
    SyncReturn(RustOpaque::new(NonSendHideData::new()))
}

// OpaqueSyncStruct does not implement Send trait.
//
// pub fn run_opaque(opaque: Opaque<OpaqueSyncStruct>) -> String {
//     data.0.hide_data()
// }

pub fn sync_run_opaque(opaque: RustOpaque<NonSendHideData>) -> SyncReturn<String> {
    SyncReturn(opaque.hide_data())
}

pub fn opaque_array_run(data: [RustOpaque<HideData>; 2]) {
    for i in data {
        i.hide_data();
    }
}

pub fn opaque_vec() -> Vec<RustOpaque<HideData>> {
    vec![
        RustOpaque::new(HideData::new()),
        RustOpaque::new(HideData::new()),
    ]
}

pub fn opaque_vec_run(data: Vec<RustOpaque<HideData>>) {
    for i in data {
        i.hide_data();
    }
}

pub fn create_nested_opaque() -> OpaqueNested {
    OpaqueNested {
        first: RustOpaque::new(HideData::new()),
        second: RustOpaque::new(HideData::new()),
    }
}

pub fn sync_loopback(opaque: DartOpaque) -> SyncReturn<DartOpaque> {
    SyncReturn(opaque)
}

pub fn sync_option_loopback(opaque: Option<DartOpaque>) -> SyncReturn<Option<DartOpaque>> {
    SyncReturn(opaque)
}

pub fn sync_option() -> Result<SyncReturn<Option<String>>> {
    Ok(SyncReturn(Some("42".to_owned())))
}

pub fn sync_option_null() -> Result<SyncReturn<Option<String>>> {
    Ok(SyncReturn(None))
}

pub fn sync_option_rust_opaque() -> Result<SyncReturn<Option<RustOpaque<HideData>>>> {
    Ok(SyncReturn(Some(RustOpaque::new(HideData::new()))))
}

pub fn sync_option_dart_opaque(opaque: DartOpaque) -> Result<SyncReturn<Option<DartOpaque>>> {
    Ok(SyncReturn(Some(opaque)))
}

pub fn sync_void() -> SyncReturn<()> {
    SyncReturn(())
}

pub fn run_nested_opaque(opaque: OpaqueNested) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

pub fn create_nested_dart_opaque(opaque1: DartOpaque, opaque2: DartOpaque) -> DartOpaqueNested {
    DartOpaqueNested {
        first: opaque1,
        second: opaque2,
    }
}

pub fn get_nested_dart_opaque(opaque: DartOpaqueNested) {}

pub fn create_enum_dart_opaque(opaque: DartOpaque) -> EnumDartOpaque {
    EnumDartOpaque::Opaque(opaque)
}

pub fn get_enum_dart_opaque(opaque: EnumDartOpaque) {}

lazy_static! {
    static ref DART_OPAQUE: Mutex<Option<DartOpaque>> = Default::default();
}

pub fn set_static_dart_opaque(opaque: DartOpaque) {
    *DART_OPAQUE.lock().unwrap() = Some(opaque);
}

pub fn drop_static_dart_opaque() {
    drop(DART_OPAQUE.lock().unwrap().take());
}

pub fn unwrap_rust_opaque(opaque: RustOpaque<HideData>) -> Result<String> {
    let data: HideData = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

pub fn return_non_droppable_dart_opaque(opaque: DartOpaque) -> SyncReturn<DartOpaque> {
    let raw = opaque.try_unwrap().unwrap();
    SyncReturn(unsafe { DartOpaque::new_non_droppable(raw.into()) })
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub fn frb_generator_test() -> RustOpaque<FrbOpaqueReturn> {
    panic!("dummy code");
}

/// Structure for testing the SyncReturn<RustOpaque> code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
pub fn frb_sync_generator_test() -> SyncReturn<RustOpaque<FrbOpaqueSyncReturn>> {
    SyncReturn(RustOpaque::new(FrbOpaqueSyncReturn))
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

#[frb(mirror(RawStringMirrored))]
pub struct _RawStringMirrored {
    pub r#value: String,
}

#[frb(mirror(NestedRawStringMirrored))]
pub struct _NestedRawStringMirrored {
    pub raw: RawStringMirrored,
}

#[frb(mirror(RawStringEnumMirrored))]
pub enum _RawStringEnumMirrored {
    Raw(RawStringMirrored),
    Nested(NestedRawStringMirrored),
    ListOfNested(ListOfNestedRawStringMirrored),
}

#[frb(mirror(ListOfNestedRawStringMirrored))]
pub struct _ListOfRawNestedStringMirrored {
    pub raw: Vec<NestedRawStringMirrored>,
}

pub fn test_raw_string_mirrored() -> RawStringMirrored {
    RawStringMirrored {
        r#value: "test".to_owned(),
    }
}

pub fn test_nested_raw_string_mirrored() -> NestedRawStringMirrored {
    NestedRawStringMirrored {
        raw: RawStringMirrored {
            r#value: "test".to_owned(),
        },
    }
}

pub fn test_raw_string_enum_mirrored(nested: bool) -> RawStringEnumMirrored {
    if nested {
        RawStringEnumMirrored::Nested(NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        })
    } else {
        RawStringEnumMirrored::Raw(RawStringMirrored {
            r#value: "test".to_owned(),
        })
    }
}

pub fn test_list_of_raw_nested_string_mirrored() -> ListOfNestedRawStringMirrored {
    ListOfNestedRawStringMirrored {
        raw: vec![NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        }],
    }
}

pub fn test_fallible_of_raw_string_mirrored() -> Result<Vec<RawStringMirrored>> {
    Ok(vec![RawStringMirrored {
        r#value: "test".to_owned(),
    }])
}

// pub fn test_list_of_nested_enums_mirrored() -> Vec<RawStringEnumMirrored> {
//     vec![
//         RawStringEnumMirrored::Nested(NestedRawStringMirrored {
//             raw: RawStringMirrored {
//                 r#value: "test".to_owned(),
//             },
//         }),
//         RawStringEnumMirrored::Raw(RawStringMirrored {
//             r#value: "test".to_owned(),
//         }),
//     ]
// }

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

pub fn test_tuple(value: Option<(String, i32)>) -> (String, i32) {
    if let Some((name, value)) = value {
        (format!("Hello {name}"), value + 1)
    } else {
        ("John".to_string(), 0)
    }
}

pub fn test_tuple_2(value: Vec<(String, i32)>) {
    drop(value)
}

pub fn sync_return_mirror() -> SyncReturn<ApplicationSettings> {
    SyncReturn(external_lib::get_app_settings())
}

macro_rules! generate_struct {
    () => {
        #[frb]
        pub struct MacroStruct {
            pub data: i32,
            #[frb(non_final)]
            pub non_final_data: i32,
        }
        #[allow(unused)]
        pub fn macro_struct() -> MacroStruct {
            MacroStruct {
                data: 123,
                non_final_data: 0,
            }
        }
    };
}

generate_struct!();

pub enum CustomError {
    Error0 { e: String, backtrace: Backtrace },
    Error1 { e: u32, backtrace: Backtrace },
}

pub fn return_err_custom_error() -> Result<u32, CustomError> {
    Err(CustomError::Error0 {
        e: "".into(),
        backtrace: Backtrace::new(),
    })
}

pub fn return_ok_custom_error() -> Result<u32, CustomError> {
    Ok(3)
}

pub fn return_error_variant(variant: u32) -> Result<u32, CustomError> {
    match variant {
        0 => Err(CustomError::Error0 {
            e: "variant0".to_string(),
            backtrace: Backtrace::new(),
        }),
        1 => Err(CustomError::Error1 {
            e: 1,
            backtrace: Backtrace::new(),
        }),
        _ => panic!("unsupported variant"),
    }
}

pub struct SomeStruct {
    pub value: u32,
}

impl SomeStruct {
    pub fn new(value: u32) -> SomeStruct {
        SomeStruct { value }
    }
    pub fn static_return_err_custom_error() -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: 3,
            backtrace: Backtrace::new(),
        })
    }

    pub fn static_return_ok_custom_error() -> Result<u32, CustomError> {
        Ok(3)
    }

    pub fn non_static_return_err_custom_error(&self) -> Result<u32, CustomError> {
        Err(CustomError::Error1 {
            e: self.value,
            backtrace: Backtrace::new(),
        })
    }

    pub fn non_static_return_ok_custom_error(&self) -> Result<u32, CustomError> {
        Ok(self.value)
    }
}

pub enum CustomNestedError1 {
    CustomNested1(String),
    ErrorNested(CustomNestedError2),
}

pub enum CustomNestedError2 {
    CustomNested2(String),
    CustomNested2Number(u32),
}

pub fn return_custom_nested_error_1() -> Result<(), CustomNestedError1> {
    Err(CustomNestedError1::ErrorNested(
        CustomNestedError2::CustomNested2Number(3),
    ))
}

pub fn return_custom_nested_error_1_variant1() -> Result<(), CustomNestedError1> {
    Err(CustomNestedError1::CustomNested1("custom".to_string()))
}

pub fn return_custom_nested_error_2() -> Result<(), CustomNestedError2> {
    Err(CustomNestedError2::CustomNested2("custom".to_string()))
}
pub struct CustomStructError {
    pub message: String,
}

pub fn return_custom_struct_error() -> Result<(), CustomStructError> {
    Err(CustomStructError {
        message: "error message".to_string(),
    })
}

pub fn sync_return_custom_struct_error() -> Result<SyncReturn<()>, CustomStructError> {
    Err(CustomStructError {
        message: "error message".to_string(),
    })
}

pub fn return_custom_struct_ok() -> Result<u32, CustomStructError> {
    Ok(3)
}

pub struct CustomStruct {
    pub message: String,
}

impl CustomStruct {
    pub fn new(message: String) -> CustomStruct {
        CustomStruct { message }
    }

    pub fn static_return_custom_struct_error() -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: "error message".to_string(),
        })
    }

    pub fn static_return_custom_struct_ok() -> Result<u32, CustomStructError> {
        Ok(3)
    }

    pub fn nonstatic_return_custom_struct_error(&self) -> Result<(), CustomStructError> {
        Err(CustomStructError {
            message: self.message.clone(),
        })
    }

    pub fn nonstatic_return_custom_struct_ok(&self) -> Result<u32, CustomStructError> {
        Ok(3)
    }
}

pub fn throw_anyhow() -> Result<(), anyhow::Error> {
    Err(anyhow!("anyhow error"))
}

pub fn panic_with_custom_result() -> Result<(), CustomError> {
    panic!("just a panic");
}

pub fn stream_sink_throw_anyhow(_sink: StreamSink<String>) -> Result<()> {
    Err(anyhow!("anyhow error"))
}
