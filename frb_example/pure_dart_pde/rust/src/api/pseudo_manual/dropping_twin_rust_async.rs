// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dropping.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

#![allow(clippy::new_without_default)]

use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicI32, Ordering};

lazy_static! {
    pub(crate) static ref DROP_COUNT: AtomicI32 = AtomicI32::new(0);
}

#[frb(opaque)]
pub struct DroppableTwinRustAsync {
    sink: Option<StreamSink<i32>>,
}

impl Drop for DroppableTwinRustAsync {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

impl DroppableTwinRustAsync {
    pub async fn new_twin_rust_async() -> DroppableTwinRustAsync {
        Self { sink: None }
    }

    pub async fn simple_method_twin_rust_async(&self) {}

    // #1723
    pub async fn create_stream_twin_rust_async(&mut self, sink: StreamSink<i32>) {
        self.sink = Some(sink);
    }

    pub async fn get_drop_count_twin_rust_async() -> i32 {
        DROP_COUNT.load(Ordering::SeqCst)
    }
}
