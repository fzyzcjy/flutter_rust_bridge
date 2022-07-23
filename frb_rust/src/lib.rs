pub use flutter_rust_bridge_macros::frb;
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use rust2dart::StreamSink;
#[cfg(target_family = "wasm")]
pub use wasm_bindgen;
#[cfg(target_family = "wasm")]
pub use wasm_bindgen::prelude::*;

pub mod ffi;
pub use ffi::*;
pub mod handler;
#[cfg(target_family = "wasm")]
mod pool;
pub mod rust2dart;
pub mod support;

/// Use this struct in return type of your function, in order to tell the code generator
/// the function should return synchronously. Otherwise, it is by default asynchronously.
pub struct SyncReturn<T>(pub T);
