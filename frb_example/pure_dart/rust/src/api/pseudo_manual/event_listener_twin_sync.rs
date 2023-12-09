// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `event_listener.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// event listener test

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref EVENTS: Mutex<Option<StreamSink<EventTwinSync>>> = Default::default();
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Clone)]
pub struct EventTwinSync {
    pub address: String,
    pub payload: String,
}

impl EventTwinSync {
    #[flutter_rust_bridge::frb(sync)]
    pub fn as_string_twin_sync(&self) -> String {
        format!("{}: {}", self.address, self.payload)
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn register_event_listener_twin_sync(listener: StreamSink<EventTwinSync>) -> Result<()> {
    match EVENTS.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn close_event_listener_twin_sync() {
    if let Ok(Some(sink)) = EVENTS.lock().map(|mut guard| guard.take()) {
        sink.close();
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn create_event_twin_sync(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(EventTwinSync { address, payload });
        }
    }
}
