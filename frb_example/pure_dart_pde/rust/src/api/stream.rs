// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use anyhow::anyhow;
use flutter_rust_bridge::for_generated::BaseThreadPool;
#[cfg(target_family = "wasm")]
use flutter_rust_bridge::for_generated::{js_sys, TransferClosure};
use flutter_rust_bridge::{frb, transfer};

#[frb(stream_dart_await)]
pub fn func_stream_return_error_twin_normal(_sink: StreamSink<String>) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

pub fn immediate_stream_twin_normal(sink: StreamSink<i32>) {
    sink.add(0).unwrap();
    sink.add(1).unwrap();
}

pub fn stream_worker_transfer_twin_normal(sink: StreamSink<i32>) {
    #[cfg(target_family = "wasm")]
    {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;
        use wasm_bindgen::{closure::Closure, JsCast, JsValue};
        use web_sys::{MessageChannel, MessagePort};

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
        let coordination = MessageChannel::new().unwrap();
        let parent_port = coordination.port1();
        let child_port = coordination.port2();
        FLUTTER_RUST_BRIDGE_HANDLER.thread_pool().with(|pool| {
            pool.execute(TransferClosure::new(
                vec![
                    values.clone().into(),
                    buffer.clone().into(),
                    child_port.clone().into(),
                ],
                vec![buffer.clone().into(), child_port.into()],
                move |data| {
                    let original_value =
                        js_sys::Array::from(&data[0]).get(0).as_f64().unwrap() as i32;
                    let original_byte = js_sys::Uint8Array::new(&data[1]).get_index(0) as i32;
                    let port: MessagePort = data[2].clone().unchecked_into();
                    let port_in_callback = port.clone();
                    let callback = Closure::once_into_js(move || {
                        for value in [i32::from(rejected), original_value, original_byte] {
                            sink_in_worker.add(value).unwrap();
                        }
                        drop(sink_in_worker);
                        finished_in_worker.store(true, Ordering::Release);
                        port_in_callback.set_onmessage(None);
                        port_in_callback.close();
                    });
                    port.set_onmessage(Some(callback.unchecked_ref()));
                    port.post_message(&JsValue::NULL).unwrap();
                },
            ))
            .unwrap();
        });
        assert_eq!(buffer.byte_length(), 0);
        values.set(0, 99.into());
        let parent_port_in_callback = parent_port.clone();
        let callback = Closure::once_into_js(move || {
            parent_port_in_callback
                .post_message(&JsValue::NULL)
                .unwrap();
            let deadline = js_sys::Date::now() + 5000.0;
            while !finished.load(Ordering::Acquire) {
                assert!(
                    js_sys::Date::now() < deadline,
                    "nested worker did not finish"
                );
            }
            parent_port_in_callback.set_onmessage(None);
            parent_port_in_callback.close();
            drop(sink);
        });
        parent_port.set_onmessage(Some(callback.unchecked_ref()));
    }
    #[cfg(not(target_family = "wasm"))]
    drop(sink);
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
