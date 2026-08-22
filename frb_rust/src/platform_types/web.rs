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
    channel_of_name: HashMap<String, CachedBroadcastChannel>,
    pending_close: Vec<CachedBroadcastChannel>,
    close_scheduled: bool,
    close_callback: Closure<dyn FnMut()>,
}

struct CachedBroadcastChannel {
    message_port: MessagePort,
    ready_port: MessagePort,
    _ready_callback: Closure<dyn FnMut(web_sys::MessageEvent)>,
    pending_messages: Vec<JsValue>,
    ready: bool,
    release_requested: bool,
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
        if !self.channel_of_name.contains_key(name) {
            self.channel_of_name
                .insert(name.to_owned(), CachedBroadcastChannel::new(name));
        }
        self.channel_of_name
            .get(name)
            .expect("cached broadcast channel was inserted")
            .message_port
            .clone()
    }

    fn post_message(&mut self, name: &str, message: JsValue) -> bool {
        self.message_port_of_name(name);
        self.channel_of_name
            .get_mut(name)
            .expect("cached broadcast channel was inserted")
            .post(message)
    }

    fn mark_ready(&mut self, name: &str) {
        let release_requested = self
            .channel_of_name
            .get_mut(name)
            .map(CachedBroadcastChannel::mark_ready)
            .unwrap_or(false);
        if release_requested {
            self.release_ready_message_port_name(name);
        }
    }

    fn release_message_port_name(&mut self, name: &str) {
        let ready = self
            .channel_of_name
            .get_mut(name)
            .map(CachedBroadcastChannel::request_release)
            .unwrap_or(false);
        if ready {
            self.release_ready_message_port_name(name);
        }
    }

    fn release_ready_message_port_name(&mut self, name: &str) {
        if let Some(channel) = self.channel_of_name.remove(name) {
            self.close_message_port_later(channel);
        }
    }

    fn close_message_port_later(&mut self, channel: CachedBroadcastChannel) {
        self.pending_close.push(channel);
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
        for channel in self.pending_close.drain(..) {
            channel.close();
        }
    }
}

impl CachedBroadcastChannel {
    fn new(name: &str) -> Self {
        let message_port = PortLike::broadcast(name);
        let ready_port = PortLike::broadcast(&format!("{name}__frb_ready"));
        let ready_name = name.to_owned();
        let ready_callback =
            Closure::<dyn FnMut(web_sys::MessageEvent)>::new(move |_| mark_ready(&ready_name));
        ready_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(Some(ready_callback.as_ref().unchecked_ref()));
        Self {
            message_port,
            ready_port,
            _ready_callback: ready_callback,
            pending_messages: Vec::new(),
            ready: false,
            release_requested: false,
        }
    }

    fn post(&mut self, message: JsValue) -> bool {
        if !self.ready {
            self.pending_messages.push(message);
            return true;
        }
        self.message_port
            .post_message(&message)
            .map_err(|error| crate::console_error!("post broadcast channel: {:?}", error))
            .is_ok()
    }

    fn mark_ready(&mut self) -> bool {
        if !self.ready {
            self.ready = true;
            for message in self.pending_messages.drain(..) {
                if let Err(error) = self.message_port.post_message(&message) {
                    crate::console_error!("flush broadcast channel: {:?}", error);
                }
            }
        }
        self.release_requested
    }

    fn request_release(&mut self) -> bool {
        self.release_requested = true;
        self.ready
    }

    fn close(self) {
        if let Err(error) = self.message_port.close() {
            crate::console_error!("close broadcast channel: {:?}", error);
        }
        if let Err(error) = self.ready_port.close() {
            crate::console_error!("close ready channel: {:?}", error);
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

pub fn post_cached_message_port(handle: &SendableMessagePortHandle, message: JsValue) -> bool {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().post_message(&handle.0, message))
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

fn mark_ready(name: &str) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().mark_ready(name))
}
