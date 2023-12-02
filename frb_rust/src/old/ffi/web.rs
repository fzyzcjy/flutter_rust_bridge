use super::DartAbi;
use super::MessagePort;
use crate::support;
pub use crate::wasm_bindgen_src::transfer::*;
pub use crate::wasm_bindgen_src::transfer::*;
use crate::DartSafe;
use crate::RustOpaque;
pub use js_sys;
pub use js_sys::Array as JsArray;
use js_sys::*;
use std::iter::FromIterator;
pub use wasm_bindgen;
pub use wasm_bindgen::closure::Closure;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    pub fn error(msg: &str);
}

#[derive(Debug)]
pub struct ZeroCopyBuffer<T>(pub T);

impl<T> ZeroCopyBuffer<Vec<T>> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

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

#[wasm_bindgen]
extern "C" {
    /// Objects implementing the interface of [`web_sys::MessagePort`].
    ///
    /// Attempts to coerce [`JsValue`]s into this interface using [`dyn_into`][JsCast::dyn_into]
    /// or [`dyn_ref`][JsCast::dyn_ref] will fail at runtime.
    #[derive(Clone)]
    pub type PortLike;
    #[wasm_bindgen(method, catch, js_name = "postMessage")]
    pub fn post_message(this: &PortLike, value: &JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch)]
    pub fn close(this: &PortLike) -> Result<(), JsValue>;
}

impl PortLike {
    /// Create a [`BroadcastChannel`] with the specified name.
    pub fn broadcast(name: &str) -> Self {
        BroadcastChannel::new(name)
            .expect("Failed to create broadcast channel")
            .unchecked_into()
    }
}

/// Copied from https://github.com/chemicstry/wasm_thread/blob/main/src/script_path.js
pub fn script_path() -> Option<String> {
    js_sys::eval(
        r"
(() => {
    try {
        throw new Error();
    } catch (e) {
        let parts = e.stack.match(/(?:\(|@)(\S+):\d+:\d+/);
        return parts[1];
    }
})()",
    )
    .ok()?
    .as_string()
}

/// # Safety
///
/// TODO: need doc
#[wasm_bindgen]
pub unsafe fn get_dart_object(ptr: usize) -> JsValue {
    *support::box_from_leak_ptr(ptr as _)
}

/// # Safety
///
/// TODO: need doc
#[wasm_bindgen]
pub unsafe fn drop_dart_object(ptr: usize) {
    drop(support::box_from_leak_ptr::<JsValue>(ptr as _));
}

