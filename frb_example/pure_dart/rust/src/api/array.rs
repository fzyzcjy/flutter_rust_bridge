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
