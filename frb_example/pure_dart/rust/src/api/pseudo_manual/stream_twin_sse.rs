// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::anyhow;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::transfer;

#[flutter_rust_bridge::frb(serialize)]
pub fn func_stream_return_error_twin_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_stream_return_panic_twin_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> anyhow::Result<()> {
    panic!("deliberate panic")
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(serialize)]
pub fn func_stream_sink_arg_position_twin_sse(
    a: u32,
    b: u32,
    c: StreamSink<u32, flutter_rust_bridge::SseCodec>,
) {
}

pub struct MyStreamEntryTwinSse {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
#[flutter_rust_bridge::frb(serialize)]
pub fn handle_stream_of_struct_twin_sse(
    _sink: StreamSink<MyStreamEntryTwinSse, flutter_rust_bridge::SseCodec>,
) {
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct LogTwinSse {
    pub key: u32,
    pub value: u32,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_stream_sink_at_1_twin_sse(
    key: u32,
    max: u32,
    sink: StreamSink<LogTwinSse, flutter_rust_bridge::SseCodec>,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_stream_sink_at_2_twin_sse(
    key: u32,
    sink: StreamSink<LogTwinSse, flutter_rust_bridge::SseCodec>,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_stream_sink_at_3_twin_sse(
    sink: StreamSink<LogTwinSse, flutter_rust_bridge::SseCodec>,
    key: u32,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

fn handle_stream_inner(
    key: u32,
    max: u32,
    sink: StreamSink<LogTwinSse, flutter_rust_bridge::SseCodec>,
) {
    for i in 0..max {
        sink.add(LogTwinSse { key, value: i }).unwrap();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn stream_sink_fixed_sized_primitive_array_twin_sse(
    sink: StreamSink<[u8; 2], flutter_rust_bridge::SseCodec>,
) {
    sink.add([1, 2]).unwrap();
    sink.add([3, 4]).unwrap();
}
