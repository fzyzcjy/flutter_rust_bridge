use crate::generalized_isolate::PortLike;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
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
    static BROADCAST_CHANNELS_PENDING_CLOSE: RefCell<Vec<MessagePort>> = Default::default();
    static BROADCAST_CHANNEL_CLOSE_CALLBACK: Closure<dyn FnMut()> = Closure::wrap(Box::new(close_pending_message_ports) as Box<dyn FnMut()>);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout, catch)]
    fn set_timeout(handler: &js_sys::Function, timeout: i32) -> Result<JsValue, JsValue>;
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
        if let Some(port) = channel_of_name.get(&handle.0) {
            return port.clone();
        }

        let port = PortLike::broadcast(&handle.0);
        channel_of_name.insert(handle.0.clone(), port.clone());
        port
    })
}

pub fn release_message_port_handle(handle: &SendableMessagePortHandle) {
    BROADCAST_CHANNEL_OF_NAME.with(|channel_of_name| {
        if let Some(port) = channel_of_name.borrow_mut().remove(&handle.0) {
            close_message_port_later(port);
        }
    })
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;

fn close_message_port_later(port: MessagePort) {
    BROADCAST_CHANNELS_PENDING_CLOSE.with(|ports| ports.borrow_mut().push(port));

    if let Err(error) = BROADCAST_CHANNEL_CLOSE_CALLBACK
        .with(|callback| set_timeout(callback.as_ref().unchecked_ref(), 0))
    {
        crate::console_error!("schedule broadcast channel close: {:?}", error);
        close_pending_message_ports();
    }
}

fn close_pending_message_ports() {
    let ports = BROADCAST_CHANNELS_PENDING_CLOSE
        .with(|ports| ports.borrow_mut().drain(..).collect::<Vec<_>>());
    for port in ports {
        close_message_port(port);
    }
}

fn close_message_port(port: MessagePort) {
    if let Err(error) = port.close() {
        crate::console_error!("close broadcast channel: {:?}", error);
    }
}
