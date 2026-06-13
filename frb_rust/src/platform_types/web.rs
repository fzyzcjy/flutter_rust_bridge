use crate::generalized_isolate::PortLike;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

pub type WireSyncRust2DartDco = wasm_bindgen::JsValue;
pub type WireSyncRust2DartSse = wasm_bindgen::JsValue;

pub type MessagePort = crate::generalized_isolate::PortLike;

pub type DartAbi = wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct SendableMessagePortHandle(String);

thread_local! {
    static BROADCAST_CHANNEL_OF_NAME: RefCell<HashMap<String, MessagePort>> = Default::default();
}

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    SendableMessagePortHandle(
        port.dyn_ref::<BroadcastChannel>()
            .map(|channel| channel.name())
            .expect("Not a BroadcastChannel"),
    )
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    BROADCAST_CHANNEL_OF_NAME.with(|channel_of_name| {
        let mut channel_of_name = channel_of_name.borrow_mut();
        channel_of_name
            .entry(handle.0.clone())
            .or_insert_with(|| PortLike::broadcast(&handle.0))
            .clone()
    })
}

pub fn release_message_port_handle(handle: &SendableMessagePortHandle) {
    BROADCAST_CHANNEL_OF_NAME.with(|channel_of_name| {
        if let Some(port) = channel_of_name.borrow_mut().remove(&handle.0) {
            if let Err(error) = port.close() {
                crate::console_error!("close broadcast channel: {:?}", error);
            }
        }
    })
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;
