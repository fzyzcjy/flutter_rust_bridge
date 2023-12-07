// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::anyhow;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{transfer, StreamSink};

pub fn func_stream_return_error_twin_normal(_sink: impl StreamSink<String>) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

pub fn func_stream_return_panic_twin_normal(_sink: impl StreamSink<String>) -> anyhow::Result<()> {
    panic!("deliberate panic")
}

#[allow(unused_variables)]
pub fn func_stream_sink_arg_position_twin_normal(a: u32, b: u32, c: impl StreamSink<u32>) {}

pub struct MyStreamEntryTwinNormal {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub fn handle_stream_of_struct_twin_normal(_sink: impl StreamSink<MyStreamEntryTwinNormal>) {
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct LogTwinNormal {
    pub key: u32,
    pub value: u32,
}

pub fn handle_stream_sink_at_1_twin_normal(
    key: u32,
    max: u32,
    sink: impl StreamSink<LogTwinNormal>,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

pub fn handle_stream_sink_at_2_twin_normal(
    key: u32,
    sink: impl StreamSink<LogTwinNormal>,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

pub fn handle_stream_sink_at_3_twin_normal(
    sink: impl StreamSink<LogTwinNormal>,
    key: u32,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

fn handle_stream_inner(key: u32, max: u32, sink: impl StreamSink<LogTwinNormal>) {
    for i in 0..max {
        let _ = sink.add(LogTwinNormal { key, value: i });
    }
    sink.close();
}
