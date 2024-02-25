use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Reproduce #1778
#[frb(opaque)]
pub struct Mqtt;

impl Mqtt {
    #[frb(sync)]
    pub fn new() -> Mqtt {
        Mqtt
    }

    pub fn send(&self) {}

    pub fn connect(&self, sink: StreamSink<i32>) {
        for i in 0..10 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let _ = sink.add(i);
        }
    }
}
