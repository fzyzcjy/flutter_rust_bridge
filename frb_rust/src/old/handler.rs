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

/// The types of return values for a particular Rust function.
#[derive(Copy, Clone)]
pub enum FfiCallMode {
    /// The default mode, returns a Dart `Future<T>`.
    Normal,
    /// Used by `SyncReturn<T>` to skip spawning workers.
    Sync,
    /// Returns a Dart `Stream<T>`.
    Stream,
}

/// Supporting information to identify a function's operating mode.
#[derive(Clone)]
pub struct WrapInfo {
    /// A Dart `SendPort`. [None] if the mode is [FfiCallMode::Sync].
    pub port: Option<MessagePort>,
    /// Usually the name of the function.
    pub debug_name: &'static str,
    /// The call mode of this function.
    pub mode: FfiCallMode,
}

/// Errors that occur from normal code execution.
pub enum Error {
    /// Errors that implement [IntoDart].
    CustomError(Box<dyn BoxIntoDart>),
    /// Exceptional errors from panicking.
    Panic(Box<dyn Any + Send>),
}

fn error_to_string(panic_err: &Box<dyn Any + Send>) -> String {
    match panic_err.downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => match panic_err.downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<dyn Any>",
        },
    }
    .to_string()
}

impl Error {
    /// The message of the error.
    pub fn message(&self) -> String {
        match self {
            Error::CustomError(_e) => "Box<dyn BoxIntoDart>".to_string(),
            Error::Panic(panic_err) => error_to_string(panic_err),
        }
    }
}

impl IntoDart for Error {
    fn into_dart(self) -> DartAbi {
        match self {
            Error::CustomError(e) => e.box_into_dart(),
            Error::Panic(panic_err) => error_to_string(&panic_err).into_dart(),
        }
    }
}
