use anyhow::Result;

use flutter_rust_bridge::ZeroCopyBuffer;

//
// NOTE: Please look at https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/simple/rust/src/api.rs
// to see more types that this code generator can generate.
//

pub fn draw_mandelbrot(image_size: Size, right_bottom: Point, num_threads: i32) -> Result<ZeroCopyBuffer<Vec<u8>>> {
    // Just an example that generates "complicated" images ;)
    let image = crate::algorithms::mandelbrot(image_size, right_bottom, num_threads)?;
    Ok(ZeroCopyBuffer(image))
}

pub fn tree_traversal(root: TreeNode) -> Result<String> {
    // Just an example of running "complicated" algorithms ;)
    Ok(crate::algorithms::tree_preorder_traversal(root).join(","))
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

