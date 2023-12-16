use crate::dart_opaque::boxes::guarded_box::{GuardedBox, GuardedBoxContext};
use delegate_attr::delegate;
use log::warn;
use std::fmt::{Debug, Formatter};
use std::thread::ThreadId;

/// Only allows manipulation of the inner value at the thread which it is created.
/// See the documentation of [GuardedBox] for more details.
#[derive(Debug)]
pub struct ThreadBox<T: Debug>(GuardedBox<T, GuardedBoxContextThread>);

impl<T: Debug> ThreadBox<T> {
    pub fn new(inner: T) -> Self {
        Self(GuardedBox::new(inner))
    }
}

#[delegate(self.0)]
impl<T: Debug> ThreadBox<T> {
    pub fn check_context(&self) -> bool {}

    pub fn into_inner(self) -> T {}
}

#[delegate(self.0)]
impl<T: Debug> AsRef<T> for ThreadBox<T> {
    fn as_ref(&self) -> &T {}
}

/// # Safety
///
/// The inner value can never be (1) used or (2) dropped
/// on any thread except for the creation thread.
///
/// Therefore, even though it is `Send`/`Sync` among threads,
/// it is just a blackbox on all other threads, so we are safe.
unsafe impl<T: Debug> Send for ThreadBox<T> {}

/// # Safety
///
/// See documentation for `Send`
unsafe impl<T: Debug> Sync for ThreadBox<T> {}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GuardedBoxContextThread(ThreadId);

impl GuardedBoxContext for GuardedBoxContextThread {
    fn new() -> Self {
        Self(std::thread::current().id())
    }
}
