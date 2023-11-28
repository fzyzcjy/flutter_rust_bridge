// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use anyhow::anyhow;
use flutter_rust_bridge::{spawn, StreamSink};
use log::info;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

pub fn func_stream_realistic_twin_normal(sink: StreamSink<String>, arg: String) {
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

pub fn func_stream_return_error_twin_normal(_sink: StreamSink<String>) -> anyhow::Result<()> {
    Err(anyhow!("deliberate error"))
}

pub fn func_stream_return_panic_twin_normal(_sink: StreamSink<String>) -> anyhow::Result<()> {
    panic!("deliberate panic")
}

#[allow(unused_variables)]
pub fn func_stream_sink_arg_position_twin_normal(a: u32, b: u32, c: StreamSink<u32>) {}

pub struct MyStreamEntry {
    pub hello: String,
}

// TODO #11193
// https://github.com/fzyzcjy/flutter_rust_bridge/issues/398 reports a compile error like this
pub fn handle_stream_of_struct(_sink: StreamSink<MyStreamEntry>) {
    // Ok(())
}

#[derive(Debug, Clone)]
pub struct Log {
    pub key: u32,
    pub value: u32,
}

pub fn handle_stream_sink_at_1(key: u32, max: u32, sink: StreamSink<Log>) {
    spawn!(|| {
        for i in 0..max {
            let _ = sink.add(Log { key, value: i });
        }
        sink.close();
    });
}

pub fn handle_stream_sink_at_2(key: u32, sink: StreamSink<Log>, max: u32) {
    handle_stream_sink_at_1(key, max, sink)
}

pub fn handle_stream_sink_at_3(sink: StreamSink<Log>, key: u32, max: u32) {
    handle_stream_sink_at_1(key, max, sink)
}
