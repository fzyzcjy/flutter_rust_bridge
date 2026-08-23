// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::frb_generated::StreamSink;
use crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER;
use flutter_rust_bridge::for_generated::BaseThreadPool;
use flutter_rust_bridge::{frb, transfer};

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

    #[frb(getter)]
    pub fn simple_getter_twin_normal(&self) -> String {
        self.a.clone()
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
        }));
    }

    pub fn handle_some_stream_sink_at_1_twin_normal(&self, sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
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
        }));
    }

    pub fn handle_some_static_stream_sink_single_arg_twin_normal(sink: StreamSink<u32>) {
        (FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()).execute(transfer!(|| {
            for i in 0..5 {
                sink.add(i).unwrap();
            }
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

pub struct MyCallableTwinNormal {
    pub one: String,
}

impl MyCallableTwinNormal {
    pub fn call(&self, two: String) -> String {
        self.one.clone() + &two
    }
}

pub struct SimpleStructTwinNormal {
    pub one: String,
}

impl SimpleStructTwinNormal {
    pub fn return_self_twin_normal(one: String) -> Self {
        Self { one }
    }

    pub fn receiver_borrow_twin_normal(&self) -> String {
        self.one.to_owned()
    }

    pub fn receiver_own_twin_normal(self) -> String {
        self.one.to_owned()
    }

    pub fn arg_self_twin_normal(a: Self, b: Self) -> String {
        a.one + &b.one
    }

    pub fn vec_self_twin_normal(arg: Vec<Self>) -> Vec<String> {
        arg.into_iter().map(|x| x.one).collect()
    }
}

// #1818
pub enum SimpleEnumTwinNormal {
    First,
    Second(String),
}

impl SimpleEnumTwinNormal {
    pub fn return_self_twin_normal(one: String) -> Self {
        Self::Second(one)
    }

    pub fn simple_method_twin_normal(&self) -> String {
        match self {
            SimpleEnumTwinNormal::First => "".to_owned(),
            SimpleEnumTwinNormal::Second(inner) => inner.to_owned(),
        }
    }
}

// #1870
pub enum SimplePrimitiveEnumTwinNormal {
    First,
    Second,
}

impl SimplePrimitiveEnumTwinNormal {
    pub fn simple_method_twin_normal(&self) -> i32 {
        match self {
            SimplePrimitiveEnumTwinNormal::First => 100,
            SimplePrimitiveEnumTwinNormal::Second => 200,
        }
    }
}

// #1838
pub struct StaticOnlyTwinNormal {
    pub one: String,
}

impl StaticOnlyTwinNormal {
    pub fn static_method_twin_normal(a: i32) -> i32 {
        a
    }
}

// #1838
#[frb(opaque)]
pub struct StaticGetterOnlyTwinNormal {}

impl StaticGetterOnlyTwinNormal {
    #[frb(getter)]
    pub fn static_getter_twin_normal() -> i32 {
        42
    }
}
