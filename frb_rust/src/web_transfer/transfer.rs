use crate::generalized_isolate::PortLike;
use js_sys::ArrayBuffer;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::BroadcastChannel;

/// Internal implementations for transferables on WASM platforms.
pub trait Transfer {
    /// Recover the self value from a [JsValue].
    fn deserialize(value: &JsValue) -> Self;
    /// Transform the self value into a [JsValue].
    fn serialize(self) -> JsValue;
    /// Extract items that are valid to be passed as the "transfer" argument.
    fn transferables(&self) -> Vec<JsValue>;
}

impl<T: Transfer> Transfer for Option<T> {
    fn deserialize(value: &JsValue) -> Self {
        (!value.is_undefined() && !value.is_null()).then(|| T::deserialize(value))
    }
    fn serialize(self) -> JsValue {
        self.map(T::serialize).unwrap_or_default()
    }
    fn transferables(&self) -> Vec<JsValue> {
        self.as_ref().map(T::transferables).unwrap_or_default()
    }
}

impl Transfer for PortLike {
    fn deserialize(value: &JsValue) -> Self {
        if let Some(name) = value.as_string() {
            BroadcastChannel::new(&name).unwrap().unchecked_into()
        } else if value.dyn_ref::<web_sys::MessagePort>().is_some() {
            value.unchecked_ref::<Self>().clone()
        } else {
            panic!("Not a PortLike: {:?}", value)
        }
    }
    fn serialize(self) -> JsValue {
        if let Some(channel) = self.dyn_ref::<BroadcastChannel>() {
            channel.name().into()
        } else {
            self.into()
        }
    }
    fn transferables(&self) -> Vec<JsValue> {
        if let Some(port) = self.dyn_ref::<web_sys::MessagePort>() {
            vec![port.clone().into()]
        } else {
            vec![]
        }
    }
}

impl Transfer for ArrayBuffer {
    fn deserialize(value: &JsValue) -> Self {
        value.dyn_ref().cloned().unwrap()
    }
    fn serialize(self) -> JsValue {
        self.into()
    }
    fn transferables(&self) -> Vec<JsValue> {
        vec![self.into()]
    }
}
