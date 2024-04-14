// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

#[derive(Debug, Clone)]
pub struct Log2TwinRustAsyncSse {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinRustAsyncSse {
    pub a: String,
}

impl ConcatenateWithTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_twin_rust_async_sse(a: String) -> ConcatenateWithTwinRustAsyncSse {
        ConcatenateWithTwinRustAsyncSse { a }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn concatenate_twin_rust_async_sse(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn concatenate_static_twin_rust_async_sse(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn simple_getter_twin_rust_async_sse(&self) -> String {
        self.a.clone()
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn handle_some_stream_sink_twin_rust_async_sse(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinRustAsyncSse {
                    key,
                    value: format!("{a}{i}"),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn handle_some_stream_sink_at_1_twin_rust_async_sse(
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
    pub async fn handle_some_static_stream_sink_twin_rust_async_sse(
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinRustAsyncSse, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinRustAsyncSse {
                    key,
                    value: i.to_string(),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn handle_some_static_stream_sink_single_arg_twin_rust_async_sse(
        sink: StreamSink<u32, flutter_rust_bridge::SseCodec>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }
}

pub struct SumWithTwinRustAsyncSse {
    pub x: u32,
}

impl SumWithTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn sum_twin_rust_async_sse(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn get_sum_struct_twin_rust_async_sse() -> SumWithTwinRustAsyncSse {
    SumWithTwinRustAsyncSse { x: 21 }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn get_sum_array_twin_rust_async_sse(
    a: u32,
    b: u32,
    c: u32,
) -> [SumWithTwinRustAsyncSse; 3] {
    [
        SumWithTwinRustAsyncSse { x: a },
        SumWithTwinRustAsyncSse { x: b },
        SumWithTwinRustAsyncSse { x: c },
    ]
}

pub struct MyCallableTwinRustAsyncSse {
    pub one: String,
}

impl MyCallableTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn call(&self, two: String) -> String {
        self.one.clone() + &two
    }
}

pub struct SimpleStructTwinRustAsyncSse {
    pub one: String,
}

impl SimpleStructTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn return_self_twin_rust_async_sse(one: String) -> Self {
        Self { one }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn receiver_borrow_twin_rust_async_sse(&self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn receiver_own_twin_rust_async_sse(self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn arg_self_twin_rust_async_sse(a: Self, b: Self) -> String {
        a.one + &b.one
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn vec_self_twin_rust_async_sse(arg: Vec<Self>) -> Vec<String> {
        arg.into_iter().map(|x| x.one).collect()
    }
}

// #1818
pub enum SimpleEnumTwinRustAsyncSse {
    First,
    Second(String),
}

impl SimpleEnumTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn return_self_twin_rust_async_sse(one: String) -> Self {
        Self::Second(one)
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn simple_method_twin_rust_async_sse(&self) -> String {
        match self {
            SimpleEnumTwinRustAsyncSse::First => "".to_owned(),
            SimpleEnumTwinRustAsyncSse::Second(inner) => inner.to_owned(),
        }
    }
}

// #1870
pub enum SimplePrimitiveEnumTwinRustAsyncSse {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn simple_method_twin_rust_async_sse(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinRustAsyncSse::First => 100,
            SimplePrimitiveEnumTwinRustAsyncSse::Second => 200,
        }
    }
}

// #1838
pub struct StaticOnlyTwinRustAsyncSse {
    pub one: String,
}

impl StaticOnlyTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_method_twin_rust_async_sse(a: i32) -> i32 {
        a
    }
}

// #1838
#[frb(opaque)]
pub struct StaticGetterOnlyTwinRustAsyncSse {}

impl StaticGetterOnlyTwinRustAsyncSse {
    #[frb(getter)]
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn static_getter_twin_rust_async_sse() -> i32 {
        42
    }
}
