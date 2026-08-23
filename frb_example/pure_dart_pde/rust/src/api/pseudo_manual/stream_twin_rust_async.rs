// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::anyhow;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

#[frb(stream_dart_await)]
pub async fn func_stream_return_error_twin_rust_async(
    _sink: StreamSink<String>,
) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

#[frb(stream_dart_await)]
pub async fn func_stream_return_panic_twin_rust_async(
    _sink: StreamSink<String>,
) -> anyhow::Result<()> {
    panic!("deliberate panic")
}

#[allow(unused_variables)]
pub async fn func_stream_sink_arg_position_twin_rust_async(a: u32, b: u32, c: StreamSink<u32>) {}

pub struct MyStreamEntryTwinRustAsync {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub async fn handle_stream_of_struct_twin_rust_async(
    _sink: StreamSink<MyStreamEntryTwinRustAsync>,
) {
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct LogTwinRustAsync {
    pub key: u32,
    pub value: u32,
}

pub async fn handle_stream_sink_at_1_twin_rust_async(
    key: u32,
    max: u32,
    sink: StreamSink<LogTwinRustAsync>,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

pub async fn handle_stream_sink_at_2_twin_rust_async(
    key: u32,
    sink: StreamSink<LogTwinRustAsync>,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

pub async fn handle_stream_sink_at_3_twin_rust_async(
    sink: StreamSink<LogTwinRustAsync>,
    key: u32,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

fn handle_stream_inner(key: u32, max: u32, sink: StreamSink<LogTwinRustAsync>) {
    for i in 0..max {
        sink.add(LogTwinRustAsync { key, value: i }).unwrap();
    }
}

pub async fn stream_sink_fixed_sized_primitive_array_twin_rust_async(sink: StreamSink<[u8; 2]>) {
    sink.add([1, 2]).unwrap();
    sink.add([3, 4]).unwrap();
}

pub async fn stream_sink_inside_vec_twin_rust_async(arg: Vec<StreamSink<i32>>) {
    for sink in arg {
        sink.add(100).unwrap();
        sink.add(200).unwrap();
    }
}

pub struct MyStructContainingStreamSinkTwinRustAsync {
    pub a: i32,
    pub b: StreamSink<i32>,
}

pub async fn stream_sink_inside_struct_twin_rust_async(
    arg: MyStructContainingStreamSinkTwinRustAsync,
) {
    arg.b.add(arg.a).unwrap();
}

pub async fn func_stream_add_value_and_error_twin_rust_async(sink: StreamSink<i32>) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
        sink.add(100).unwrap();
        sink.add(200).unwrap();
        sink.add_error(anyhow!("deliberate error")).unwrap();
    }));
}
