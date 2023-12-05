use web_sys::BroadcastChannel;

/// cbindgen:ignore
pub type WireSyncReturn = wasm_bindgen::JsValue;

pub type MessagePort = crate::generalized_isolate::PortLike;

pub type DartAbi = wasm_bindgen::JsValue;

pub struct SendableMessagePortHandle(String);

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    port.dyn_ref::<BroadcastChannel>()
        .map(|channel| channel.name())
        .expect("Not a BroadcastChannel")
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    PortLike::broadcast(&handle.0)
}
