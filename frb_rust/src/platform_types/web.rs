use crate::generalized_isolate::PortLike;

pub type WireSyncRust2DartDco = wasm_bindgen::JsValue;
pub type WireSyncRust2DartSse = wasm_bindgen::JsValue;

pub type MessagePort = crate::generalized_isolate::PortLike;

pub type DartAbi = wasm_bindgen::JsValue;

#[derive(Clone, Debug)]
pub struct SendableMessagePortHandle(String);

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    SendableMessagePortHandle(port.channel_name().expect("Not a named message port"))
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    PortLike::broadcast(&handle.0)
}

pub fn release_message_port_handle(_handle: &SendableMessagePortHandle) {}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;
