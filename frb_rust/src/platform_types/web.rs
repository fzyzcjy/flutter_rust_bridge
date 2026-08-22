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

const BROADCAST_CHANNEL_CLOSE_DELAY_MILLIS: i32 = 100;

#[derive(Clone, Debug)]
pub struct SendableMessagePortHandle(String);

thread_local! {
    static BROADCAST_CHANNEL_STATE: RefCell<BroadcastChannelState> = RefCell::new(BroadcastChannelState::new());
}

struct BroadcastChannelState {
    channel_of_name: HashMap<String, MessagePort>,
    pending_close: Vec<MessagePort>,
    close_scheduled: bool,
    close_callback: Closure<dyn FnMut()>,
}

impl BroadcastChannelState {
    fn new() -> Self {
        Self {
            channel_of_name: Default::default(),
            pending_close: Default::default(),
            close_scheduled: false,
            close_callback: Closure::wrap(Box::new(close_pending_message_ports) as Box<dyn FnMut()>),
        }
    }

    fn message_port_of_name(&mut self, name: &str) -> MessagePort {
        if let Some(port) = self.channel_of_name.get(name) {
            return port.clone();
        }

        let port = PortLike::broadcast(name);
        self.channel_of_name.insert(name.to_owned(), port.clone());
        port
    }

    fn release_message_port_name(&mut self, name: &str) {
        if let Some(port) = self.channel_of_name.remove(name) {
            self.close_message_port_later(port);
        }
    }

    fn close_message_port_later(&mut self, port: MessagePort) {
        self.pending_close.push(port);
        if self.close_scheduled {
            return;
        }

        self.close_scheduled = true;
        if let Err(error) = js_set_timeout(
            self.close_callback.as_ref().unchecked_ref(),
            BROADCAST_CHANNEL_CLOSE_DELAY_MILLIS,
        ) {
            crate::console_error!("schedule broadcast channel close: {:?}", error);
            self.close_pending_message_ports();
        }
    }

    fn close_pending_message_ports(&mut self) {
        self.close_scheduled = false;
        for port in self.pending_close.drain(..) {
            if let Err(error) = port.close() {
                crate::console_error!("close broadcast channel: {:?}", error);
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout, catch)]
    fn js_set_timeout(handler: &js_sys::Function, timeout: i32) -> Result<JsValue, JsValue>;
}

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    SendableMessagePortHandle(
        port.dyn_ref::<BroadcastChannel>()
            .map(|channel| channel.name())
            .expect("Not a BroadcastChannel"),
    )
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().message_port_of_name(&handle.0))
}

pub fn release_message_port_handle(handle: &SendableMessagePortHandle) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().release_message_port_name(&handle.0))
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;

fn close_pending_message_ports() {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().close_pending_message_ports())
}
