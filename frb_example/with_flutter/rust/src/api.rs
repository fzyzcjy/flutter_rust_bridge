use anyhow::Result;

use flutter_rust_bridge::ZeroCopyBuffer;

//
// NOTE: Please look at https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/rust/src/api.rs
// to see more types that this code generator can generate.
//

pub fn draw_mandelbrot(image_size: Size, zoom_point: Point, scale: f64, num_threads: i32) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    // Just an example that generates "complicated" images ;)
    let image = crate::off_topic_code::mandelbrot(image_size, zoom_point, scale, num_threads)?;
    Ok(ZeroCopyBuffer(image))
}

pub fn passing_complex_structs(root: TreeNode) -> Result<String> {
    Ok(format!("Hi this string is from Rust. I received a complex struct: {:?}", root))
}

pub fn work_on_big_array(input: Vec<u8>) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    // Just an example to detect memory problems in Flutter.
    let mut output = vec![0; input.len()];
    for i in 0..input.len() {
        output[i] = 255 - input[i];
    }
    Ok(ZeroCopyBuffer(output))
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

