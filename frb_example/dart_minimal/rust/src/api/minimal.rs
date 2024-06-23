use std::sync::Mutex;
use flutter_rust_bridge::for_generated::{anyhow, lazy_static};
use flutter_rust_bridge::for_generated::anyhow::anyhow;
use flutter_rust_bridge::frb;
use crate::frb_generated::StreamSink;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

lazy_static! {
    static ref TEST_EVENT_STREAM: Mutex<Option<StreamSink<TestDevice>>> = Default::default();
}

pub fn test_event_stream(listener: StreamSink<TestDevice>) -> anyhow::Result<()> {
    match TEST_EVENT_STREAM.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

fn push_test(device: TestDevice) {
    if let Ok(mut guard) = TEST_EVENT_STREAM.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(device).unwrap(); // <-- error happens here
        }
    }
}

pub struct TestDevice {
    pub values: Vec<u64>,
}
