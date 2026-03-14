// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

#[derive(Debug, Clone)]
pub struct Log2TwinSyncSse {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinSyncSse {
    pub a: String,
}

impl ConcatenateWithTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync_sse(a: String) -> ConcatenateWithTwinSyncSse {
        ConcatenateWithTwinSyncSse { a }
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn concatenate_twin_sync_sse(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn concatenate_static_twin_sync_sse(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_getter_twin_sync_sse(&self) -> String {
        self.a.clone()
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_stream_sink_twin_sync_sse(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinSyncSse, flutter_rust_bridge::SseCodec>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinSyncSse {
                    key,
                    value: format!("{a}{i}"),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_stream_sink_at_1_twin_sync_sse(
        &self,
        sink: StreamSink<u32, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_static_stream_sink_twin_sync_sse(
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinSyncSse, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinSyncSse {
                    key,
                    value: i.to_string(),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_static_stream_sink_single_arg_twin_sync_sse(
        sink: StreamSink<u32, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }
}

pub struct SumWithTwinSyncSse {
    pub x: u32,
}

impl SumWithTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn sum_twin_sync_sse(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn get_sum_struct_twin_sync_sse() -> SumWithTwinSyncSse {
    SumWithTwinSyncSse { x: 21 }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn get_sum_array_twin_sync_sse(a: u32, b: u32, c: u32) -> [SumWithTwinSyncSse; 3] {
    [
        SumWithTwinSyncSse { x: a },
        SumWithTwinSyncSse { x: b },
        SumWithTwinSyncSse { x: c },
    ]
}

pub struct MyCallableTwinSyncSse {
    pub one: String,
}

impl MyCallableTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn call(&self, two: String) -> String {
        self.one.clone() + &two
    }
}

pub struct SimpleStructTwinSyncSse {
    pub one: String,
}

impl SimpleStructTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn return_self_twin_sync_sse(one: String) -> Self {
        Self { one }
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn receiver_borrow_twin_sync_sse(&self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn receiver_own_twin_sync_sse(self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn arg_self_twin_sync_sse(a: Self, b: Self) -> String {
        a.one + &b.one
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn vec_self_twin_sync_sse(arg: Vec<Self>) -> Vec<String> {
        arg.into_iter().map(|x| x.one).collect()
    }
}

// #1818
pub enum SimpleEnumTwinSyncSse {
    First,
    Second(String),
}

impl SimpleEnumTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn return_self_twin_sync_sse(one: String) -> Self {
        Self::Second(one)
    }

    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_method_twin_sync_sse(&self) -> String {
        match self {
            SimpleEnumTwinSyncSse::First => "".to_owned(),
            SimpleEnumTwinSyncSse::Second(inner) => inner.to_owned(),
        }
    }
}

// #1870
pub enum SimplePrimitiveEnumTwinSyncSse {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_method_twin_sync_sse(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinSyncSse::First => 100,
            SimplePrimitiveEnumTwinSyncSse::Second => 200,
        }
    }
}

// #1838
pub struct StaticOnlyTwinSyncSse {
    pub one: String,
}

impl StaticOnlyTwinSyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_twin_sync_sse(a: i32) -> i32 {
        a
    }
}

// #1838
#[frb(opaque)]
pub struct StaticGetterOnlyTwinSyncSse {}

impl StaticGetterOnlyTwinSyncSse {
    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_getter_twin_sync_sse() -> i32 {
        42
    }
}
