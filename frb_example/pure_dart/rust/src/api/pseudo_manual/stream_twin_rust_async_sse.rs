// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::anyhow;
use flutter_rust_bridge::for_generated::BaseThreadPool;
#[cfg(target_family = "wasm")]
use flutter_rust_bridge::for_generated::{js_sys, TransferClosure};
use flutter_rust_bridge::{frb, transfer};

#[frb(stream_dart_await)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn func_stream_return_error_twin_rust_async_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn immediate_stream_twin_rust_async_sse(
    sink: StreamSink<i32, flutter_rust_bridge::SseCodec>,
) {
    sink.add(0).unwrap();
    sink.add(1).unwrap();
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn stream_worker_transfer_twin_rust_async_sse(
    sink: StreamSink<i32, flutter_rust_bridge::SseCodec>,
) {
    #[cfg(target_family = "wasm")]
    {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let rejected = FLUTTER_RUST_BRIDGE_HANDLER.thread_pool().with(|pool| {
            pool.execute(TransferClosure::new(
                vec![js_sys::Function::new_no_args("return 0").into()],
                vec![],
                |_| {},
            ))
            .is_err()
        });
        let values = js_sys::Array::of1(&7.into());
        let bytes = js_sys::Uint8Array::new_with_length(1);
        bytes.set_index(0, 11);
        let buffer = bytes.buffer();
        let sink_in_worker = sink.clone();
        let finished = Arc::new(AtomicBool::new(false));
        let finished_in_worker = finished.clone();
        FLUTTER_RUST_BRIDGE_HANDLER.thread_pool().with(|pool| {
            pool.execute(TransferClosure::new(
                vec![values.clone().into(), buffer.clone().into()],
                vec![buffer.clone().into()],
                move |data| {
                    let original_value =
                        js_sys::Array::from(&data[0]).get(0).as_f64().unwrap() as i32;
                    let original_byte = js_sys::Uint8Array::new(&data[1]).get_index(0) as i32;
                    for value in [i32::from(rejected), original_value, original_byte] {
                        sink_in_worker.add(value).unwrap();
                    }
                    drop(sink_in_worker);
                    finished_in_worker.store(true, Ordering::Release);
                },
            ))
            .unwrap();
        });
        assert_eq!(buffer.byte_length(), 0);
        values.set(0, 99.into());
        let deadline = js_sys::Date::now() + 5000.0;
        while !finished.load(Ordering::Acquire) {
            assert!(
                js_sys::Date::now() < deadline,
                "nested worker did not finish"
            );
        }
    }
    #[cfg(not(target_family = "wasm"))]
    drop(sink);
}

#[frb(stream_dart_await)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn func_stream_return_panic_twin_rust_async_sse(
    _sink: StreamSink<String, flutter_rust_bridge::SseCodec>,
) -> anyhow::Result<()> {
    panic!("deliberate panic")
}

#[allow(unused_variables)]
#[flutter_rust_bridge::frb(serialize)]
pub async fn func_stream_sink_arg_position_twin_rust_async_sse(
    a: u32,
    b: u32,
    c: StreamSink<u32, flutter_rust_bridge::SseCodec>,
) {
}

pub struct MyStreamEntryTwinRustAsyncSse {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_stream_of_struct_twin_rust_async_sse(
    _sink: StreamSink<MyStreamEntryTwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
) {
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct LogTwinRustAsyncSse {
    pub key: u32,
    pub value: u32,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_stream_sink_at_1_twin_rust_async_sse(
    key: u32,
    max: u32,
    sink: StreamSink<LogTwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_stream_sink_at_2_twin_rust_async_sse(
    key: u32,
    sink: StreamSink<LogTwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_stream_sink_at_3_twin_rust_async_sse(
    sink: StreamSink<LogTwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
    key: u32,
    max: u32,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool())
        .execute(transfer!(|| { handle_stream_inner(key, max, sink) }));
}

fn handle_stream_inner(
    key: u32,
    max: u32,
    sink: StreamSink<LogTwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
) {
    for i in 0..max {
        sink.add(LogTwinRustAsyncSse { key, value: i }).unwrap();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn stream_sink_fixed_sized_primitive_array_twin_rust_async_sse(
    sink: StreamSink<[u8; 2], flutter_rust_bridge::SseCodec>,
) {
    sink.add([1, 2]).unwrap();
    sink.add([3, 4]).unwrap();
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn stream_sink_inside_vec_twin_rust_async_sse(
    arg: Vec<StreamSink<i32, flutter_rust_bridge::SseCodec>>,
) {
    for sink in arg {
        sink.add(100).unwrap();
        sink.add(200).unwrap();
    }
}

pub struct MyStructContainingStreamSinkTwinRustAsyncSse {
    pub a: i32,
    pub b: StreamSink<i32, flutter_rust_bridge::SseCodec>,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn stream_sink_inside_struct_twin_rust_async_sse(
    arg: MyStructContainingStreamSinkTwinRustAsyncSse,
) {
    arg.b.add(arg.a).unwrap();
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_stream_add_value_and_error_twin_rust_async_sse(
    sink: StreamSink<i32, flutter_rust_bridge::SseCodec>,
) {
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
        sink.add(100).unwrap();
        sink.add(200).unwrap();
        sink.add_error(anyhow!("deliberate error")).unwrap();
    }));
}
