pub use flutter_rust_bridge_macros::frb;
pub use handler::{FfiCallMode, Handler, TaskInfo};
pub use rust2dart::StreamSink;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub mod ffi;
pub use ffi::*;

pub mod thread;

pub mod handler;
#[macro_use]
mod macros;
mod into_into_dart;
pub mod rust2dart;
pub mod support;

#[cfg(feature = "rust-async")]
mod rust_async;

#[cfg(wasm)]
mod wasm_bindgen_src;

#[cfg(wasm)]
pub use wasm_bindgen;
