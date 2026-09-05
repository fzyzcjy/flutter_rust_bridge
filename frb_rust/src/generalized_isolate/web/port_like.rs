use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use web_sys::{MessageChannel, MessageEvent, MessagePort};

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

pub(crate) fn create_message_router() -> Result<MessagePort, JsValue> {
    let channel = MessageChannel::new()?;
    match message_router()? {
        Some(router) => router
            .post_message_with_transferable(&channel.port1(), &Array::of1(&channel.port1()))?,
        None => receive_routed_messages(channel.port1()),
    }
    Ok(channel.port2())
}

fn post_named_message(name: &str, value: &JsValue) -> Result<(), JsValue> {
    if let Some(router) = message_router()? {
        return router.post_message(&Array::of2(&name.into(), value));
    }
    let ports = Reflect::get(&js_sys::global(), &"__frb_named_ports".into())?;
    if !ports.is_undefined() && !ports.is_null() {
        let port = Reflect::get(&ports, &name.into())?;
        if !port.is_undefined() {
            return port.unchecked_into::<PortLike>().post_message_raw(value);
        }
    }
    Ok(())
}

fn message_router() -> Result<Option<MessagePort>, JsValue> {
    let router = Reflect::get(&js_sys::global(), &"__frb_message_router".into())?;
    Ok((!router.is_undefined()).then(|| router.unchecked_into()))
}

fn receive_routed_messages(port: MessagePort) {
    let callback = Closure::<dyn FnMut(MessageEvent)>::new(|event: MessageEvent| {
        let data = event.data();
        if let Some(port) = data.dyn_ref::<MessagePort>() {
            receive_routed_messages(port.clone());
        } else {
            let data = data.unchecked_ref::<Array>();
            post_named_message(&data.get(0).as_string().unwrap_throw(), &data.get(1))
                .unwrap_throw();
        }
    });
    port.set_onmessage(Some(callback.as_ref().unchecked_ref()));
    let _ = callback.into_js_value();
}
