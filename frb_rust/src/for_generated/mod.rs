//! Utilities to support the auto-generated Rust code.
//! These functions are usually *not* meant to be used by humans directly.

mod boilerplate;
mod cast;
mod pointer;

pub use crate::codec::dco::{transform_result_dco, Rust2DartMessageDco};
pub use crate::codec::sse::{
    Dart2RustMessageSse, Rust2DartMessageSse, SseDeserializer, SseSerializer,
};
pub use crate::codec::{cst::CstCodec, dco::DcoCodec, sse::SseCodec, BaseCodec};
pub use crate::dart_opaque::dart2rust::dart_opaque_dart2rust_encode;
pub use crate::dart_opaque::dart2rust::{cst_decode_dart_opaque, sse_decode_dart_opaque};
pub use crate::generalized_isolate::Channel;
pub use crate::generalized_isolate::IntoDartExceptPrimitive;
pub use crate::handler::handler::{handler_initialize, FfiCallMode, TaskInfo};
pub use crate::misc::manual_impl::*;
pub use crate::misc::rust_arc::{rust_arc_decrement_strong_count, rust_arc_increment_strong_count};
pub use crate::misc::rust_auto_opaque::rust_auto_opaque_encode;
pub use crate::platform_types::{
    DartAbi, MessagePort, PlatformGeneralizedUint8ListPtr, WireSyncRust2DartDco,
    WireSyncRust2DartSse,
};
pub use crate::rust2dart::action::Rust2DartAction;
pub use crate::rust2dart::stream_sink::StreamSinkBase;
pub use crate::rust_async::{BaseAsyncRuntime, SimpleAsyncRuntime};
pub use crate::rust_opaque::dart2rust::{cst_decode_rust_opaque, sse_decode_rust_opaque};
pub use crate::thread_pool::{BaseThreadPool, SimpleThreadPool};
#[cfg(wasm)]
pub use crate::web_transfer::transfer_closure::TransferClosure;
pub use byteorder;
pub use cast::*;
#[cfg(not(wasm))]
pub use dart_sys;
pub use futures;
#[cfg(wasm)]
pub use js_sys;
pub use lazy_static::lazy_static;
pub use pointer::*;
#[cfg(wasm)]
pub use wasm_bindgen;
