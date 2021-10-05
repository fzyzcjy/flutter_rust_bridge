use std::io::Error;
use std::sync::Mutex;

use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;

///! NOTE: This file is **unrelated** to the main topic of our example.
///! Only for generating beautiful image.
///! Mandelbrot is copied and modified from
///! https://github.com/ProgrammingRust/mandelbrot/blob/task-queue/src/main.rs and
///! https://github.com/Ducolnd/rust-mandelbrot/blob/master/src/main.rs

use crate::api::*;

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
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(bounds: (usize, usize),
                  pixel: (usize, usize),
                  upper_left: Complex<f64>,
                  lower_right: Complex<f64>)
                  -> Complex<f64>
{
    let (width, height) = (lower_right.re - upper_left.re,
                           upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        // Why subtraction here? pixel.1 increases as we go down,
        // but the imaginary component increases as we go up.
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100, 200), (25, 175),
                              Complex { re: -1.0, im: 1.0 },
                              Complex { re: 1.0, im: -1.0 }),
               Complex { re: -0.5, im: -0.75 });
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-
/// left and lower-right corners of the pixel buffer.
fn render(pixels: &mut [u8],
          bounds: (usize, usize),
          upper_left: Complex<f64>,
          lower_right: Complex<f64>)
{
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
                                       upper_left, lower_right);
            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
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

// color comes from:
/*
import numpy as np; from matplotlib import cm;
x = np.linspace(0,1,256); y = (cm.summer(x)[:, :3] * 255).astype(np.uint8);
s = '[' + ','.join('(%d,%d,%d)' % tuple(it) for it in y) + ']';
import pyperclip; pyperclip.copy(s)
*/
const COLORMAP: [(u8, u8, u8); 256] = [(0, 127, 102), (1, 128, 102), (2, 128, 102), (3, 129, 102), (4, 129, 102), (5, 130, 102), (6, 130, 102), (7, 131, 102), (8, 131, 102), (9, 132, 102), (10, 132, 102), (11, 133, 102), (12, 133, 102), (13, 134, 102), (14, 134, 102), (15, 135, 102), (16, 135, 102), (17, 136, 102), (18, 136, 102), (19, 137, 102), (20, 137, 102), (21, 138, 102), (22, 138, 102), (23, 139, 102), (24, 139, 102), (25, 140, 102), (26, 140, 102), (27, 141, 102), (28, 141, 102), (29, 142, 102), (30, 142, 102), (31, 143, 102), (32, 143, 102), (32, 144, 102), (34, 144, 102), (35, 145, 102), (36, 145, 102), (36, 146, 102), (38, 146, 102), (39, 147, 102), (40, 147, 102), (40, 147, 102), (42, 148, 102), (43, 149, 102), (44, 149, 102), (44, 150, 102), (46, 150, 102), (47, 151, 102), (48, 151, 102), (48, 152, 102), (50, 152, 102), (51, 153, 102), (52, 153, 102), (52, 154, 102), (54, 154, 102), (55, 155, 102), (56, 155, 102), (56, 156, 102), (58, 156, 102), (59, 157, 102), (60, 157, 102), (60, 158, 102), (62, 158, 102), (63, 159, 102), (64, 159, 102), (65, 160, 102), (65, 160, 102), (67, 161, 102), (68, 161, 102), (69, 162, 102), (70, 162, 102), (71, 163, 102), (72, 163, 102), (73, 163, 102), (73, 164, 102), (75, 165, 102), (76, 165, 102), (77, 166, 102), (78, 166, 102), (79, 167, 102), (80, 167, 102), (81, 168, 102), (81, 168, 102), (83, 169, 102), (84, 169, 102), (85, 170, 102), (86, 170, 102), (87, 171, 102), (88, 171, 102), (89, 172, 102), (89, 172, 102), (91, 173, 102), (92, 173, 102), (93, 174, 102), (94, 174, 102), (95, 175, 102), (96, 175, 102), (97, 176, 102), (97, 176, 102), (99, 177, 102), (100, 177, 102), (101, 178, 102), (102, 178, 102), (103, 179, 102), (104, 179, 102), (105, 179, 102), (105, 180, 102), (107, 181, 102), (108, 181, 102), (109, 182, 102), (110, 182, 102), (111, 183, 102), (112, 183, 102), (113, 184, 102), (113, 184, 102), (115, 185, 102), (116, 185, 102), (117, 186, 102), (118, 186, 102), (119, 187, 102), (120, 187, 102), (121, 188, 102), (121, 188, 102), (123, 189, 102), (124, 189, 102), (125, 190, 102), (126, 190, 102), (127, 191, 102), (128, 191, 102), (129, 192, 102), (130, 192, 102), (131, 193, 102), (131, 193, 102), (133, 194, 102), (134, 194, 102), (135, 195, 102), (136, 195, 102), (137, 195, 102), (138, 196, 102), (139, 196, 102), (140, 197, 102), (141, 198, 102), (142, 198, 102), (143, 199, 102), (144, 199, 102), (145, 200, 102), (146, 200, 102), (147, 201, 102), (147, 201, 102), (149, 202, 102), (150, 202, 102), (151, 203, 102), (152, 203, 102), (153, 204, 102), (154, 204, 102), (155, 205, 102), (156, 205, 102), (157, 206, 102), (158, 206, 102), (159, 207, 102), (160, 207, 102), (161, 208, 102), (162, 208, 102), (163, 209, 102), (163, 209, 102), (165, 210, 102), (166, 210, 102), (167, 211, 102), (168, 211, 102), (169, 211, 102), (170, 212, 102), (171, 212, 102), (172, 213, 102), (173, 214, 102), (174, 214, 102), (175, 215, 102), (176, 215, 102), (177, 216, 102), (178, 216, 102), (179, 217, 102), (179, 217, 102), (181, 218, 102), (182, 218, 102), (183, 219, 102), (184, 219, 102), (185, 220, 102), (186, 220, 102), (187, 221, 102), (188, 221, 102), (189, 222, 102), (190, 222, 102), (191, 223, 102), (192, 223, 102), (193, 224, 102), (194, 224, 102), (195, 225, 102), (195, 225, 102), (197, 226, 102), (198, 226, 102), (199, 227, 102), (200, 227, 102), (201, 227, 102), (202, 228, 102), (203, 228, 102), (204, 229, 102), (205, 230, 102), (206, 230, 102), (207, 231, 102), (208, 231, 102), (209, 232, 102), (210, 232, 102), (211, 233, 102), (211, 233, 102), (213, 234, 102), (214, 234, 102), (215, 235, 102), (216, 235, 102), (217, 236, 102), (218, 236, 102), (219, 237, 102), (220, 237, 102), (221, 238, 102), (222, 238, 102), (223, 239, 102), (224, 239, 102), (225, 240, 102), (226, 240, 102), (227, 241, 102), (227, 241, 102), (229, 242, 102), (230, 242, 102), (231, 243, 102), (232, 243, 102), (233, 243, 102), (234, 244, 102), (235, 244, 102), (236, 245, 102), (237, 246, 102), (238, 246, 102), (239, 247, 102), (240, 247, 102), (241, 248, 102), (242, 248, 102), (243, 249, 102), (243, 249, 102), (245, 250, 102), (246, 250, 102), (247, 251, 102), (248, 251, 102), (249, 252, 102), (250, 252, 102), (251, 253, 102), (252, 253, 102), (253, 254, 102), (254, 254, 102), (255, 255, 102)];

pub fn colorize_pixel(it: u8) -> (u8, u8, u8) {
    if it == 0 { (0, 0, 0) } else { COLORMAP[it as usize] }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(pixels: &[u8], bounds: (usize, usize)) -> Result<Vec<u8>, std::io::Error> {
    let mut buf = Vec::new();

    let encoder = PNGEncoder::new(&mut buf);
    encoder.encode(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::RGB(8))?;

    Ok(buf)
}

pub fn mandelbrot(image_size: Size, zoom_point: Point, scale: f64, num_threads: i32) -> Result<Vec<u8>, Error> {
    let bounds = (image_size.width as usize, image_size.height as usize);
    let upper_left = Complex::new(zoom_point.x - scale, zoom_point.y - scale);
    let lower_right = Complex::new(zoom_point.x + scale, zoom_point.y + scale);

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let band_rows = bounds.1 / (num_threads as usize) + 1;

    {
        let bands = Mutex::new(pixels.chunks_mut(band_rows * bounds.0).enumerate());
        crossbeam::scope(|scope| {
            for _ in 0..num_threads {
                scope.spawn(|_| {
                    loop {
                        match {
                            let mut guard = bands.lock().unwrap();
                            guard.next()
                        }
                        {
                            None => { return; }
                            Some((i, band)) => {
                                let top = band_rows * i;
                                let height = band.len() / bounds.0;
                                let band_bounds = (bounds.0, height);
                                let band_upper_left = pixel_to_point(bounds, (0, top),
                                                                     upper_left, lower_right);
                                let band_lower_right = pixel_to_point(bounds, (bounds.0, top + height),
                                                                      upper_left, lower_right);
                                render(band, band_bounds, band_upper_left, band_lower_right);
                            }
                        }
                    }
                });
            }
        }).unwrap();
    }

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