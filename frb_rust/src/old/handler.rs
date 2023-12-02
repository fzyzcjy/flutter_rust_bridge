//! Wrappers and executors for Rust functions.

use crate::ffi::{IntoDart, MessagePort};
use crate::rust2dart::{BoxIntoDart, IntoIntoDart, Rust2Dart, Rust2DartAction, TaskCallback};
use crate::rust_async;
use crate::support::WireSyncReturn;
use crate::{spawn, DartAbi};
use futures::FutureExt;
use std::any::Any;
use std::future::Future;
use std::panic;
use std::panic::{RefUnwindSafe, UnwindSafe};

// TODO move
#[cfg(not(wasm))]
pub trait TaskRetFutTrait: Send {}
#[cfg(not(wasm))]
impl<T: Send> TaskRetFutTrait for T {}
#[cfg(wasm)]
pub trait TaskRetFutTrait {}
#[cfg(wasm)]
impl<T> TaskRetFutTrait for T {}
