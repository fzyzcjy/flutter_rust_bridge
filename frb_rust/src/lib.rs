use std::panic::{RefUnwindSafe, UnwindSafe};

pub use allo_isolate::ZeroCopyBuffer;

pub use flutter_rust_bridge_macros::frb;
pub use handler::{FfiCallMode, Handler, WrapInfo};
pub use opaque::Opaque;
pub use rust2dart::StreamSink;

pub mod handler;
pub mod opaque;
pub mod rust2dart;
pub mod support;

/// Use this struct in return type of your function, in order to tell the code generator
/// the function should return synchronously. Otherwise, it is by default asynchronously.
pub struct SyncReturn<T>(pub T);

/// Marker trait representing types that exhibit behavior that are safe to be serialized as
/// Dart objects to send to the Dart VM.
pub trait DartSafe: Send + Sync + UnwindSafe + RefUnwindSafe {}
impl<T: Send + Sync + UnwindSafe + RefUnwindSafe> DartSafe for T {}
