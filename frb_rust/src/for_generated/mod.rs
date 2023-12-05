//! Utilities to support the auto-generated Rust code.
//! These functions are usually *not* meant to be used by humans directly.

mod cast;
mod pointer;

pub use crate::generalized_isolate::Channel;
pub use crate::generalized_isolate::IntoDartExceptPrimitive;
pub use crate::handler::handler::{FfiCallMode, TaskInfo};
pub use crate::misc::manual_impl::*;
pub use crate::misc::rust_arc::{rust_arc_decrement_strong_count, rust_arc_increment_strong_count};
pub use crate::misc::rust_auto_opaque::rust_auto_opaque_api2wire;
pub use crate::platform_types::DartAbi;
pub use crate::platform_types::{MessagePort, WireSyncReturn};
pub use crate::rust_async::{BaseAsyncRuntime, SimpleAsyncRuntime};
pub use crate::rust_opaque::dart2rust::wire2api_rust_opaque;
pub use crate::thread_pool::{BaseThreadPool, SimpleThreadPool};
#[cfg(wasm)]
pub use crate::web_transfer::transfer_closure::TransferClosure;
pub use cast::*;
pub use futures;
#[cfg(wasm)]
pub use js_sys;
pub use lazy_static::lazy_static;
pub use pointer::*;
#[cfg(wasm)]
pub use wasm_bindgen;
