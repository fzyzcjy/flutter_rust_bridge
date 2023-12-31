use crate::dart_opaque::boxes::guarded_box::{GuardedBox, GuardedBoxContext};
use delegate_attr::delegate;
use std::fmt::Debug;
use std::thread::ThreadId;

/// Only allows manipulation of the inner value at the thread which it is created.
/// See the documentation of [GuardedBox] for more details.
#[derive(Debug)]
pub struct ThreadBox<T: Debug>(GuardedBox<T, GuardedBoxContextThread>);

impl<T: Debug> ThreadBox<T> {
    #[allow(unused)]
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
    fn current() -> Self {
        Self(std::thread::current().id())
    }
}

#[cfg(not(wasm))]
#[cfg(test)]
mod tests {
    use crate::dart_opaque::boxes::thread_box::ThreadBox;
    use cool_asserts::assert_panics;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_thread_box_simple() {
        let b = ThreadBox::new(42);
        assert!(b.check_context());
        assert_eq!(*b.as_ref(), 42);
        assert_eq!(b.into_inner(), 42);
    }

    #[test]
    fn test_thread_box_should_panic_when_access_on_another_thread() {
        let b = Arc::new(ThreadBox::new(42));
        let b2 = b.clone();
        thread::spawn(move || {
            assert_panics!({
                let _inner: &i32 = (*b2).as_ref();
            });
        })
        .join()
        .unwrap();
        drop(b);
    }

    #[test]
    fn test_thread_box_should_panic_and_leak_when_access_and_drop_on_another_thread() {
        let b = ThreadBox::new(42);

        #[allow(clippy::redundant_closure_call)]
        thread::spawn(move || {
            assert_panics!((move || {
                let _inner: &i32 = b.as_ref();
            })());
        })
        .join()
        .unwrap();
    }

    #[test]
    fn test_thread_box_should_panic_when_drop_on_another_thread() {
        let b = ThreadBox::new(42);
        thread::spawn(move || {
            assert_panics!(drop(b));
        })
        .join()
        .unwrap();
    }
}
