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
