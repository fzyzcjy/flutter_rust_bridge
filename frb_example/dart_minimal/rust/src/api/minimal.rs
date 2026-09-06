use flutter_rust_bridge::frb;
use crate::frb_generated::{StreamSink, FLUTTER_RUST_BRIDGE_HANDLER};
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::transfer;
#[cfg(target_family = "wasm")]
use flutter_rust_bridge::for_generated::wasm_bindgen;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(sync)]
pub fn minimal_stream(sink: StreamSink<u32>) {
    FLUTTER_RUST_BRIDGE_HANDLER.thread_pool().execute(transfer!(|| {
        for value in 0..5 {
            sink.add(value).unwrap();
        }
    }));
}
