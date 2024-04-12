// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};
use log::info;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

// Do not test this on web+async, since atomic is not allowed there
pub fn func_stream_realistic_twin_normal(sink: StreamSink<String>, arg: String) {
    info!("handle_stream_realistic arg={}", arg);

    let cnt = Arc::new(AtomicI32::new(0));

    // just to show that, you can send data to sink even in other threads
    let cnt2 = cnt.clone();
    let sink2 = sink.clone();
    (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
        for i in 0..5 {
            let old_cnt = cnt2.fetch_add(1, Ordering::SeqCst);
            let msg = format!("(thread=child, i={i}, old_cnt={old_cnt})");
            format!("send data to sink msg={msg}");
            sink2.add(msg).unwrap();
            sleep(Duration::from_millis(100));
        }
    }));

    for i in 0..5 {
        let old_cnt = cnt.fetch_add(1, Ordering::SeqCst);
        let msg = format!("(thread=normal, i={i}, old_cnt={old_cnt})");
        format!("send data to sink msg={msg}");
        sink.add(msg).unwrap();
        sleep(Duration::from_millis(50));
    }
}

#[frb(stream_dart_await)]
pub fn stream_sink_dart_async_twin_normal(sink: StreamSink<i32>) {
    sink.add(100).unwrap()
}
