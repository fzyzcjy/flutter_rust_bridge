use std::fmt::Debug;
use std::thread::ThreadId;

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
#[derive(Debug)]
pub struct ThreadBox<T: Debug> {
    // `Option` is used for correct drop.
    inner: Option<T>,
    /// The ID of the thread on which it was created.
    thread_id: ThreadId,
}

impl<T: Debug> ThreadBox<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            thread_id: std::thread::current().id(),
        }
    }

    pub fn is_on_creation_thread(&self) -> bool {
        std::thread::current().id() == self.thread_id
    }

    fn ensure_on_creation_thread(&self) {
        if !self.is_on_creation_thread() {
            panic!(
                "ThreadBox can only be used on the creation thread. current_thread={:?} creation_thread={:?}",
                std::thread::current().id(), self.thread_id,
            )
        }
    }

    pub fn into_inner(mut self) -> T {
        self.ensure_on_creation_thread();
        self.inner.take().unwrap()
    }
}

impl<T: Debug> AsRef<T> for ThreadBox<T> {
    fn as_ref(&self) -> &T {
        self.ensure_on_creation_thread();
        self.inner.as_ref().unwrap()
    }
}

impl<T: Debug> Drop for ThreadBox<T> {
    fn drop(&mut self) {
        if self.inner.is_some() {
            self.ensure_on_creation_thread();
        }
    }
}

/// # Safety
///
/// See documentation of `ThreadBox` struct
unsafe impl<T: Debug> Send for ThreadBox<T> {}

/// # Safety
///
/// See documentation of `ThreadBox` struct
unsafe impl<T: Debug> Sync for ThreadBox<T> {}
