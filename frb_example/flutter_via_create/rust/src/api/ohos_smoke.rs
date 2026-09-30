use crate::frb_generated::StreamSink;

#[derive(Clone)]
pub struct OhosSmokePayload {
    pub code: i32,
    pub label: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn reverse_bytes(value: Vec<u8>) -> Vec<u8> {
    value.into_iter().rev().collect()
}

#[flutter_rust_bridge::frb(sync)]
pub fn transform_payload(value: OhosSmokePayload) -> OhosSmokePayload {
    OhosSmokePayload {
        code: value.code + 1,
        label: format!("{}!", value.label),
    }
}

pub async fn async_greet(name: String) -> String {
    format!("Async hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)]
pub fn fallible_value(should_fail: bool) -> Result<String, String> {
    if should_fail {
        Err("expected smoke failure".to_owned())
    } else {
        Ok("smoke ok".to_owned())
    }
}

pub fn emit_values(sink: StreamSink<u32>) {
    let _ = sink.add(7);
    let _ = sink.add(11);
    let _ = sink.add(13);
}
