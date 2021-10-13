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

pub fn passing_complex_structs(root: TreeNode) -> Result<String> {
    Ok(format!(
        "Hi this string is from Rust. I received a complex struct: {:?}",
        root
    ))
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

// following are used only for memory tests. Readers of this example do not need to consider it.

pub fn off_topic_memory_test_input_array(input: Vec<u8>) -> Result<i32> {
    Ok(input.len() as i32)
}

pub fn off_topic_memory_test_output_zero_copy_buffer(len: i32) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    Ok(ZeroCopyBuffer(vec![0u8; len as usize]))
}

pub fn off_topic_memory_test_output_vec_u8(len: i32) -> Result<Vec<u8>> {
    Ok(vec![0u8; len as usize])
}

pub fn off_topic_memory_test_input_vec_of_object(input: Vec<Size>) -> Result<i32> {
    Ok(input.len() as i32)
}

pub fn off_topic_memory_test_output_vec_of_object(len: i32) -> Result<Vec<Size>> {
    let item = Size {
        width: 42,
        height: 42,
    };
    Ok(vec![item; len as usize])
}

pub fn off_topic_memory_test_input_complex_struct(input: TreeNode) -> Result<i32> {
    Ok(input.children.len() as i32)
}

pub fn off_topic_memory_test_output_complex_struct(len: i32) -> Result<TreeNode> {
    let child = TreeNode {
        name: "child".to_string(),
        children: Vec::new(),
    };
    Ok(TreeNode {
        name: "root".to_string(),
        children: vec![child; len as usize],
    })
}

pub fn off_topic_deliberately_return_error() -> Result<i32> {
    std::env::set_var("RUST_BACKTRACE", "1"); // optional, just to see more info...
    Err(anyhow!("deliberately return Error!"))
}

pub fn off_topic_deliberately_panic() -> Result<i32> {
    std::env::set_var("RUST_BACKTRACE", "1"); // optional, just to see more info...
    panic!("deliberately panic!")
}
