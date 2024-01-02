// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::transfer;
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
            let _ = sink2.add(msg);
            sleep(Duration::from_millis(100));
        }

        if cnt2.load(Ordering::SeqCst) == 10 {
            sink2.close();
        }
    }));

    for i in 0..5 {
        let old_cnt = cnt.fetch_add(1, Ordering::SeqCst);
        let msg = format!("(thread=normal, i={i}, old_cnt={old_cnt})");
        format!("send data to sink msg={msg}");
        let _ = sink.add(msg);
        sleep(Duration::from_millis(50));
    }

    if cnt.load(Ordering::SeqCst) == 10 {
        sink.close().unwrap();
    }
}
