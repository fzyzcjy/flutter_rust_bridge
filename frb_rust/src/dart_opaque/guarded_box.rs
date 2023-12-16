use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct GuardedBox<T: Debug, G: GuardedBoxGuard> {
    // `Option` is used for correct drop.
    inner: Option<T>,
    guard: G,
}

pub(crate) trait GuardedBoxGuard: Debug {
    fn new() -> Self;

    fn check(&self) -> bool;

    fn check_failure_message(&self) -> String;
}

impl<T: Debug, G: GuardedBoxGuard> GuardedBox<T, G> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            guard: G::new(),
        }
    }

    pub fn check_guard(&self) -> bool {
        self.guard.check()
    }

    fn ensure_guard(&self) {
        if !self.check_guard() {
            self.panic_because_guard_failed()
        }
    }

    fn panic_because_guard_failed(&self) -> ! {
        panic!("{}", self.guard.check_failure_message())
    }

    pub fn into_inner(mut self) -> T {
        self.ensure_guard();
        self.inner.take().unwrap()
    }
}

impl<T: Debug> AsRef<T> for GuardedBox<T> {
    fn as_ref(&self) -> &T {
        self.ensure_guard();
        self.inner.as_ref().unwrap()
    }
}

impl<T: Debug> Drop for GuardedBox<T> {
    fn drop(&mut self) {
        if self.inner.is_some() && !self.check_guard() {
            if std::thread::panicking() {
                let msg = "GuardedBox.drop cannot drop data because the guard failed. \
However, system is already panicking so we cannot panic twice. \
Therefore, we have to make a memory leak for the data.";
                warn!("{}", msg);
                println!("{}", msg);

                std::mem::forget(self.inner.take());
            } else {
                self.panic_because_guard_failed()
            }
        }
    }
}
