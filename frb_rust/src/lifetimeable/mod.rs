pub(crate) mod lifetime_changer;

use crate::for_generated::{BaseArc, RustOpaqueBase};
use std::any::Any;
use std::ops;

pub struct Lifetimeable<T> {
    // NOTE: The borrowed value must be *before* the dependency values to have correct *drop order*
    value: T,
    dependencies: Vec<Box<dyn Any + Send + Sync>>,
}

impl<T> Lifetimeable<T> {
    pub fn new(value: T, dependencies: Vec<Box<dyn Any + Send + Sync>>) -> Self {
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
