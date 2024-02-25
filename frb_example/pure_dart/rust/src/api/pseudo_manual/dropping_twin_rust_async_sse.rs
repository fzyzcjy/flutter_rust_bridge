// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dropping.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

#![allow(clippy::new_without_default)]

use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicI32, Ordering};

lazy_static! {
    pub(crate) static ref DROP_COUNT: AtomicI32 = AtomicI32::new(0);
}

#[frb(opaque)]
pub struct DroppableTwinRustAsyncSse {
    sink: Option<StreamSink<i32, flutter_rust_bridge::SseCodec>>,
}

impl Drop for DroppableTwinRustAsyncSse {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

impl DroppableTwinRustAsyncSse {
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn new_twin_rust_async_sse() -> DroppableTwinRustAsyncSse {
        Self { sink: None }
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn simple_method_twin_rust_async_sse(&self) {}

    // #1723
    #[flutter_rust_bridge::frb(serialize)]
    pub async fn create_stream_twin_rust_async_sse(
        &mut self,
        sink: StreamSink<i32, flutter_rust_bridge::SseCodec>,
    ) {
        self.sink = Some(sink);
    }

    #[flutter_rust_bridge::frb(serialize)]
    pub async fn get_drop_count_twin_rust_async_sse() -> i32 {
        DROP_COUNT.load(Ordering::SeqCst)
    }
}
