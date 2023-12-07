// event listener test

use anyhow::{anyhow, Result};
use flutter_rust_bridge::{frb, StreamSink, StreamSinkImpl};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref EVENTS: Mutex<Option<StreamSinkImpl<EventTwinNormal>>> = Default::default();
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

pub fn register_event_listener_twin_normal(
    listener: impl StreamSink<EventTwinNormal>,
) -> Result<()> {
    match EVENTS.lock() {
        Ok(mut guard) => {
            *guard = Some(listener);
            Ok(())
        }
        Err(err) => Err(anyhow!("Could not register event listener: {}", err)),
    }
}

pub fn close_event_listener_twin_normal() {
    if let Ok(Some(sink)) = EVENTS.lock().map(|mut guard| guard.take()) {
        sink.close();
    }
}

pub fn create_event_twin_normal(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(EventTwinNormal { address, payload });
        }
    }
}
