use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn anyhow_test(a: StreamSink<Undurchsichtiger>) {
    todo!()
}

#[frb(opaque)]
pub struct Undurchsichtiger {
    a: std::fs::File,
}
impl Undurchsichtiger {
    pub fn read(&self) -> Vec<u8> {
        todo!()
    }
}
