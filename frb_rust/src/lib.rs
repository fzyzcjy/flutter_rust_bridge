// #![cfg_attr(nightly, feature(generic_const_exprs))]

pub use flutter_rust_bridge_macros::frb;
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use rust2dart::StreamSink;
use support::WireSyncReturnData;

pub mod ffi;
pub use ffi::*;

pub mod thread;

pub mod handler;
#[macro_use]
mod macros;
pub mod rust2dart;
pub mod support;

#[cfg(wasm)]
mod wasm_bindgen_src;

/// Use this struct in return type of your function, in order to tell the code generator
/// the function should return synchronously. Otherwise, it is by default asynchronously.
pub struct SyncReturn<T>(pub T)
where
    WireSyncReturnData: From<T>;
