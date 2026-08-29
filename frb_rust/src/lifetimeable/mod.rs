pub(crate) mod dependency;
pub(crate) mod lifetime_changer;

use crate::lifetimeable::dependency::LifetimeableDependency;
use std::ops;

pub struct Lifetimeable<T> {
    // NOTE: The borrowed value must be *before* the dependency values to have correct *drop order*
    value: T,
    #[allow(dead_code)]
    dependencies: Vec<LifetimeableDependency>,
}

impl<T> Lifetimeable<T> {
    pub fn new(value: T, dependencies: Vec<LifetimeableDependency>) -> Self {
        Self {
            value,
            dependencies,
        }
    }
}

impl<T> ops::Deref for Lifetimeable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> ops::DerefMut for Lifetimeable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
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
    /// Dereferences to the wrapped value mutably and immutably.
    fn dereferences_to_value() {
        let mut value = Lifetimeable::new(42, vec![]);
        assert_eq!(*value, 42);

        *value = 100;

        assert_eq!(*value, 100);
    }

    #[test]
    /// Drops the value before its dependencies.
    fn drops_value_before_dependencies() {
        let dropped = Arc::new(Mutex::new(Vec::new()));
        let value = Lifetimeable::new(
            DropRecorder("value", dropped.clone()),
            vec![LifetimeableDependency::new_guard_lockable(
                Box::new(DropRecorder("guard", dropped.clone())),
                Box::new(DropRecorder("lockable", dropped.clone())),
            )],
        );

        drop(value);

        assert_eq!(*dropped.lock().unwrap(), ["value", "guard", "lockable"]);
    }
}
