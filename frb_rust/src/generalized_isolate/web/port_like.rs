use js_sys::{Array, Object, Reflect};
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::BroadcastChannel;

thread_local! {
    static BROADCAST_CHANNELS: RefCell<HashMap<String, BroadcastChannel>> = RefCell::new(HashMap::new());
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = queueMicrotask)]
    fn queue_microtask(callback: &js_sys::Function);

    /// Objects implementing the interface of [`web_sys::MessagePort`].
    ///
    /// Attempts to coerce [`JsValue`]s into this interface using [`dyn_into`][JsCast::dyn_into]
    /// or [`dyn_ref`][JsCast::dyn_ref] will fail at runtime.
    #[derive(Clone)]
    pub type PortLike;
    #[wasm_bindgen(method, catch, js_name = "postMessage")]
    fn post_message_raw(this: &PortLike, value: &JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch)]
    #[wasm_bindgen(js_name = close)]
    fn close_raw(this: &PortLike) -> Result<(), JsValue>;
    #[wasm_bindgen(method, getter, js_name = __frb_port_name)]
    pub(crate) fn channel_name(this: &PortLike) -> Option<String>;
}

impl PortLike {
    /// Create a message port handle with the specified name.
    pub fn broadcast(name: &str) -> Self {
        if !name.starts_with("__frb_broadcast_") {
            return BroadcastChannel::new(name).unwrap_throw().unchecked_into();
        }
        let port = Object::new();
        Reflect::set(&port, &"__frb_port_name".into(), &name.into()).unwrap_throw();
        port.unchecked_into()
    }

    pub fn post_message(&self, value: &JsValue) -> Result<(), JsValue> {
        match self.channel_name() {
            Some(name) => post_named_message(&name, value),
            None => self.post_message_raw(value),
        }
    }

    pub fn close(&self) -> Result<(), JsValue> {
        match self.channel_name() {
            Some(_) => Ok(()),
            None => self.close_raw(),
        }
    }
}

fn post_named_message(name: &str, value: &JsValue) -> Result<(), JsValue> {
    let (channel_name, port_name) = name.split_once('/').ok_or_else(|| JsValue::from_str("Invalid broadcast port name"))?;
    let channel = BROADCAST_CHANNELS.with(|channels| {
        let mut channels = channels.borrow_mut();
        if channels.is_empty() {
            queue_microtask(Closure::once_into_js(close_broadcast_channels).unchecked_ref());
        }
        let channel = match channels.entry(channel_name.to_owned()) {
            std::collections::hash_map::Entry::Occupied(entry) => entry.into_mut(),
            std::collections::hash_map::Entry::Vacant(entry) => entry.insert(BroadcastChannel::new(channel_name)?),
        };
        Ok::<_, JsValue>(channel.clone())
    })?;
    channel.post_message(&Array::of2(&port_name.into(), value))
}

fn close_broadcast_channels() {
    BROADCAST_CHANNELS.with(|channels| {
        for (_, channel) in channels.borrow_mut().drain() {
            channel.close();
        }
    });
}
