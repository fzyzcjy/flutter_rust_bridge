//! NOTE: This file is **unrelated** to the main topic of our example.
//! Only for generating beautiful image.
//! Mandelbrot is copied and modified from
//! https://github.com/ProgrammingRust/mandelbrot/blob/task-queue/src/main.rs and
//! https://github.com/Ducolnd/rust-mandelbrot/blob/master/src/main.rs

use crate::api::mandelbrot::{Point, Size};
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::*;
use flutter_rust_bridge::for_generated::futures::future::try_join_all;
use flutter_rust_bridge::spawn_blocking_with;
use image::codecs::png::PngEncoder;
use image::*;
use num::Complex;

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the
/// iteration limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut ans = None;
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            // NOTE: Normal mandelbrot should NOT compute like this,
            // but should immediately return once find the answer.
            // However, here we just want to let Rust compute something heavy
            // and have *predictable* time. Otherwise, different images are generated
            // with different computation time, and thus the user will be confused why
            // their changed parameters does not reflect change of computation time (because
            // indeed the computation is influenced by the specific image).
            if ans.is_none() {
                ans = Some(i);
            }
        }
        z = z * z + c;
    }
    ans
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 },
        ),
        Complex {
            re: -0.5,
            im: -0.75,
        }
    );
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn colorize(grey_pixels: &[u8]) -> Vec<u8> {
    let mut ans = vec![0u8; grey_pixels.len() * 3];
    for i in 0..grey_pixels.len() {
        let (r, g, b) = colorize_pixel(grey_pixels[i]);
        ans[i * 3] = r;
        ans[i * 3 + 1] = g;
        ans[i * 3 + 2] = b;
    }
    ans
}

const A: f64 = 1.0 * (1.0 / std::f64::consts::LOG2_10);
const B: f64 = (1.0 / (3.0 * std::f64::consts::SQRT_2)) * (1.0 / std::f64::consts::LOG2_10);

pub fn colorize_pixel(it: u8) -> (u8, u8, u8) {
    if it == 0 {
        return (0, 0, 0);
    }
    let it = it as f64;

    let c: f64 = (1.0_f64 / ((7.0 * 3.0_f64).powf(1.0 / 8.0))) * (1.0 / std::f64::consts::LOG2_10);

    let r = 255.0 * ((1.0 - (A * it).cos()) / 2.0);
    let g = 255.0 * ((1.0 - (B * it).cos()) / 2.0);
    let b = 255.0 * ((1.0 - (c * it).cos()) / 2.0);

    // print!(" {:?} ", [r, g, b]);

    (r as u8, b as u8, g as u8)
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(pixels: &[u8], bounds: (usize, usize)) -> Result<Vec<u8>> {
    let mut buf = Vec::new();

    let encoder = PngEncoder::new(&mut buf);
    #[allow(deprecated)]
    encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8)?;

    Ok(buf)
}

pub async fn mandelbrot(
    image_size: Size,
    zoom_point: Point,
    scale: f64,
    num_threads: i32,
) -> Result<Vec<u8>> {
    let num_threads = num_threads as usize;
    let bounds = (image_size.width as usize, image_size.height as usize);
    let upper_left = Complex::new(zoom_point.x - scale, zoom_point.y - scale);
    let lower_right = Complex::new(zoom_point.x + scale, zoom_point.y + scale);

    let band_rows = bounds.1 / num_threads;

    let mut join_handles = vec![];
    for i in 0..num_threads {
        join_handles.push(spawn_blocking_with(
            move || {
                let top = band_rows * i;
                let bottom = usize::min(band_rows * (i + 1), bounds.1);
                let height = bottom - top;
                let mut band = vec![0; bounds.0 * height];

                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                render(&mut band, band_bounds, band_upper_left, band_lower_right);

                band
            },
            FLUTTER_RUST_BRIDGE_HANDLER.thread_pool(),
        ));
    }

    let bands_arr = try_join_all(join_handles).await?;
    let pixels = bands_arr.concat();

    write_image(&colorize(&pixels), bounds)
}

// pub fn tree_preorder_traversal(root: TreeNode) -> Vec<String> {
//     let mut ans = Vec::new();
//     tree_preorder_traversal_core(root, &mut ans);
//     ans
// }
//
// fn tree_preorder_traversal_core(node: TreeNode, dst: &mut Vec<String>) {
//     dst.push(node.name);
//     for child in node.children {
//         tree_preorder_traversal_core(child, dst);
//     }
// }
