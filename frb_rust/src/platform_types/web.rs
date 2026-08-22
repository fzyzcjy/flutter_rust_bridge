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
const BROADCAST_CHANNEL_READY: &str = "__flutter_rust_bridge_ready";
const BROADCAST_CHANNEL_READY_RETRY_MILLIS: i32 = 10;

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
    _ready_callback: Closure<dyn FnMut(web_sys::MessageEvent)>,
    pending_messages: Vec<JsValue>,
    ready: bool,
    release_requested: bool,
    readiness_probe_callback: Option<Closure<dyn FnMut()>>,
    readiness_probe_timer: Option<i32>,
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

    fn mark_message_port_ready(&mut self, name: &str) {
        let release_requested = self
            .channel_of_name
            .get_mut(name)
            .map(CachedBroadcastChannel::mark_ready)
            .unwrap_or(false);
        if release_requested {
            self.release_ready_message_port_name(name);
        }
    }

    fn probe_message_port_name(&mut self, name: &str) {
        if let Some(channel) = self.channel_of_name.get_mut(name) {
            channel.probe_readiness();
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
        let ready_name = name.to_owned();
        let ready_callback = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MessageEvent| {
            if event.data() == JsValue::from_str(BROADCAST_CHANNEL_READY) {
                mark_message_port_ready(&ready_name);
            }
        });
        message_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(Some(ready_callback.as_ref().unchecked_ref()));

        Self {
            message_port,
            _ready_callback: ready_callback,
            pending_messages: Vec::new(),
            ready: false,
            release_requested: false,
            readiness_probe_callback: None,
            readiness_probe_timer: None,
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

        self.pending_messages.push(message);
        if self.readiness_probe_callback.is_none() {
            let name = self
                .message_port
                .dyn_ref::<BroadcastChannel>()
                .expect("Not a BroadcastChannel")
                .name();
            self.readiness_probe_callback = Some(Closure::wrap(Box::new(move || {
                probe_message_port_name(&name);
            }) as Box<dyn FnMut()>));
            self.schedule_readiness_probe();
        }
        true
    }

    fn mark_ready(&mut self) -> bool {
        if self.ready {
            return self.release_requested;
        }
        self.ready = true;
        self.cancel_readiness_probe();
        for message in self.pending_messages.drain(..) {
            self.message_port
                .post_message(&message)
                .expect("ready broadcast channel rejects a pre-cloned message");
        }
        self.release_requested
    }

    fn probe_readiness(&mut self) {
        self.readiness_probe_timer = None;
        if self.ready {
            return;
        }
        if let Err(error) = self
            .message_port
            .post_message(&JsValue::from_str(BROADCAST_CHANNEL_READY))
        {
            crate::console_error!("probe broadcast channel readiness: {:?}", error);
        }
        self.schedule_readiness_probe();
    }

    fn request_release(&mut self) -> bool {
        self.release_requested = true;
        self.ready
    }

    fn schedule_readiness_probe(&mut self) {
        if self.ready || self.readiness_probe_timer.is_some() {
            return;
        }
        let callback = self
            .readiness_probe_callback
            .as_ref()
            .expect("readiness probe callback was initialized");
        self.readiness_probe_timer = Some(
            js_set_timeout(
                callback.as_ref().unchecked_ref(),
                BROADCAST_CHANNEL_READY_RETRY_MILLIS,
            )
            .expect("schedule broadcast channel readiness probe")
            .as_f64()
            .expect("setTimeout returned a timer id") as i32,
        );
    }

    fn cancel_readiness_probe(&mut self) {
        if let Some(timer) = self.readiness_probe_timer.take() {
            clear_timeout(timer);
        }
        self.readiness_probe_callback = None;
    }

    fn close(mut self) {
        self.cancel_readiness_probe();
        self.message_port
            .dyn_ref::<BroadcastChannel>()
            .expect("Not a BroadcastChannel")
            .set_onmessage(None);
        if let Err(error) = self.message_port.close() {
            crate::console_error!("close broadcast channel: {:?}", error);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTimeout, catch)]
    fn js_set_timeout(handler: &js_sys::Function, timeout: i32) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = clearTimeout)]
    fn js_clear_timeout(timer: i32);
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

fn mark_message_port_ready(name: &str) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().mark_message_port_ready(name))
}

fn probe_message_port_name(name: &str) {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().probe_message_port_name(name))
}

fn clear_timeout(timer: i32) {
    js_clear_timeout(timer);
}
