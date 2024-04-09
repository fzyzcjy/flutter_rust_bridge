use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn hello_stream(sink: crate::frb_generated::StreamSink<i32>) {
    sink.add(100).unwrap();
    sink.add(200).unwrap();
}
