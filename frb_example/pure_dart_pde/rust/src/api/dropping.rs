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
pub struct DroppableTwinNormal {
    sink: Option<StreamSink<i32>>,
}

impl Drop for DroppableTwinNormal {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

impl DroppableTwinNormal {
    pub fn new_twin_normal() -> DroppableTwinNormal {
        Self { sink: None }
    }

    pub fn simple_method_twin_normal(&self) {}

    // #1723
    pub fn create_stream(&mut self, sink: StreamSink<i32>) {
        self.sink = Some(sink);
    }

    pub fn get_drop_count_twin_normal() -> i32 {
        DROP_COUNT.load(Ordering::SeqCst)
    }
}
