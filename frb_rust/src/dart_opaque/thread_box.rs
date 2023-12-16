use crate::dart_opaque::guarded_box::{GuardedBox, GuardedBoxGuard};
use log::warn;
use std::fmt::{Debug, Formatter};
use std::thread::ThreadId;

// TODO the comments
/// Only allows manipulation at the thread which it is created.
/// It is a "black box" that nobody can open it when it is on another thread.
///
/// # Safety
///
/// The inner value can never be (1) used or (2) dropped
/// on any thread except for the creation thread.
///
/// Therefore, even though it is `Send`/`Sync` among threads,
/// it is just a blackbox on all other threads, so we are safe.
pub struct ThreadBox<T>(GuardedBox<T, ThreadGuard>);

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
        std::thread::current().id()
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
