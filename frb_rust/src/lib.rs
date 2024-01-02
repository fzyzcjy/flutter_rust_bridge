//! Main documentation is in <https://github.com/fzyzcjy/flutter_rust_bridge>

mod generalized_isolate;
pub(crate) mod handler;
mod misc;
mod platform_types;
mod rust2dart;
pub(crate) mod third_party;
pub(crate) mod thread_pool;

pub(crate) mod codec;
pub(crate) mod dart_fn;
pub(crate) mod dart_opaque;
pub(crate) mod ffi_binding;
#[doc(hidden)]
pub mod for_generated;
pub(crate) mod rust_async;
pub(crate) mod rust_opaque;
pub(crate) mod web_transfer;

pub use crate::codec::sse::SseCodec;
pub use crate::dart_fn::DartFnFuture;
pub use crate::dart_opaque::DartOpaque;
pub use crate::generalized_isolate::{IntoDart, ZeroCopyBuffer};
pub use crate::handler::handler::Handler;
pub use crate::handler::implementation::handler::DefaultHandler;
pub use crate::misc::dart_dynamic::DartDynamic;
pub use crate::misc::into_into_dart::IntoIntoDart;
pub use crate::misc::user_utils;
pub use crate::rust_async::{spawn, spawn_blocking_with, spawn_local, JoinHandle};
pub use crate::rust_opaque::{DartSafe, RustOpaque};
pub use flutter_rust_bridge_macros::frb;
