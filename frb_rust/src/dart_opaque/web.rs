use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
use crate::generalized_isolate::Channel;
use crate::generalized_isolate::PortLike;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::BroadcastChannel;

pub type GeneralizedAutoDropDartPersistentHandle = wasm_bindgen::JsValue;
