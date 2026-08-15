use crate::generalized_isolate::PortLike;
use js_sys::{Function, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

pub type WireSyncRust2DartDco = wasm_bindgen::JsValue;
pub type WireSyncRust2DartSse = wasm_bindgen::JsValue;

pub type MessagePort = crate::generalized_isolate::PortLike;

pub type DartAbi = wasm_bindgen::JsValue;

const BROADCAST_CHANNEL_RELEASE_SUFFIX: &str = "__flutter_rust_bridge_release";
#[derive(Clone, Debug)]
pub struct SendableMessagePortHandle(String);

thread_local! {
    static BROADCAST_CHANNEL_STATE: RefCell<BroadcastChannelState> = RefCell::new(BroadcastChannelState::new());
}

struct BroadcastChannelState {
    channel_of_name: HashMap<String, CachedBroadcastChannel>,
}

struct CachedBroadcastChannel {
    message_port: MessagePort,
    release_port: MessagePort,
    _release_callback: Closure<dyn FnMut()>,
    pending_messages: Vec<JsValue>,
    flush_callback: Option<Closure<dyn FnMut()>>,
}

impl BroadcastChannelState {
    fn new() -> Self {
        Self {
            channel_of_name: Default::default(),
        }
    }

    fn release_message_port_name(&mut self, name: &str) {
        if let Some(channel) = self.channel_of_name.get(name) {
            if let Err(error) = channel.release_port.post_message(&JsValue::NULL) {
                crate::console_error!("broadcast channel release: {:?}", error);
            }
        }

        self.release_message_port_name_locally(name);
    }

    fn release_message_port_name_locally(&mut self, name: &str) {
        if let Some(channel) = self.channel_of_name.remove(name) {
            channel.close();
        }
    }

    fn post_message(&mut self, name: &str, message: JsValue) -> bool {
        if !self.channel_of_name.contains_key(name) {
            self.channel_of_name
                .insert(name.to_owned(), CachedBroadcastChannel::new(name));
        }
        self.channel_of_name
            .get_mut(name)
            .expect("cached broadcast channel was inserted")
            .post(message)
    }

    fn flush_message_port_name(&mut self, name: &str) {
        if let Some(channel) = self.channel_of_name.get_mut(name) {
            channel.flush();
        }
    }
}

impl CachedBroadcastChannel {
    fn new(name: &str) -> Self {
        let message_port = PortLike::broadcast(name);
        let release_port =
            PortLike::broadcast(&format!("{name}{BROADCAST_CHANNEL_RELEASE_SUFFIX}"));
        let name = name.to_owned();
        let release_callback = Closure::wrap(Box::new(move || {
            release_message_port_name_locally(&name);
        }) as Box<dyn FnMut()>);
        release_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(Some(release_callback.as_ref().unchecked_ref()));

        Self {
            message_port,
            release_port,
            _release_callback: release_callback,
            pending_messages: Vec::new(),
            flush_callback: None,
        }
    }

    fn post(&mut self, message: JsValue) -> bool {
        self.pending_messages.push(message);
        if self.flush_callback.is_none() {
            let name = self
                .message_port
                .dyn_ref::<BroadcastChannel>()
                .expect("Not a BroadcastChannel")
                .name();
            let callback = Closure::wrap(Box::new(move || {
                flush_message_port_name(&name);
            }) as Box<dyn FnMut()>);
            schedule_timeout(&callback);
            self.flush_callback = Some(callback);
        }
        true
    }

    fn flush(&mut self) {
        self.flush_callback = None;
        for message in mem::take(&mut self.pending_messages) {
            if let Err(error) = self.message_port.post_message(&message) {
                crate::console_error!("post cached broadcast channel: {:?}", error);
            }
        }
    }

    fn close(self) {
        self.release_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(None);

        for port in [self.message_port, self.release_port] {
            if let Err(error) = port.close() {
                crate::console_error!("close broadcast channel: {:?}", error);
            }
        }
    }
}

pub fn message_port_to_handle(port: &MessagePort) -> SendableMessagePortHandle {
    SendableMessagePortHandle(
        port.dyn_ref::<BroadcastChannel>()
            .map(|channel| channel.name())
            .expect("Not a BroadcastChannel"),
    )
}

pub fn handle_to_message_port(handle: &SendableMessagePortHandle) -> MessagePort {
    PortLike::broadcast(&handle.0)
}

pub fn post_cached_message_port(handle: &SendableMessagePortHandle, message: JsValue) -> bool {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().post_message(&handle.0, message))
}

pub fn release_cached_message_port_handle(handle: &SendableMessagePortHandle) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().release_message_port_name(&handle.0))
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;

fn release_message_port_name_locally(name: &str) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().release_message_port_name_locally(name))
}

fn flush_message_port_name(name: &str) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().flush_message_port_name(name))
}

fn schedule_timeout(callback: &Closure<dyn FnMut()>) {
    let global = js_sys::global();
    let set_timeout = Reflect::get(&global, &JsValue::from_str("setTimeout"))
        .expect("setTimeout is unavailable")
        .unchecked_into::<Function>();
    set_timeout
        .call2(
            &global,
            callback.as_ref().unchecked_ref(),
            &JsValue::from(0),
        )
        .expect("setTimeout failed");
}
