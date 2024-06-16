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
