// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use anyhow::anyhow;
use flutter_rust_bridge::{spawn, StreamSink};
use log::info;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub async fn func_stream_realistic_twin_rust_async(sink: StreamSink<String>, arg: String) {
    info!("handle_stream_realistic arg={}", arg);

    let cnt = Arc::new(AtomicI32::new(0));

    // just to show that, you can send data to sink even in other threads
    let cnt2 = cnt.clone();
    let sink2 = sink.clone();
    spawn!(|| {
        for i in 0..5 {
            let old_cnt = cnt2.fetch_add(1, Ordering::Relaxed);
            let msg = format!("(thread=child, i={i}, old_cnt={old_cnt})");
            format!("send data to sink msg={msg}");
            let _ = sink2.add(msg);
            sleep(Duration::from_millis(100));
        }
        sink2.close();
    });

    for i in 0..5 {
        let old_cnt = cnt.fetch_add(1, Ordering::Relaxed);
        let msg = format!("(thread=normal, i={i}, old_cnt={old_cnt})");
        format!("send data to sink msg={msg}");
        let _ = sink.add(msg);
        sleep(Duration::from_millis(50));
    }
}

pub async fn func_stream_return_error_twin_rust_async(
    _sink: StreamSink<String>,
) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

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
    spawn!(|| { handle_stream_inner(key, max, sink) });
}

pub async fn handle_stream_sink_at_2_twin_rust_async(
    key: u32,
    sink: StreamSink<LogTwinRustAsync>,
    max: u32,
) {
    spawn!(|| { handle_stream_inner(key, max, sink) });
}

pub async fn handle_stream_sink_at_3_twin_rust_async(
    sink: StreamSink<LogTwinRustAsync>,
    key: u32,
    max: u32,
) {
    spawn!(|| { handle_stream_inner(key, max, sink) });
}

fn handle_stream_inner(key: u32, max: u32, sink: StreamSink<LogTwinRustAsync>) {
    for i in 0..max {
        let _ = sink.add(LogTwinRustAsync { key, value: i });
    }
    sink.close();
}
