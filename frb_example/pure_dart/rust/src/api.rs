#![allow(unused_variables)]

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anyhow::{anyhow, Result};

use flutter_rust_bridge::Opaque;
use flutter_rust_bridge::{StreamSink, SyncReturn, ZeroCopyBuffer};

/// Documentation on a simple adder function.
pub fn simple_adder(a: i32, b: i32) -> Result<i32> {
    Ok(a + b)
}

/**
 Multiline comments are fine,
 but they are not preferred in Rust nor in Dart.

 Newlines are preserved.
*/
pub fn primitive_types(my_i32: i32, my_i64: i64, my_f64: f64, my_bool: bool) -> Result<i32> {
    println!(
        "primitive_types({}, {}, {}, {})",
        my_i32, my_i64, my_f64, my_bool
    );
    Ok(42)
}

pub fn primitive_u32(my_u32: u32) -> Result<u32> {
    println!("primitive_u32({})", my_u32);
    assert_eq!(my_u32, 0xff112233);
    let ret = 0xfe112233;
    println!("returning {}", ret);
    Ok(ret)
}

pub fn handle_string(s: String) -> Result<String> {
    println!("handle_string({})", &s);
    let s2 = s.clone();
    Ok(s + &s2)
}

pub fn handle_return_unit() -> Result<()> {
    println!("handle_return_unit()");
    Ok(())
}

// to check that `Vec<u8>` can be used as return type
pub fn handle_vec_u8(v: Vec<u8>) -> Result<Vec<u8>> {
    println!("handle_vec_u8(first few elements: {:?})", &v[..5]);
    Ok(v.repeat(2))
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

pub fn handle_vec_of_primitive(n: i32) -> Result<VecOfPrimitivePack> {
    Ok(VecOfPrimitivePack {
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

pub fn handle_zero_copy_vec_of_primitive(n: i32) -> Result<ZeroCopyVecOfPrimitivePack> {
    Ok(ZeroCopyVecOfPrimitivePack {
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

pub fn handle_struct(arg: MySize, boxed: Box<MySize>) -> Result<MySize> {
    println!("handle_struct({:?}, {:?})", &arg, &boxed);
    Ok(MySize {
        width: arg.width + boxed.width,
        height: arg.height + boxed.height,
    })
}

#[derive(Debug)]
pub struct NewTypeInt(pub i64);

pub fn handle_newtype(arg: NewTypeInt) -> Result<NewTypeInt> {
    println!("handle_newtype({:?})", &arg);
    Ok(NewTypeInt(arg.0 * 2))
}

pub fn handle_list_of_struct(mut l: Vec<MySize>) -> Result<Vec<MySize>> {
    println!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    Ok(ans)
}

pub fn handle_string_list(names: Vec<String>) -> Result<Vec<String>> {
    for name in &names {
        println!("Hello, {}", name);
    }
    Ok(names)
}

#[derive(Debug, Clone)]
pub struct MyTreeNode {
    pub value_i32: i32,
    pub value_vec_u8: Vec<u8>,
    pub children: Vec<MyTreeNode>,
}

pub fn handle_complex_struct(s: MyTreeNode) -> Result<MyTreeNode> {
    println!("handle_complex_struct({:?})", &s);
    let s_cloned = s.clone();
    Ok(s)
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

pub fn return_err() -> Result<i32> {
    Err(anyhow!(
        "return_err() is called, thus deliberately return Err"
    ))
}

pub fn return_panic() -> Result<i32> {
    panic!("return_panic() is called, thus deliberately panic")
}

pub fn handle_optional_return(left: f64, right: f64) -> Result<Option<f64>> {
    if right == 0. {
        Ok(None)
    } else {
        Ok(Some(left / right))
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

pub fn handle_optional_struct(document: Option<String>) -> Result<Option<Element>> {
    Ok(document.map(|inner| Element {
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
    }))
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

pub fn handle_optional_increment(opt: Option<ExoticOptionals>) -> Result<Option<ExoticOptionals>> {
    fn manipulate_list<T>(src: Option<Vec<T>>, push_value: T) -> Option<Vec<T>> {
        let mut list = src.unwrap_or_else(Vec::new);
        list.push(push_value);
        Some(list)
    }

    Ok(opt.map(|mut opt| ExoticOptionals {
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
            let mut list = opt.attributes.unwrap_or_else(Vec::new);
            list.push(Attribute {
                key: "some-attrib".to_owned(),
                value: "some-value".to_owned(),
            });
            list
        }),
        nullable_attributes: Some({
            let mut list = opt.nullable_attributes.unwrap_or_else(Vec::new);
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
    }))
}

pub fn handle_increment_boxed_optional(opt: Option<Box<f64>>) -> Result<f64> {
    match opt {
        Some(e) => Ok(*e + 1.),
        None => Ok(42.),
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
) -> Result<String> {
    Ok(format!(
        "handle_option_box_arguments({:?})",
        (i8box, u8box, i32box, i64box, f64box, boolbox, structbox)
    ))
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

pub fn handle_return_enum(input: String) -> Result<Option<Weekdays>> {
    Ok(match input.as_str() {
        "Monday" => Some(Weekdays::Monday),
        "Tuesday" => Some(Weekdays::Tuesday),
        "Wednesday" => Some(Weekdays::Wednesday),
        "Thursday" => Some(Weekdays::Thursday),
        "Friday" => Some(Weekdays::Friday),
        "Saturday" => Some(Weekdays::Saturday),
        "Sunday" => Some(Weekdays::Sunday),
        _ => None,
    })
}

pub fn handle_enum_parameter(weekday: Weekdays) -> Result<Weekdays> {
    println!("The weekday is {:?}", weekday);
    Ok(weekday)
}

pub enum Foobar {
    Foo,
    Bar(String),
    Baz { name: String },
}

#[derive(Debug)]
pub enum KitchenSink {
    Empty,
    Nested(Box<KitchenSink>),
    Optional(i32, Option<i32>),
    Boxed(Box<i32>),
    Buffer(ZeroCopyBuffer<Vec<u8>>),
    Enums(Weekdays),
}

pub fn handle_enum_struct(mut val: Foobar) -> Result<Foobar> {
    if let Foobar::Bar(val) = &mut val {
        *val = "foo'd".to_owned()
    }
    Ok(val)
}

pub fn handle_complex_enum(val: KitchenSink) -> Result<String> {
    Ok(format!("{:?}", val))
}
