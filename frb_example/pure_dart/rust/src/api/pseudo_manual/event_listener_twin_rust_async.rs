// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `event_listener.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// event listener test

use anyhow::{anyhow, Result};
use flutter_rust_bridge::{frb, StreamSink, StreamSinkImpl};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref EVENTS: Mutex<Option<StreamSinkImpl<EventTwinRustAsync>>> = Default::default();
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Clone)]
pub struct EventTwinRustAsync {
    pub address: String,
    pub payload: String,
}

impl EventTwinRustAsync {
    pub async fn as_string_twin_rust_async(&self) -> String {
        format!("{}: {}", self.address, self.payload)
    }
}

pub async fn register_event_listener_twin_rust_async(
    listener: impl StreamSink<EventTwinRustAsync>,
) -> Result<()> {
    match EVENTS.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

pub async fn close_event_listener_twin_rust_async() {
    if let Ok(Some(sink)) = EVENTS.lock().map(|mut guard| guard.take()) {
        sink.close();
    }
}

pub async fn create_event_twin_rust_async(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(EventTwinRustAsync { address, payload });
        }
    }
}
