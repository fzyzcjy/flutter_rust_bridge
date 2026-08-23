// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

#[derive(Debug, Clone)]
pub struct Log2TwinSse {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinSse {
    pub a: String,
}

impl ConcatenateWithTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn new_twin_sse(a: String) -> ConcatenateWithTwinSse {
        ConcatenateWithTwinSse { a }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn concatenate_twin_sse(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn concatenate_static_twin_sse(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn simple_getter_twin_sse(&self) -> String {
        self.a.clone()
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn handle_some_stream_sink_twin_sse(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinSse, flutter_rust_bridge::SseCodec>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinSse {
                    key,
                    value: format!("{a}{i}"),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn handle_some_stream_sink_at_1_twin_sse(
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
    pub fn handle_some_static_stream_sink_twin_sse(
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinSse, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinSse {
                    key,
                    value: i.to_string(),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn handle_some_static_stream_sink_single_arg_twin_sse(
        sink: StreamSink<u32, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }
}

pub struct SumWithTwinSse {
    pub x: u32,
}

impl SumWithTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn sum_twin_sse(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn get_sum_struct_twin_sse() -> SumWithTwinSse {
    SumWithTwinSse { x: 21 }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn get_sum_array_twin_sse(a: u32, b: u32, c: u32) -> [SumWithTwinSse; 3] {
    [
        SumWithTwinSse { x: a },
        SumWithTwinSse { x: b },
        SumWithTwinSse { x: c },
    ]
}

pub struct MyCallableTwinSse {
    pub one: String,
}

impl MyCallableTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn call(&self, two: String) -> String {
        self.one.clone() + &two
    }
}

pub struct SimpleStructTwinSse {
    pub one: String,
}

impl SimpleStructTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn return_self_twin_sse(one: String) -> Self {
        Self { one }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn receiver_borrow_twin_sse(&self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn receiver_own_twin_sse(self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn arg_self_twin_sse(a: Self, b: Self) -> String {
        a.one + &b.one
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn vec_self_twin_sse(arg: Vec<Self>) -> Vec<String> {
        arg.into_iter().map(|x| x.one).collect()
    }
}

// #1818
pub enum SimpleEnumTwinSse {
    First,
    Second(String),
}

impl SimpleEnumTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn return_self_twin_sse(one: String) -> Self {
        Self::Second(one)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub fn simple_method_twin_sse(&self) -> String {
        match self {
            SimpleEnumTwinSse::First => "".to_owned(),
            SimpleEnumTwinSse::Second(inner) => inner.to_owned(),
        }
    }
}

// #1870
pub enum SimplePrimitiveEnumTwinSse {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn simple_method_twin_sse(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinSse::First => 100,
            SimplePrimitiveEnumTwinSse::Second => 200,
        }
    }
}

// #1838
pub struct StaticOnlyTwinSse {
    pub one: String,
}

impl StaticOnlyTwinSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_method_twin_sse(a: i32) -> i32 {
        a
    }
}

// #1838
#[frb(opaque)]
pub struct StaticGetterOnlyTwinSse {}

impl StaticGetterOnlyTwinSse {
    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    pub fn static_getter_twin_sse() -> i32 {
        42
    }
}
