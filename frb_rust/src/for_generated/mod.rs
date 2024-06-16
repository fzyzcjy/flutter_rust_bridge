//! Utilities to support the auto-generated Rust code.
//! These functions are usually *not* meant to be used by humans directly.

mod boilerplate;
mod boilerplate_io;
mod boilerplate_web;
mod cast;
#[cfg(feature = "rust-async")]
mod misc_rust_async;
mod pointer;

pub use crate::codec::dco::{transform_result_dco, Rust2DartMessageDco};
pub use crate::codec::sse::{
    Dart2RustMessageSse, Rust2DartMessageSse, SseDeserializer, SseSerializer,
};
pub use crate::codec::Rust2DartMessageTrait;
pub use crate::codec::{cst::CstCodec, dco::DcoCodec, sse::SseCodec, BaseCodec};
#[cfg(feature = "dart-opaque")]
pub use crate::dart_opaque::dart2rust::{cst_decode_dart_opaque, sse_decode_dart_opaque};
pub use crate::generalized_arc::base_arc::BaseArc;
pub use crate::generalized_arc::std_arc::StdArc; // TODO temp
pub use crate::generalized_isolate::Channel;
pub use crate::generalized_isolate::IntoDartExceptPrimitive;
pub use crate::handler::error::Error as HandlerError;
pub use crate::handler::error_listener::ErrorListener;
pub use crate::handler::executor::Executor;
pub use crate::handler::handler::{FfiCallMode, TaskInfo};
pub use crate::handler::handler::{TaskContext, TaskRetFutTrait};
pub use crate::handler::implementation::error_listener::NoOpErrorListener;
pub use crate::handler::implementation::executor::SimpleExecutor;
pub use crate::handler::implementation::handler::SimpleHandler;
pub use crate::lifetimeable::lifetime_changer::{
    ouroboros_change_lifetime, ouroboros_change_lifetime_mut,
};
pub use crate::lifetimeable::{dependency::LifetimeableDependency, Lifetimeable};
#[cfg(feature = "rust-async")]
pub use crate::lockable::{
    base::Lockable, order::LockableOrder, order_computer::lockable_compute_decode_order,
    order_info::LockableOrderInfo,
};
#[allow(unused)]
pub use crate::misc::manual_impl::*;
pub use crate::misc::version::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION;
#[cfg(wasm)]
pub use crate::misc::web_utils;
pub use crate::platform_types::{
    DartAbi, MessagePort, PlatformGeneralizedUint8ListPtr, WireSyncRust2DartDco,
    WireSyncRust2DartSse,
};
pub use crate::rust2dart::action::Rust2DartAction;
pub use crate::rust_async;
pub use crate::rust_async::{BaseAsyncRuntime, SimpleAsyncRuntime};
#[cfg(feature = "rust-async")]
pub use crate::rust_auto_opaque::dart2rust_explicit::rust_auto_opaque_explicit_decode;
#[cfg(feature = "rust-async")]
pub use crate::rust_auto_opaque::dart2rust_implicit::{
    rust_auto_opaque_decode_owned, rust_auto_opaque_encode, rust_auto_opaque_lockable_order,
};
#[cfg(feature = "rust-async")]
pub use crate::rust_auto_opaque::rust2dart_explicit::rust_auto_opaque_explicit_encode;
#[cfg(feature = "rust-async")]
pub use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
pub use crate::rust_opaque::{dart2rust::decode_rust_opaque_nom, RustOpaqueBase};
pub use crate::stream::stream_sink::StreamSinkBase;
pub use crate::thread_pool::{BaseThreadPool, SimpleThreadPool};
#[cfg(wasm)]
pub use crate::web_transfer::transfer_closure::TransferClosure;
#[cfg(feature = "anyhow")]
pub use anyhow;
pub use byteorder;
#[cfg(wasm)]
pub use cast::slice_from_byte_buffer;
#[cfg(feature = "dart-opaque")]
#[cfg(not(wasm))]
pub use dart_sys_fork as dart_sys;
#[cfg(feature = "rust-async")]
pub use futures;
#[cfg(wasm)]
pub use js_sys;
pub use lazy_static::lazy_static;
#[cfg(feature = "rust-async")]
pub use misc_rust_async::*;
pub use pointer::*;
#[cfg(wasm)]
pub use wasm_bindgen;
