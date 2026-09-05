use wasm_bindgen::prelude::*;
use js_sys::{Array, Object, Reflect};
use web_sys::{DedicatedWorkerGlobalScope, MessageEvent, Worker};

#[wasm_bindgen]
extern "C" {
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

pub(crate) fn install_message_forwarding(worker: &Worker) -> Result<(), JsValue> {
    let callback = Closure::<dyn FnMut(MessageEvent)>::new(|event: MessageEvent| {
        let data = event.data();
        if !Array::is_array(&data) {
            return;
        }
        let data = Array::from(&data);
        if data.length() != 3 || data.get(0).as_string().as_deref() != Some("__frb_named_message") {
            return;
        }
        event.stop_immediate_propagation();
        post_named_message(&data.get(1).as_string().unwrap_throw(), &data.get(2)).unwrap_throw();
    });
    worker.add_event_listener_with_callback("message", callback.as_ref().unchecked_ref())?;
    let _ = callback.into_js_value();
    Ok(())
}

fn post_named_message(name: &str, value: &JsValue) -> Result<(), JsValue> {
    let global = js_sys::global();
    if Reflect::get(&global, &"__frb_rust_worker".into())?.is_truthy() {
        return global
            .unchecked_into::<DedicatedWorkerGlobalScope>()
            .post_message(&Array::of3(&"__frb_named_message".into(), &name.into(), value));
    }
    let ports = Reflect::get(&global, &"__frb_named_ports".into())?;
    if !ports.is_undefined() && !ports.is_null() {
        let port = Reflect::get(&ports, &name.into())?;
        if !port.is_undefined() {
            return port.unchecked_into::<PortLike>().post_message_raw(value);
        }
    }
    Ok(())
}
