use crate::frb_generated::StreamSink;
use flutter_rust_bridge::{frb, DartFnFuture};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

pub async fn smoke_async(input: u32) -> u32 {
    input + 1
}

pub fn smoke_stream(count: u32, sink: StreamSink<u32>) {
    for value in 0..count {
        sink.add(value).unwrap();
    }
}

pub async fn smoke_callback(callback: impl Fn(String) -> DartFnFuture<String>) -> String {
    callback("rust".to_owned()).await
}

#[frb(opaque)]
pub struct SmokeCounter {
    value: u32,
}

pub fn smoke_counter_create(initial: u32) -> SmokeCounter {
    SmokeCounter { value: initial }
}

impl SmokeCounter {
    pub fn add(&mut self, delta: u32) -> u32 {
        self.value += delta;
        self.value
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

pub fn smoke_error(should_fail: bool) -> anyhow::Result<String> {
    if should_fail {
        anyhow::bail!("deliberate OHOS smoke error");
    }
    Ok("ok".to_owned())
}

pub struct SmokePayload {
    pub label: String,
    pub bytes: Vec<u8>,
    pub state: SmokeState,
}

pub enum SmokeState {
    Ready,
}

pub fn smoke_payload(size: u32) -> SmokePayload {
    SmokePayload {
        label: "payload".to_owned(),
        bytes: (0..size).map(|value| (value % 251) as u8).collect(),
        state: SmokeState::Ready,
    }
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
