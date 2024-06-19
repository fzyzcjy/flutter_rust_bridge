use bigdecimal::BigDecimal;
use num_rational::Ratio;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Clone, Debug)]
pub enum SpecificMetadata {
    Image {
        width: u32,
        height: u32,
        pixel_aspect: Ratio<u32>,
    },
    Video {
        width: u32,
        height: u32,
        pixel_aspect: Ratio<u32>,
        number_of_frames: usize,
        frames_per_second: BigDecimal,
    },
}

pub fn my_func(a: SpecificMetadata) -> SpecificMetadata {
    a
}
