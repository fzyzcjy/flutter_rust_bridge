use super::TryLockError;
use std::sync;
pub use sync::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Default, Debug)]
pub struct RwLock<T>(sync::RwLock<T>);

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self(sync::RwLock::new(value))
    }

    pub fn into_inner(self) -> T {
        self.0.into_inner().unwrap()
    }

    pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().unwrap()
    }

    pub fn blocking_write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }

    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError> {
        self.0.try_read().map_err(|_| TryLockError)
    }

    pub fn try_write(&self) -> Result<RwLockWriteGuard<'_, T>, TryLockError> {
        self.0.try_write().map_err(|_| TryLockError)
    }
}
