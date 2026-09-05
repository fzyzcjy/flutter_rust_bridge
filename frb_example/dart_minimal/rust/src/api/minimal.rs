use flutter_rust_bridge::frb;
use crate::frb_generated::StreamSink;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn immediate_stream(sink: StreamSink<i32>) {
    sink.add(0).unwrap();
    sink.add(1).unwrap();
}
