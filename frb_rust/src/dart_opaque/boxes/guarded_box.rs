use crate::misc::logs::log_warn_or_println;
use std::fmt::Debug;

/// Only allows manipulation of [inner] value when the [context] is the same
/// as the context when it is created.
///
/// It is a "black box" that nobody can open it otherwise.
#[derive(Debug)]
pub(crate) struct GuardedBox<T: Debug, C: GuardedBoxContext> {
    // `Option` is used for correct drop.
    inner: Option<T>,
    context: C,
}

pub(crate) trait GuardedBoxContext: Debug + Eq {
    fn current() -> Self;
}

impl<T: Debug, C: GuardedBoxContext> GuardedBox<T, C> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            context: C::current(),
        }
    }

    pub fn check_context(&self) -> bool {
        self.context == C::current()
    }

    fn ensure_context(&self) {
        if !self.check_context() {
            self.panic_because_context_failed()
        }
    }

    fn panic_because_context_failed(&self) -> ! {
        panic!(
            "GuardedBox can only be used when the context is the same as the context when it is created. current={:?} creation={:?}",
            C::current(), self.context,
        )
    }

    pub fn into_inner(mut self) -> T {
        self.ensure_context();
        self.inner.take().unwrap()
    }
}

impl<T: Debug, C: GuardedBoxContext> AsRef<T> for GuardedBox<T, C> {
    fn as_ref(&self) -> &T {
        self.ensure_context();
        self.inner.as_ref().unwrap()
    }
}

impl<T: Debug, C: GuardedBoxContext> Drop for GuardedBox<T, C> {
    fn drop(&mut self) {
        if self.inner.is_some() && !self.check_context() {
            if std::thread::panicking() {
                log_warn_or_println(
                    "GuardedBox.drop cannot drop data because the context is different. \
However, system is already panicking so we cannot panic twice. \
Therefore, we have to make a memory leak for the data.",
                );

                std::mem::forget(self.inner.take());
            } else {
                self.panic_because_context_failed()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GuardedBox, GuardedBoxContext};
    use cool_asserts::assert_panics;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Mutex;

    static CURRENT_CONTEXT: AtomicI32 = AtomicI32::new(0);
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[derive(Debug, PartialEq, Eq)]
    struct TestContext(i32);

    impl GuardedBoxContext for TestContext {
        fn current() -> Self {
            Self(CURRENT_CONTEXT.load(Ordering::SeqCst))
        }
    }

    fn set_context(value: i32) {
        CURRENT_CONTEXT.store(value, Ordering::SeqCst);
    }

    /// Allows access and ownership transfer in the creation context.
    #[test]
    fn test_access_and_into_inner_succeed_in_creation_context() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_context(1);
        let value = GuardedBox::<_, TestContext>::new(String::from("value"));

        assert!(value.check_context());
        assert_eq!(value.as_ref(), "value");
        assert_eq!(value.into_inner(), "value");
    }

    /// Rejects shared access after the context changes.
    #[test]
    fn test_as_ref_panics_after_context_changes() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_context(2);
        let value = GuardedBox::<_, TestContext>::new(4);
        set_context(3);

        assert_panics!(value.as_ref());
        set_context(2);
        drop(value);
    }

    /// Rejects ownership transfer after the context changes.
    #[test]
    fn test_into_inner_panics_after_context_changes() {
        let _guard = TEST_LOCK.lock().unwrap();
        set_context(4);
        let value = GuardedBox::<_, TestContext>::new(5);
        set_context(6);

        assert_panics!(value.into_inner());
        set_context(4);
    }
}
