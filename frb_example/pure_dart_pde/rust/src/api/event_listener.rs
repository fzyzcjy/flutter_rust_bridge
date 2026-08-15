// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

// event listener test

use crate::frb_generated::StreamSink;
use anyhow::{anyhow, Result};
use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref EVENTS: Mutex<Option<StreamSink<EventTwinNormal>>> = Default::default();
}

#[frb(dart_metadata = ("freezed"))]
#[derive(Clone)]
pub struct EventTwinNormal {
    pub address: String,
    pub payload: String,
}

impl EventTwinNormal {
    pub fn as_string_twin_normal(&self) -> String {
        format!("{}: {}", self.address, self.payload)
    }
}

#[frb(stream_dart_await)]
pub fn register_event_listener_twin_normal(listener: StreamSink<EventTwinNormal>) -> Result<()> {
    match EVENTS.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

pub fn close_event_listener_twin_normal() {
    let _ = EVENTS.lock().map(|mut guard| guard.take());
}

pub fn create_event_twin_normal(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(EventTwinNormal { address, payload }).unwrap();
        }
    }
}

// FRB_INTERNAL_GENERATOR_DISABLE_DUPLICATOR_START
// #1836
#[frb(sync)]
pub fn create_event_sync_twin_normal(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(EventTwinNormal { address, payload }).unwrap();
        }
    }
}
// FRB_INTERNAL_GENERATOR_DISABLE_DUPLICATOR_END
