// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use crate::api::event_listener::{EventTwinNormal, EVENTS};
use flutter_rust_bridge::frb;

// #1836
#[frb(sync)]
pub fn create_event_sync_twin_normal(address: String, payload: String) {
    if let Ok(mut guard) = EVENTS.lock() {
        if let Some(sink) = guard.as_mut() {
            sink.add(EventTwinNormal { address, payload }).unwrap();
        }
    }
}
