use crate::dart_opaque::boxes::guarded_box::{GuardedBox, GuardedBoxGuard};
use delegate_attr::delegate;
use log::warn;
use std::fmt::{Debug, Formatter};
use std::thread::ThreadId;

/// Only allows manipulation at the Dart Isolate which it is created.
///
/// Only *after* knowing [T] is safe to be worked with within a single isolate,
/// we can unsafe impl [Send] or [Sync].
#[derive(Debug)]
pub struct DartIsolateBox<T: Debug>(GuardedBox<T, DartIsolateGuard>);

impl<T: Debug> DartIsolateBox<T> {
    pub fn new(inner: T) -> Self {
        Self(GuardedBox::new(inner))
    }
}

#[delegate(self.0)]
impl<T: Debug> DartIsolateBox<T> {
    pub fn check_guard(&self) -> bool {}

    pub fn into_inner(self) -> T {}
}

#[delegate(self.0)]
impl<T: Debug> AsRef<T> for DartIsolateBox<T> {
    fn as_ref(&self) -> &T {}
}

#[derive(Debug)]
pub(crate) struct DartIsolateGuard(dart_sys::Dart_Isolate);

impl GuardedBoxGuard for DartIsolateGuard {
    fn new() -> Self {
        unsafe { Self(dart_current_isolate()) }
    }

    fn check(&self) -> bool {
        dart_current_isolate() == self.0
    }

    fn check_failure_message(&self) -> String {
        format!(
            "DartIsolateGuard can only be used within the same isolaet. current_isolate={:?} creation_isolate={:?}",
            dart_current_isolate(), self.0,
        )
    }
}

fn dart_current_isolate() -> dart_sys::Dart_Isolate {
    unsafe { dart_sys::Dart_CurrentIsolate_DL.unwrap()() }
}
