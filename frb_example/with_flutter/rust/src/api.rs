use anyhow::{anyhow, Result};

use flutter_rust_bridge::ZeroCopyBuffer;

//
// NOTE: Please look at https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/rust/src/api.rs
// to see more types that this code generator can generate.
//

pub fn draw_mandelbrot(
    image_size: Size,
    zoom_point: Point,
    scale: f64,
    num_threads: i32,
) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    // Just an example that generates "complicated" images ;)
    let image = crate::off_topic_code::mandelbrot(image_size, zoom_point, scale, num_threads)?;
    Ok(ZeroCopyBuffer(image))
}

pub fn passing_complex_structs(root: TreeNode) -> String {
    format!(
        "Hi this string is from Rust. I received a complex struct: {:?}",
        root
    )
}

pub fn returning_structs_with_boxed_fields() -> BoxedPoint {
    BoxedPoint {
        point: Box::new(Point { x: 0.0, y: 0.0 }),
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone)]
pub struct BoxedPoint {
    pub point: Box<Point>,
}

// following are used only for memory tests. Readers of this example do not need to consider it.

pub fn off_topic_memory_test_input_array(input: Vec<u8>) -> i32 {
    input.len() as i32
}

pub fn off_topic_memory_test_output_zero_copy_buffer(len: i32) -> ZeroCopyBuffer<Vec<u8>> {
    ZeroCopyBuffer(vec![0u8; len as usize])
}

pub fn off_topic_memory_test_output_vec_u8(len: i32) -> Vec<u8> {
    vec![0u8; len as usize]
}

pub fn off_topic_memory_test_input_vec_of_object(input: Vec<Size>) -> i32 {
    input.len() as i32
}

pub fn off_topic_memory_test_output_vec_of_object(len: i32) -> Vec<Size> {
    let item = Size {
        width: 42,
        height: 42,
    };
    vec![item; len as usize]
}

pub fn off_topic_memory_test_input_complex_struct(input: TreeNode) -> i32 {
    input.children.len() as i32
}

pub fn off_topic_memory_test_output_complex_struct(len: i32) -> TreeNode {
    let child = TreeNode {
        name: "child".to_string(),
        children: Vec::new(),
    };
    TreeNode {
        name: "root".to_string(),
        children: vec![child; len as usize],
    }
}

pub fn off_topic_deliberately_return_error() -> Result<i32> {
    #[cfg(not(target_family = "wasm"))]
    std::env::set_var("RUST_BACKTRACE", "1"); // optional, just to see more info...
    Err(anyhow!("deliberately return Error!"))
}

pub fn off_topic_deliberately_panic() -> i32 {
    #[cfg(not(target_family = "wasm"))]
    std::env::set_var("RUST_BACKTRACE", "1"); // optional, just to see more info...
    panic!("deliberately panic!")
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
