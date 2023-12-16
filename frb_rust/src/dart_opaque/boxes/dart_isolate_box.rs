use crate::dart_opaque::boxes::guarded_box::{GuardedBox, GuardedBoxContext};
use delegate_attr::delegate;
use log::warn;
use std::fmt::{Debug, Formatter};
use std::thread::ThreadId;

/// Only allows manipulation at the Dart Isolate which it is created.
///
/// Only *after* knowing [T] is safe to be worked with within a single isolate,
/// we can unsafe impl [Send] or [Sync].
#[derive(Debug)]
pub struct DartIsolateBox<T: Debug>(GuardedBox<T, GuardedBoxContextDartIsolate>);

impl<T: Debug> DartIsolateBox<T> {
    pub fn new(inner: T) -> Self {
        Self(GuardedBox::new(inner))
    }
}

#[delegate(self.0)]
impl<T: Debug> DartIsolateBox<T> {
    pub fn check_context(&self) -> bool {}

    pub fn into_inner(self) -> T {}
}

#[delegate(self.0)]
impl<T: Debug> AsRef<T> for DartIsolateBox<T> {
    fn as_ref(&self) -> &T {}
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GuardedBoxContextDartIsolate(dart_sys::Dart_Isolate);

impl GuardedBoxContext for GuardedBoxContextDartIsolate {
    fn new() -> Self {
        Self(unsafe { dart_sys::Dart_CurrentIsolate_DL.unwrap()() })
    }
}
