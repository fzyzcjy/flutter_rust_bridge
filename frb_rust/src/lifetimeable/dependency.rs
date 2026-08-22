use std::any::Any;

pub struct LifetimeableDependency {
    // NOTE again the ordering - firstly drop guard, secondly drop lockable
    #[allow(dead_code)]
    guard: Box<dyn Any + Send + Sync>,
    #[allow(dead_code)]
    lockable: Box<dyn Any + Send + Sync>,
}

impl LifetimeableDependency {
    pub fn new_guard_lockable(
        guard: Box<dyn Any + Send + Sync>,
        lockable: Box<dyn Any + Send + Sync>,
    ) -> Self {
        Self { guard, lockable }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    struct DropRecorder(&'static str, Arc<Mutex<Vec<&'static str>>>);

    impl Drop for DropRecorder {
        fn drop(&mut self) {
            self.1.lock().unwrap().push(self.0);
        }
    }

    #[test]
    /// Drops the guard before the lockable dependency.
    fn drops_guard_before_lockable() {
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let dependency = LifetimeableDependency::new_guard_lockable(
            Box::new(DropRecorder("guard", dropped.clone())),
            Box::new(DropRecorder("lockable", dropped.clone())),
        );

        drop(dependency);

        assert_eq!(*dropped.lock().unwrap(), ["guard", "lockable"]);
    }
}
