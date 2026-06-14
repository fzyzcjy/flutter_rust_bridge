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

    fn release_message_port_name(&mut self, name: &str) -> Option<MessagePort> {
        self.channel_of_name.remove(name)
    }

    fn push_pending_close(&mut self, port: MessagePort) -> bool {
        self.pending_close.push(port);
        if self.close_scheduled {
            true
        } else {
            self.close_scheduled = true;
            false
        }
    }

    fn close_callback_function(&self) -> &js_sys::Function {
        self.close_callback.as_ref().unchecked_ref()
    }

    fn take_pending_close(&mut self) -> Vec<MessagePort> {
        self.close_scheduled = false;
        self.pending_close.drain(..).collect()
    }
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
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().message_port_of_name(&handle.0))
}

pub fn release_message_port_handle(handle: &SendableMessagePortHandle) {
    BROADCAST_CHANNEL_STATE.with(|state| {
        if let Some(port) = state.borrow_mut().release_message_port_name(&handle.0) {
            close_message_port_later(port);
        }
    })
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;

fn close_message_port_later(port: MessagePort) {
    if BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().push_pending_close(port)) {
        return;
    }

    if let Err(error) = BROADCAST_CHANNEL_STATE.with(|state| {
        set_timeout(
            state.borrow().close_callback_function(),
            BROADCAST_CHANNEL_CLOSE_DELAY_MILLIS,
        )
    }) {
        crate::console_error!("schedule broadcast channel close: {:?}", error);
        close_pending_message_ports();
    }
}

fn close_pending_message_ports() {
    let ports = BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().take_pending_close());
    for port in ports {
        close_message_port(port);
    }
}

fn close_message_port(port: MessagePort) {
    if let Err(error) = port.close() {
        crate::console_error!("close broadcast channel: {:?}", error);
    }
}
