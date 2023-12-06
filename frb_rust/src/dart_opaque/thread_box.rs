/// Only allows manipulation at the thread which it is created.
/// It is a "black box" that nobody can open it when it is on another thread.
pub struct ThreadBox<T> {
    inner: T,
    /// The ID of the thread on which it was created.
    thread_id: ThreadId,
}

impl<T> ThreadBox<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            thread_id: std::thread::current().id(),
        }
    }

    pub fn try_unwrap(mut self) -> Result<T, Self> {
        if std::thread::current().id() == self.thread_id {
            Ok(self.inner.take().unwrap().unwrap())
        } else {
            Err(self)
        }
    }
}
