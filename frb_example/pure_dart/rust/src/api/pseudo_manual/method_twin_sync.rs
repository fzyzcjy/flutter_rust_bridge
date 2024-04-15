// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `method.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

#[derive(Debug, Clone)]
pub struct Log2TwinSync {
    pub key: u32,
    pub value: String,
}

pub struct ConcatenateWithTwinSync {
    pub a: String,
}

impl ConcatenateWithTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new_twin_sync(a: String) -> ConcatenateWithTwinSync {
        ConcatenateWithTwinSync { a }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn concatenate_twin_sync(&self, b: String) -> String {
        format!("{}{b}", self.a)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn concatenate_static_twin_sync(a: String, b: String) -> String {
        format!("{a}{b}")
    }

    #[frb(getter)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_getter_twin_sync(&self) -> String {
        self.a.clone()
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_stream_sink_twin_sync(
        &self,
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinSync>,
    ) {
        let a = self.a.clone();
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinSync {
                    key,
                    value: format!("{a}{i}"),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_stream_sink_at_1_twin_sync(&self, sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_static_stream_sink_twin_sync(
        key: u32,
        max: u32,
        sink: StreamSink<Log2TwinSync>,
    ) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..max {
                sink.add(Log2TwinSync {
                    key,
                    value: i.to_string(),
                })
                .unwrap();
            }
        }));
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn handle_some_static_stream_sink_single_arg_twin_sync(sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
        }));
    }
}

pub struct SumWithTwinSync {
    pub x: u32,
}

impl SumWithTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn sum_twin_sync(&self, y: u32, z: u32) -> u32 {
        self.x + y + z
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_sum_struct_twin_sync() -> SumWithTwinSync {
    SumWithTwinSync { x: 21 }
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_sum_array_twin_sync(a: u32, b: u32, c: u32) -> [SumWithTwinSync; 3] {
    [
        SumWithTwinSync { x: a },
        SumWithTwinSync { x: b },
        SumWithTwinSync { x: c },
    ]
}

pub struct MyCallableTwinSync {
    pub one: String,
}

impl MyCallableTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn call(&self, two: String) -> String {
        self.one.clone() + &two
    }
}

pub struct SimpleStructTwinSync {
    pub one: String,
}

impl SimpleStructTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn return_self_twin_sync(one: String) -> Self {
        Self { one }
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn receiver_borrow_twin_sync(&self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn receiver_own_twin_sync(self) -> String {
        self.one.to_owned()
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn arg_self_twin_sync(a: Self, b: Self) -> String {
        a.one + &b.one
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn vec_self_twin_sync(arg: Vec<Self>) -> Vec<String> {
        arg.into_iter().map(|x| x.one).collect()
    }
}

// #1818
pub enum SimpleEnumTwinSync {
    First,
    Second(String),
}

impl SimpleEnumTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn return_self_twin_sync(one: String) -> Self {
        Self::Second(one)
    }

    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_method_twin_sync(&self) -> String {
        match self {
            SimpleEnumTwinSync::First => "".to_owned(),
            SimpleEnumTwinSync::Second(inner) => inner.to_owned(),
        }
    }
}

// #1870
pub enum SimplePrimitiveEnumTwinSync {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn simple_method_twin_sync(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinSync::First => 100,
            SimplePrimitiveEnumTwinSync::Second => 200,
        }
    }
}

// #1838
pub struct StaticOnlyTwinSync {
    pub one: String,
}

impl StaticOnlyTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_method_twin_sync(a: i32) -> i32 {
        a
    }
}

// #1838
#[frb(opaque)]
pub struct StaticGetterOnlyTwinSync {}

impl StaticGetterOnlyTwinSync {
    #[frb(getter)]
    #[flutter_rust_bridge::frb(sync)]
    pub fn static_getter_twin_sync() -> i32 {
        42
    }
}
