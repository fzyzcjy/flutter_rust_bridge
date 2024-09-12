use crate::lockable::order::LockableOrder;
use std::fmt;
use std::fmt::Formatter;
use tokio::sync::RwLock;

pub struct RustAutoOpaqueInner<T> {
    pub(crate) data: RwLock<T>,
    pub(crate) order: LockableOrder,
}

impl<T> RustAutoOpaqueInner<T> {
    pub(crate) fn new(data: RwLock<T>) -> Self {
        Self {
            data,
            order: LockableOrder::new(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for RustAutoOpaqueInner<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}
