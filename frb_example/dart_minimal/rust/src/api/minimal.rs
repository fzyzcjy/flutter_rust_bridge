use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub struct BMPimage {
    pub width: u32,
    pub height: u32,
    pub bmp: Vec<u8>,
}

pub fn render_image(width: i64, height: i64) -> BMPimage {
    let tt = std::time::Instant::now();
    let res = BMPimage {
        width: width as u32,
        height: height as u32,
        bmp: vec![0; (width * height * 4) as usize],
    };
    println!("IN RUST: {:?}", tt.elapsed());
    res
}
