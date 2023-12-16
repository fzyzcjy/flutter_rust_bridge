use crate::dart_opaque::boxes::guarded_box::{GuardedBox, GuardedBoxGuard};
use delegate_attr::delegate;
use log::warn;
use std::fmt::{Debug, Formatter};
use std::thread::ThreadId;

/// Only allows manipulation at the thread which it is created.
/// See the documentation of [GuardedBox] for more details.
///
/// # Safety
///
/// The inner value can never be (1) used or (2) dropped
/// on any thread except for the creation thread.
///
/// Therefore, even though it is `Send`/`Sync` among threads,
/// it is just a blackbox on all other threads, so we are safe.
#[derive(Debug)]
pub struct ThreadBox<T: Debug>(GuardedBox<T, ThreadGuard>);

impl<T: Debug> ThreadBox<T> {
    pub fn new(inner: T) -> Self {
        Self(GuardedBox::new(inner))
    }
}

#[delegate(self.0)]
impl<T: Debug> ThreadBox<T> {
    pub fn check_guard(&self) -> bool {}

    pub fn into_inner(self) -> T {}
}

#[delegate(self.0)]
impl<T: Debug> AsRef<T> for ThreadBox<T> {
    fn as_ref(&self) -> &T {}
}

/// # Safety
///
/// See documentation of `ThreadBox` struct
unsafe impl<T: Debug> Send for ThreadBox<T> {}

/// # Safety
///
/// See documentation of `ThreadBox` struct
unsafe impl<T: Debug> Sync for ThreadBox<T> {}

#[derive(Debug)]
pub(crate) struct ThreadGuard(ThreadId);

impl GuardedBoxGuard for ThreadGuard {
    fn new() -> Self {
        Self(std::thread::current().id())
    }

    fn check(&self) -> bool {
        std::thread::current().id() == self.0
    }

    fn check_failure_message(&self) -> String {
        format!(
            "ThreadGuard can only be used on the creation thread. current_thread={:?} creation_thread={:?}",
            std::thread::current().id(), self.0,
        )
    }
}
