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
pub struct ThreadBox<T> {
    // `Option` is used for correct drop.
    inner: Option<T>,
    /// The ID of the thread on which it was created.
    thread_id: ThreadId,
}

impl<T> ThreadBox<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            thread_id: std::thread::current().id(),
        }
    }

    pub fn try_unwrap(mut self) -> Result<T, Self> {
        if std::thread::current().id() == self.thread_id {
            Ok(self.inner.take().unwrap())
        } else {
            Err(self)
        }
    }
}

impl<T> Drop for ThreadBox<T> {
    fn drop(&mut self) {
        if self.inner.is_some() && std::thread::current().id() != self.thread_id {
            panic!("ThreadBox should never be dropped on a thread other than creation thread.")
        }
    }
}

/// # Safety
///
/// See documentation of `ThreadBox` struct
unsafe impl<T> Send for ThreadBox<T> {}

/// # Safety
///
/// See documentation of `ThreadBox` struct
unsafe impl<T> Sync for ThreadBox<T> {}
