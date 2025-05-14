pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

pub trait BaseAsyncRuntime {}

#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleAsyncRuntime;

impl BaseAsyncRuntime for SimpleAsyncRuntime {}
