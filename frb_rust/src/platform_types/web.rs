use crate::generalized_isolate::PortLike;
use js_sys::{Function, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BroadcastChannel, MessageEvent};

pub type WireSyncRust2DartDco = wasm_bindgen::JsValue;
pub type WireSyncRust2DartSse = wasm_bindgen::JsValue;

pub type MessagePort = crate::generalized_isolate::PortLike;

pub type DartAbi = wasm_bindgen::JsValue;

const BROADCAST_CHANNEL_RELEASE_SUFFIX: &str = "__flutter_rust_bridge_release";
const BROADCAST_CHANNEL_READY_SUFFIX: &str = "__flutter_rust_bridge_ready";
const MAX_READINESS_PROBE_DELAY_MILLIS: i32 = 1000;
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
    ready_port: MessagePort,
    _release_callback: Closure<dyn FnMut()>,
    _ready_callback: Closure<dyn FnMut(MessageEvent)>,
    pending_messages: Vec<JsValue>,
    ready: bool,
    readiness_nonce: String,
    readiness_probe_callback: Option<Closure<dyn FnMut()>>,
    readiness_probe_timer: Option<i32>,
    next_readiness_probe_delay_millis: i32,
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
            channel.mark_ready();
        }
    }

    fn probe_message_port_name(&mut self, name: &str) {
        if let Some(channel) = self.channel_of_name.get_mut(name) {
            channel.probe_readiness();
        }
    }
}

impl CachedBroadcastChannel {
    fn new(name: &str) -> Self {
        let message_port = PortLike::broadcast(name);
        let release_port =
            PortLike::broadcast(&format!("{name}{BROADCAST_CHANNEL_RELEASE_SUFFIX}"));
        let ready_port = PortLike::broadcast(&format!("{name}{BROADCAST_CHANNEL_READY_SUFFIX}"));
        let name = name.to_owned();
        let release_name = name.clone();
        let release_callback = Closure::wrap(Box::new(move || {
            release_message_port_name_locally(&release_name);
        }) as Box<dyn FnMut()>);
        release_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(Some(release_callback.as_ref().unchecked_ref()));

        let readiness_nonce = readiness_nonce();
        let ready_name = name.clone();
        let expected_nonce = readiness_nonce.clone();
        let ready_callback = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            if event.data() == JsValue::from_str(&expected_nonce) {
                flush_message_port_name(&ready_name);
            }
        });
        ready_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(Some(ready_callback.as_ref().unchecked_ref()));

        Self {
            message_port,
            release_port,
            ready_port,
            _release_callback: release_callback,
            _ready_callback: ready_callback,
            pending_messages: Vec::new(),
            ready: false,
            readiness_nonce,
            readiness_probe_callback: None,
            readiness_probe_timer: None,
            next_readiness_probe_delay_millis: 0,
        }
    }

    fn post(&mut self, message: JsValue) -> bool {
        if self.ready {
            return self
                .message_port
                .post_message(&message)
                .map_err(|error| {
                    crate::console_error!("post cached broadcast channel: {:?}", error)
                })
                .is_ok();
        }

        let Some(message) = clone_for_deferred_post(&message) else {
            return false;
        };
        self.pending_messages.push(message);
        if self.readiness_probe_callback.is_none() {
            let name = self
                .message_port
                .dyn_ref::<BroadcastChannel>()
                .expect("Not a BroadcastChannel")
                .name();
            let callback = Closure::wrap(Box::new(move || {
                probe_message_port_name(&name);
            }) as Box<dyn FnMut()>);
            self.readiness_probe_callback = Some(callback);
            self.schedule_readiness_probe();
        }
        true
    }

    fn mark_ready(&mut self) {
        if self.ready {
            return;
        }
        self.ready = true;
        self.cancel_readiness_probe();
        for message in mem::take(&mut self.pending_messages) {
            self.message_port
                .post_message(&message)
                .expect("ready broadcast channel rejects a pre-cloned message");
        }
    }

    fn probe_readiness(&mut self) {
        self.readiness_probe_timer = None;
        if self.ready {
            return;
        }
        if let Err(error) = self
            .ready_port
            .post_message(&JsValue::from_str(&self.readiness_nonce))
        {
            crate::console_error!("probe broadcast channel readiness: {:?}", error);
        }
        if !self.ready {
            self.schedule_readiness_probe();
        }
    }

    fn close(mut self) {
        self.cancel_readiness_probe();
        self.release_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(None);
        self.ready_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(None);

        for port in [self.message_port, self.release_port, self.ready_port] {
            if let Err(error) = port.close() {
                crate::console_error!("close broadcast channel: {:?}", error);
            }
        }
    }

    fn schedule_readiness_probe(&mut self) {
        if self.ready || self.readiness_probe_timer.is_some() {
            return;
        }
        let callback = self
            .readiness_probe_callback
            .as_ref()
            .expect("readiness probe callback was initialized");
        self.readiness_probe_timer = Some(schedule_timeout(
            callback,
            self.next_readiness_probe_delay_millis,
        ));
        self.next_readiness_probe_delay_millis = self
            .next_readiness_probe_delay_millis
            .max(1)
            .saturating_mul(2)
            .min(MAX_READINESS_PROBE_DELAY_MILLIS);
    }

    fn cancel_readiness_probe(&mut self) {
        if let Some(timer) = self.readiness_probe_timer.take() {
            clear_timeout(timer);
        }
        self.readiness_probe_callback = None;
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

fn probe_message_port_name(name: &str) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().probe_message_port_name(name))
}

fn clone_for_deferred_post(message: &JsValue) -> Option<JsValue> {
    let global = js_sys::global();
    let structured_clone = Reflect::get(&global, &JsValue::from_str("structuredClone"))
        .expect("structuredClone is unavailable")
        .unchecked_into::<Function>();
    structured_clone
        .call1(&global, message)
        .map_err(|error| {
            crate::console_error!("clone deferred broadcast channel message: {:?}", error);
        })
        .ok()
}

fn readiness_nonce() -> String {
    format!("{}-{}", js_sys::Date::now(), js_sys::Math::random())
}

fn schedule_timeout(callback: &Closure<dyn FnMut()>, delay_millis: i32) -> i32 {
    let global = js_sys::global();
    let set_timeout = Reflect::get(&global, &JsValue::from_str("setTimeout"))
        .expect("setTimeout is unavailable")
        .unchecked_into::<Function>();
    set_timeout
        .call2(
            &global,
            callback.as_ref().unchecked_ref(),
            &JsValue::from(delay_millis),
        )
        .expect("setTimeout failed")
        .as_f64()
        .expect("setTimeout returned a timer id") as i32
}

fn clear_timeout(timer: i32) {
    let global = js_sys::global();
    let clear_timeout = Reflect::get(&global, &JsValue::from_str("clearTimeout"))
        .expect("clearTimeout is unavailable")
        .unchecked_into::<Function>();
    clear_timeout
        .call1(&global, &JsValue::from(timer))
        .expect("clearTimeout failed");
}
