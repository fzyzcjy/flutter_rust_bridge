#![allow(unused_variables)]

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};

use flutter_rust_bridge::*;

use crate::data::{MyEnum, MyStruct};

/// Documentation on a simple adder function.
pub fn simple_adder(a: i32, b: i32) -> i32 {
    a + b
}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.

 Newlines are preserved.
*/
pub fn primitive_types(my_i32: i32, my_i64: i64, my_f64: f64, my_bool: bool) -> i32 {
    println!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    42
}

pub fn primitive_u32(my_u32: u32) -> u32 {
    println!("primitive_u32({})", my_u32);
    assert_eq!(my_u32, 0xff112233);
    let ret = 0xfe112233;
    println!("returning {}", ret);
    ret
}

pub fn handle_string(s: String) -> String {
    println!("handle_string({})", &s);
    let s2 = s.clone();
    s + &s2
}

#[allow(clippy::unused_unit)]
pub fn handle_return_unit() -> () {
    println!("handle_return_unit()");
}

// to check that `Vec<u8>` can be used as return type
pub fn handle_vec_u8(v: Vec<u8>) -> Vec<u8> {
    println!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    v.repeat(2)
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
    }
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

#[derive(Debug, Clone)]
pub struct MySize {
    pub width: i32,
    pub height: i32,
}

pub fn handle_struct(arg: MySize, boxed: Box<MySize>) -> MySize {
    println!("handle_struct({:?}, {:?})", &arg, &boxed);
    MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    }
}

#[derive(Debug)]
pub struct NewTypeInt(pub i64);

pub fn handle_newtype(arg: NewTypeInt) -> NewTypeInt {
    println!("handle_newtype({:?})", &arg);
    NewTypeInt(arg.0 * 2)
}

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Vec<MySize> {
    println!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

pub fn handle_string_list(names: Vec<String>) -> Vec<String> {
    for name in &names {
        println!("Hello, {}", name);
    }
    names
}

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub value_boolean: bool,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> MyTreeNode {
    println!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    s
}

pub fn handle_sync_return(mode: String) -> Result<SyncReturn<Vec<u8>>> {
    match &mode[..] {
        "NORMAL" => Ok(SyncReturn(vec![42u8; 100])),
        "RESULT_ERR" => Err(anyhow!("deliberate error in handle_sync_return_err")),
        "PANIC" => panic!("deliberate panic in handle_sync_return_panic"),
        _ => panic!("unknown mode"),
    }
}

pub fn handle_stream(sink: StreamSink<String>, arg: String) -> Result<()> {
    println!("handle_stream arg={}", arg);

    let cnt = Arc::new(AtomicI32::new(0));

    // just to show that, you can send data to sink even in other threads
    let cnt2 = cnt.clone();
    let sink2 = sink.clone();
    thread::spawn(move || {
        for i in 0..5 {
            let old_cnt = cnt2.fetch_add(1, Ordering::Relaxed);
            let msg = format!("(thread=child, i={}, old_cnt={})", i, old_cnt);
            format!("send data to sink msg={}", msg);
            sink2.add(msg);
            thread::sleep(Duration::from_millis(100));
        }
        sink2.close();
    });

    for i in 0..5 {
        let old_cnt = cnt.fetch_add(1, Ordering::Relaxed);
        let msg = format!("(thread=normal, i={}, old_cnt={})", i, old_cnt);
        format!("send data to sink msg={}", msg);
        sink.add(msg);
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}

pub struct MyStreamEntry {
    pub hello: String,
}

// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub fn handle_stream_of_struct(sink: StreamSink<MyStreamEntry>) -> Result<()> {
    Ok(())
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
    pub int64list: Option<Vec<i64>>,
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
        int64list: manipulate_list(opt.int64list, 0),
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
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Note {
    pub day: Box<Weekdays>,
    pub body: String,
}

pub fn print_note(note: Note) -> ZeroCopyBuffer<Vec<u8>> {
    println!("{:#?}", note);
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
    println!("The weekday is {:?}", weekday);
    weekday
}

#[frb]
#[derive(Debug, Clone)]
pub struct Customized {
    pub final_field: String,
    #[frb(non_final)]
    pub non_final_field: Option<String>,
}

pub fn handle_customized_struct(val: Customized) {
    println!("{:#?}", val);
}

#[frb]
#[derive(Debug)]
pub enum KitchenSink {
    /// Comment on variant
    Empty,
    #[frb(unimpl_variant_attr)]
    Primitives {
        #[frb(unimpl_field_attr)]
        /// Dart field comment
        int32: i32,
        #[frb(unimpl_deprecated)]
        float64: f64,
        boolean: bool,
    },
    Nested(Box<KitchenSink>, i32),
    Optional(
        /// Comment on anonymous field
        Option<i32>,
        Option<i32>,
    ),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(Weekdays),
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
        Nested(_, val) => Nested(Box::new(Empty), val + 1),
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
};
use lazy_static::lazy_static;

// To mirror an external struct, you need to define a placeholder type with the same definition
#[frb(mirror(ApplicationSettings))]
pub struct _ApplicationSettings {
    pub name: String,
    pub version: String,
    pub mode: ApplicationMode,
    pub env: Box<ApplicationEnv>,
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

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
pub fn is_app_embedded(app_settings: ApplicationSettings) -> bool {
    // println!("env: {:?}", app_settings.env.vars);
    matches!(app_settings.mode, ApplicationMode::Embedded)
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
    pub value: u32,
}

pub fn next_user_id(user_id: UserId) -> UserId {
    UserId {
        value: user_id.value + 1,
    }
}

// event listener test
lazy_static! {
    static ref EVENT_LISTENER: Arc<Mutex<Option<StreamSink<Event>>>> = Default::default();
}

#[derive(Clone)]
pub struct Event {
    pub address: String,
    pub payload: String,
}

pub fn register_event_listener(listener: StreamSink<Event>) -> Result<()> {
    (*EVENT_LISTENER.lock().unwrap()) = Some(listener);
    Ok(())
}

pub fn close_event_listener() {
    if let Some(ref listener) = *EVENT_LISTENER.lock().unwrap() {
        listener.close();
    } else {
        return;
    }
    (*EVENT_LISTENER.lock().unwrap()) = None;
}

pub fn create_event() {
    if let Some(ref listener) = *EVENT_LISTENER.lock().unwrap() {
        listener.add(Event {
            address: "something".into(),
            payload: "payload".into(),
        });
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub key: u32,
    pub value: u32,
}

pub fn handle_stream_sink_at_1(
    key: u32,
    max: u32,
    sink: StreamSink<Log>,
) -> Result<(), anyhow::Error> {
    std::thread::spawn(move || {
        for i in 0..max {
            sink.add(Log { key, value: i });
        }
        sink.close();
    });
    Ok(())
}

pub fn handle_stream_sink_at_2(
    key: u32,
    sink: StreamSink<Log>,
    max: u32,
) -> Result<(), anyhow::Error> {
    handle_stream_sink_at_1(key, max, sink)
}

pub fn handle_stream_sink_at_3(
    sink: StreamSink<Log>,
    key: u32,
    max: u32,
) -> Result<(), anyhow::Error> {
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

pub struct ConcatenateWith {
    pub a: String,
}

#[derive(Debug, Clone)]
pub struct Log2 {
    pub key: u32,
    pub value: String,
}

impl ConcatenateWith {
    pub fn new(a: String) -> ConcatenateWith {
        ConcatenateWith { a }
    }
    pub fn concatenate(&self, b: String) -> String {
        format!("{}{}", self.a, b)
    }
    pub fn concatenate_static(a: String, b: String) -> String {
        format!("{}{}", a, b)
    }

    pub fn handle_some_stream_sink(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2>,
    ) -> Result<(), anyhow::Error> {
        let a = self.a.clone();
        std::thread::spawn(move || {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: format!("{}{}", a, i),
                });
            }
            sink.close();
        });
        Ok(())
    }

    pub fn handle_some_stream_sink_at_1(&self, sink: StreamSink<u32>) -> Result<(), anyhow::Error> {
        std::thread::spawn(move || {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
        Ok(())
    }

    pub fn handle_some_static_stream_sink(
        key: u32,
        max: u32,
        sink: StreamSink<Log2>,
    ) -> Result<(), anyhow::Error> {
        std::thread::spawn(move || {
            for i in 0..max {
                sink.add(Log2 {
                    key,
                    value: i.to_string(),
                });
            }
            sink.close();
        });
        Ok(())
    }

    pub fn handle_some_static_stream_sink_single_arg(
        sink: StreamSink<u32>,
    ) -> Result<(), anyhow::Error> {
        std::thread::spawn(move || {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        });
        Ok(())
    }
}
