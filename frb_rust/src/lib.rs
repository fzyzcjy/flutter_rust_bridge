pub use flutter_rust_bridge_macros::frb;
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use opaque::Opaque;
pub use rust2dart::StreamSink;
use std::panic::{RefUnwindSafe, UnwindSafe};
use support::WireSyncReturnData;

pub mod ffi;
pub use ffi::*;

pub mod thread;

pub mod handler;
pub mod opaque;
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

/// Marker trait for types that are safe to share with Dart and can be dropped
/// safely in case of a panic.
pub trait DartSafe: Send + Sync + UnwindSafe + RefUnwindSafe {}
impl<T: Send + Sync + UnwindSafe + RefUnwindSafe> DartSafe for T {}
