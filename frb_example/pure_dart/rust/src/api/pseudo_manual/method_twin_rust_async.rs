// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{transfer, StreamSink};

#[derive(Debug, Clone)]
pub struct Log2TwinRustAsync {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinRustAsync {
    pub a: String,
}

impl ConcatenateWithTwinRustAsync {
    pub async fn new_twin_rust_async(a: String) -> ConcatenateWithTwinRustAsync {
        ConcatenateWithTwinRustAsync { a }
    }

    pub async fn concatenate_twin_rust_async(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    pub async fn concatenate_static_twin_rust_async(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    pub async fn handle_some_stream_sink_twin_rust_async(
        &self,
        key: u32,
        max: u32,
        sink: impl StreamSink<Log2TwinRustAsync>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinRustAsync {
                    key,
                    value: format!("{a}{i}"),
                });
            }
            sink.close();
        }));
    }

    pub async fn handle_some_stream_sink_at_1_twin_rust_async(&self, sink: impl StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        }));
    }

    pub async fn handle_some_static_stream_sink_twin_rust_async(
        key: u32,
        max: u32,
        sink: impl StreamSink<Log2TwinRustAsync>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinRustAsync {
                    key,
                    value: i.to_string(),
                });
            }
            sink.close();
        }));
    }

    pub async fn handle_some_static_stream_sink_single_arg_twin_rust_async(
        sink: impl StreamSink<u32>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i);
            }
            sink.close();
        }));
    }
}

pub struct SumWithTwinRustAsync {
    pub x: u32,
}

impl SumWithTwinRustAsync {
    pub async fn sum_twin_rust_async(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

pub async fn get_sum_struct_twin_rust_async() -> SumWithTwinRustAsync {
    SumWithTwinRustAsync { x: 21 }
}

pub async fn get_sum_array_twin_rust_async(a: u32, b: u32, c: u32) -> [SumWithTwinRustAsync; 3] {
    [
        SumWithTwinRustAsync { x: a },
        SumWithTwinRustAsync { x: b },
        SumWithTwinRustAsync { x: c },
    ]
}
