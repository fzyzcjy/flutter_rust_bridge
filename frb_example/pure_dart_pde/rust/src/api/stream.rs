// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::anyhow;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

#[frb(stream_dart_await)]
pub fn func_stream_return_error_twin_normal(_sink: StreamSink<String>) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

#[frb(stream_dart_await)]
pub fn func_stream_return_panic_twin_normal(_sink: StreamSink<String>) -> anyhow::Result<()> {
    panic!("deliberate panic")
}

#[allow(unused_variables)]
pub fn func_stream_sink_arg_position_twin_normal(a: u32, b: u32, c: StreamSink<u32>) {}

pub struct MyStreamEntryTwinNormal {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub fn handle_stream_of_struct_twin_normal(_sink: StreamSink<MyStreamEntryTwinNormal>) {
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct LogTwinNormal {
    pub key: u32,
    pub value: u32,
}

pub fn handle_stream_sink_at_1_twin_normal(key: u32, max: u32, sink: StreamSink<LogTwinNormal>) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

pub fn handle_stream_sink_at_2_twin_normal(key: u32, sink: StreamSink<LogTwinNormal>, max: u32) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

pub fn handle_stream_sink_at_3_twin_normal(sink: StreamSink<LogTwinNormal>, key: u32, max: u32) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

fn handle_stream_inner(key: u32, max: u32, sink: StreamSink<LogTwinNormal>) {
    for i in 0..max {
        sink.add(LogTwinNormal { key, value: i }).unwrap();
    }
}

pub fn stream_sink_fixed_sized_primitive_array_twin_normal(sink: StreamSink<[u8; 2]>) {
    sink.add([1, 2]).unwrap();
    sink.add([3, 4]).unwrap();
}

pub fn stream_sink_inside_vec_twin_normal(arg: Vec<StreamSink<i32>>) {
    for sink in arg {
        sink.add(100).unwrap();
        sink.add(200).unwrap();
    }
}

pub struct MyStructContainingStreamSinkTwinNormal {
    pub a: i32,
    pub b: StreamSink<i32>,
}

pub fn stream_sink_inside_struct_twin_normal(arg: MyStructContainingStreamSinkTwinNormal) {
    arg.b.add(arg.a).unwrap();
}

pub fn func_stream_add_value_and_error_twin_normal(sink: StreamSink<i32>) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
        sink.add(100).unwrap();
        sink.add(200).unwrap();
        sink.add_error(anyhow!("deliberate error")).unwrap();
    }));
}

pub fn stream_sink_emit_range_twin_normal(sink: StreamSink<i32>, count: u32) {
    for i in 0..count {
        sink.add(i as i32).unwrap();
    }
}

pub fn stream_sink_emit_range_then_hold_twin_normal(
    sink: StreamSink<i32>,
    count: u32,
    hold_millis: u64,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
        for i in 0..count {
            if sink.add(i as i32).is_err() {
                return;
            }
        }
        sleep(Duration::from_millis(hold_millis));
    }));
}

pub fn stream_sink_emit_many_twin_normal(sink: StreamSink<i32>, count: u32) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
        for i in 0..count {
            if sink.add(i as i32).is_err() {
                break;
            }
        }
    }));
}

lazy_static! {
    static ref STORED_STREAM_SINK_TWIN_NORMAL: Mutex<Option<StreamSink<i32>>> = Default::default();
}

pub fn store_stream_sink_twin_normal(sink: StreamSink<i32>) {
    if let Ok(mut guard) = STORED_STREAM_SINK_TWIN_NORMAL.lock() {
        *guard = Some(sink);
    }
}

pub fn clear_stored_stream_sink_twin_normal() {
    let _ = STORED_STREAM_SINK_TWIN_NORMAL
        .lock()
        .map(|mut guard| guard.take());
}

pub fn stored_stream_sink_emit_many_twin_normal(count: u32) {
    if let Ok(mut guard) = STORED_STREAM_SINK_TWIN_NORMAL.lock() {
        if let Some(sink) = guard.as_mut() {
            for i in 0..count {
                if sink.add(i as i32).is_err() {
                    break;
                }
            }
        }
    }
}

pub fn stored_stream_sink_start_spam_twin_normal(total: u32, interval_millis: u64) {
    let sink = STORED_STREAM_SINK_TWIN_NORMAL
        .lock()
        .ok()
        .and_then(|guard| guard.as_ref().cloned());
    if let Some(sink) = sink {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..total {
                if sink.add(i as i32).is_err() {
                    break;
                }
                if interval_millis > 0 {
                    sleep(Duration::from_millis(interval_millis));
                }
            }
        }));
    }
}

pub fn stored_stream_sink_emit_error_twin_normal(message: String) {
    if let Ok(mut guard) = STORED_STREAM_SINK_TWIN_NORMAL.lock() {
        if let Some(sink) = guard.as_mut() {
            let _ = sink.add_error(anyhow!(message));
        }
    }
}
