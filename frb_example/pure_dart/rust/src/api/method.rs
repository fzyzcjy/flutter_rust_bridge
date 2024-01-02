use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::transfer;

#[derive(Debug, Clone)]
pub struct Log2TwinNormal {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinNormal {
    pub a: String,
}

impl ConcatenateWithTwinNormal {
    pub fn new_twin_normal(a: String) -> ConcatenateWithTwinNormal {
        ConcatenateWithTwinNormal { a }
    }

    pub fn concatenate_twin_normal(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    pub fn concatenate_static_twin_normal(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    pub fn handle_some_stream_sink_twin_normal(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinNormal>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinNormal {
                    key,
                    value: format!("{a}{i}"),
                })
                .unwrap();
            }
            sink.close().unwrap();
        }));
    }

    pub fn handle_some_stream_sink_at_1_twin_normal(&self, sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
            sink.close().unwrap();
        }));
    }

    pub fn handle_some_static_stream_sink_twin_normal(
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinNormal>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinNormal {
                    key,
                    value: i.to_string(),
                })
                .unwrap();
            }
            sink.close().unwrap();
        }));
    }

    pub fn handle_some_static_stream_sink_single_arg_twin_normal(sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
            sink.close().unwrap();
        }));
    }
}

pub struct SumWithTwinNormal {
    pub x: u32,
}

impl SumWithTwinNormal {
    pub fn sum_twin_normal(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

pub fn get_sum_struct_twin_normal() -> SumWithTwinNormal {
    SumWithTwinNormal { x: 21 }
}

pub fn get_sum_array_twin_normal(a: u32, b: u32, c: u32) -> [SumWithTwinNormal; 3] {
    [
        SumWithTwinNormal { x: a },
        SumWithTwinNormal { x: b },
        SumWithTwinNormal { x: c },
    ]
}
