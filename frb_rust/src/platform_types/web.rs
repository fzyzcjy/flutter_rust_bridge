use crate::generalized_isolate::PortLike;
use js_sys::{Array, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BroadcastChannel, MessageChannel, MessageEvent};

pub type WireSyncRust2DartDco = wasm_bindgen::JsValue;
pub type WireSyncRust2DartSse = wasm_bindgen::JsValue;

pub type MessagePort = crate::generalized_isolate::PortLike;

pub type DartAbi = wasm_bindgen::JsValue;

const BROADCAST_CHANNEL_CLOSE_DELAY_MILLIS: i32 = 100;

#[derive(Clone, Debug)]
pub struct SendableMessagePortHandle(String);

/// Set by the worker bootstrap script in `worker_pool.rs`.
const RUST2DART_RELAY_GLOBAL_KEY: &str = "__frbRust2DartRelay";

const RELAY_ACTION_POST: u32 = 0;
const RELAY_ACTION_RELEASE: u32 = 1;
const RELAY_ACTION_ADOPT_PORT: u32 = 2;

thread_local! {
    static BROADCAST_CHANNEL_STATE: RefCell<BroadcastChannelState> = RefCell::new(BroadcastChannelState::new());

    /// Only present inside pool workers: a port whose other end is on the main thread.
    static RUST2DART_RELAY: Option<web_sys::MessagePort> =
        Reflect::get(&js_sys::global(), &RUST2DART_RELAY_GLOBAL_KEY.into())
            .ok()
            .and_then(|relay| relay.dyn_into().ok());

    static RUST2DART_RELAY_HANDLER: Closure<dyn FnMut(MessageEvent)> =
        Closure::new(|event: MessageEvent| receive_relay_message(event.data()));
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
    let release = Array::of2(&RELAY_ACTION_RELEASE.into(), &handle.0.as_str().into());
    if let Some(Err(error)) = relay_to_main_thread(&release, None) {
        crate::console_error!("relay broadcast channel release: {:?}", error);
    }
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().release_message_port_name(&handle.0))
}

/// Posts `msg` to `port`.
///
/// Inside a pool worker, posts to a `BroadcastChannel` are relayed to the main thread, which
/// re-posts them on its own channel. A worker's fresh `BroadcastChannel` registers with the
/// browser asynchronously and independently of the main thread, so its first messages can
/// overtake the registration of the Dart receiver and be silently dropped.
///
/// Messages keep their order per sending thread only: posts to one channel from different
/// threads travel through different ports, as before.
pub fn post_message_to_port(port: &MessagePort, msg: &JsValue) -> Result<(), JsValue> {
    if let Some(channel) = port.dyn_ref::<BroadcastChannel>() {
        let post = Array::of3(&RELAY_ACTION_POST.into(), &channel.name().into(), msg);
        if let Some(result) = relay_to_main_thread(&post, None) {
            return result;
        }
    }
    port.post_message(msg)
}

/// Creates the port that a new pool worker relays its Rust-to-Dart messages through.
///
/// The other end is listened to by the thread hosting the outermost pool, i.e. the main thread
/// in practice; workers spawned by pool workers hand their end over to it, so relayed messages
/// never wait for a busy intermediate worker.
pub fn create_rust2dart_relay_port() -> Result<web_sys::MessagePort, JsValue> {
    let channel = MessageChannel::new()?;
    let main_thread_end = channel.port1();
    let adopt = Array::of2(&RELAY_ACTION_ADOPT_PORT.into(), &main_thread_end);
    match relay_to_main_thread(&adopt, Some(&Array::of1(&main_thread_end))) {
        Some(result) => result?,
        None => listen_to_relay_port(&main_thread_end),
    }
    Ok(channel.port2())
}

/// Returns `None` when not running inside a pool worker.
fn relay_to_main_thread(message: &Array, transfer: Option<&Array>) -> Option<Result<(), JsValue>> {
    RUST2DART_RELAY.with(|relay| {
        relay.as_ref().map(|relay| match transfer {
            Some(transfer) => relay.post_message_with_transferable(message, transfer),
            None => relay.post_message(message),
        })
    })
}

fn listen_to_relay_port(port: &web_sys::MessagePort) {
    RUST2DART_RELAY_HANDLER
        .with(|handler| port.set_onmessage(Some(handler.as_ref().unchecked_ref())));
}

fn receive_relay_message(data: JsValue) {
    let data: Array = data.unchecked_into();
    let action = data.get(0).as_f64().map(|action| action as u32);
    let arg = data.get(1);
    match action {
        Some(RELAY_ACTION_POST) | Some(RELAY_ACTION_RELEASE) => {
            let Some(name) = arg.as_string() else {
                crate::console_error!("relayed message without channel name: {:?}", data);
                return;
            };
            let handle = SendableMessagePortHandle(name);
            if action == Some(RELAY_ACTION_RELEASE) {
                release_message_port_handle(&handle);
            } else if let Err(error) =
                post_message_to_port(&handle_to_message_port(&handle), &data.get(2))
            {
                crate::console_error!("post relayed message: {:?}", error);
            }
        }
        Some(RELAY_ACTION_ADOPT_PORT) => listen_to_relay_port(&arg.unchecked_into()),
        _ => crate::console_error!("unknown relayed message: {:?}", data),
    }
}

pub fn deserialize_sendable_message_port_handle(raw: String) -> SendableMessagePortHandle {
    SendableMessagePortHandle(raw)
}

pub type PlatformGeneralizedUint8ListPtr = wasm_bindgen::JsValue;

fn close_pending_message_ports() {
    BROADCAST_CHANNEL_STATE.with(|state| state.borrow_mut().close_pending_message_ports())
}
