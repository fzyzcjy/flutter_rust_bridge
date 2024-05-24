use crate::rust_auto_opaque::order::RustAutoOpaqueOrder;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

pub struct RustAutoOpaqueInner<T> {
    pub(crate) data: RwLock<T>,
    pub(crate) order: RustAutoOpaqueOrder,
}

impl<T> RustAutoOpaqueInner<T> {
    pub(crate) fn new(data: RwLock<T>) -> Self {
        Self {
            data,
            order: RustAutoOpaqueOrder::new(),
        }
    }
}

// TODO rm
// impl<T> RustAutoOpaqueInner<T> {
//     pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, TryLockError> {
//         self.data.try_read()
//     }
//
//     pub fn try_write(&self) -> Result<RwLockWriteGuard<'_, T>, TryLockError> {
//         self.data.try_write()
//     }
// }
