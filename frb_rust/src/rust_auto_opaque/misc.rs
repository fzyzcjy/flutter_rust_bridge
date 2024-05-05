use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::{RustAutoOpaqueBase, RustAutoOpaqueInner};
use crate::rust_auto_opaque::order::RustAutoOpaqueOrder;

impl<T> RustAutoOpaqueInner<T> {
    pub fn new(data: RwLock<T>) -> Self {
        Self {
            data,
            order: RustAutoOpaqueOrder::new(),
        }
    }
}

impl<T> RustAutoOpaqueInner<T> {
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError> {
        self.data.try_read()
    }

    pub fn try_write(&self) -> Result<RwLockWriteGuard<'_, T>, TryLockError> {
        self.data.try_write()
    }
}
