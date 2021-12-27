pub use allo_isolate::ZeroCopyBuffer;

pub use core::ffi::c_void;
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use opaque::Opaque;
pub use rust2dart::StreamSink;
pub use std::sync::Arc;

pub mod handler;
pub mod opaque;
pub mod rust2dart;
pub mod support;

/// Use this struct in return type of your function, in order to tell the code generator
/// the function should return synchronously. Otherwise, it is by default asynchronously.
pub struct SyncReturn<T>(pub T);
