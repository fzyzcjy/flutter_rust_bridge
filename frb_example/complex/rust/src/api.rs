use anyhow::Result;

use flutter_rust_bridge::ZeroCopyBuffer;

//
// NOTE: Please look at https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/rust/src/api.rs
// to see more types that this code generator can generate.
//

pub fn draw_mandelbrot(image_size: Size, zoom_point: Point, scale: f64, num_threads: i32) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    // Just an example that generates "complicated" images ;)
    let image = crate::algorithms::mandelbrot(image_size, zoom_point, scale, num_threads)?;
    Ok(ZeroCopyBuffer(image))
}

pub fn passing_complex_structs(root: TreeNode) -> Result<String> {
    Ok(format!("Hi this string is from Rust. I received a complex struct: {:?}", root))
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

