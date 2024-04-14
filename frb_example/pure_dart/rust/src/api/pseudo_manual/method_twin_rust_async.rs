// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

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

    #[frb(getter)]
    pub async fn simple_getter_twin_rust_async(&self) -> String {
        self.a.clone()
    }

    pub async fn handle_some_stream_sink_twin_rust_async(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinRustAsync>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinRustAsync {
                    key,
                    value: format!("{a}{i}"),
                })
                .unwrap();
            }
        }));
    }

    pub async fn handle_some_stream_sink_at_1_twin_rust_async(&self, sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }

    pub async fn handle_some_static_stream_sink_twin_rust_async(
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinRustAsync>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinRustAsync {
                    key,
                    value: i.to_string(),
                })
                .unwrap();
            }
        }));
    }

    pub async fn handle_some_static_stream_sink_single_arg_twin_rust_async(sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
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

pub struct MyCallableTwinRustAsync {
    pub one: String,
}

impl MyCallableTwinRustAsync {
    pub async fn call(&self, two: String) -> String {
        self.one.clone() + &two
    }
}

pub struct SimpleStructTwinRustAsync {
    pub one: String,
}

impl SimpleStructTwinRustAsync {
    pub async fn return_self_twin_rust_async(one: String) -> Self {
        Self { one }
    }

    pub async fn receiver_borrow_twin_rust_async(&self) -> String {
        self.one.to_owned()
    }

    pub async fn receiver_own_twin_rust_async(self) -> String {
        self.one.to_owned()
    }

    pub async fn arg_self_twin_rust_async(a: Self, b: Self) -> String {
        a.one + &b.one
    }

    pub async fn vec_self_twin_rust_async(arg: Vec<Self>) -> Vec<String> {
        arg.into_iter().map(|x| x.one).collect()
    }
}

// #1818
pub enum SimpleEnumTwinRustAsync {
    First,
    Second(String),
}

impl SimpleEnumTwinRustAsync {
    pub async fn return_self_twin_rust_async(one: String) -> Self {
        Self::Second(one)
    }

    pub async fn simple_method_twin_rust_async(&self) -> String {
        match self {
            SimpleEnumTwinRustAsync::First => "".to_owned(),
            SimpleEnumTwinRustAsync::Second(inner) => inner.to_owned(),
        }
    }
}

// #1870
pub enum SimplePrimitiveEnumTwinRustAsync {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinRustAsync {
    pub async fn simple_method_twin_rust_async(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinRustAsync::First => 100,
            SimplePrimitiveEnumTwinRustAsync::Second => 200,
        }
    }
}

// #1838
pub struct StaticOnlyTwinRustAsync {
    pub one: String,
}

impl StaticOnlyTwinRustAsync {
    pub async fn static_method_twin_rust_async(a: i32) -> i32 {
        a
    }
}

// #1838
#[frb(opaque)]
pub struct StaticGetterOnlyTwinRustAsync {}

impl StaticGetterOnlyTwinRustAsync {
    #[frb(getter)]
    pub async fn static_getter_twin_rust_async() -> i32 {
        42
    }
}
