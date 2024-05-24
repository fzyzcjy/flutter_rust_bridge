use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_async::{RwLockReadGuard, RwLockWriteGuard};
use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
use crate::rust_auto_opaque::RustAutoOpaqueBase;
use crate::rust_opaque::RustOpaqueBase;
use tokio::sync::RwLock;

impl<T: 'static, A: BaseArc<RustAutoOpaqueInner<T>>> RustAutoOpaqueBase<T, A> {
    pub fn new(value: T) -> Self {
        Self(RustOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(
            value,
        ))))
    }

    pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
        self.0.data.blocking_read()
    }

    pub fn blocking_write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.data.blocking_write()
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.data.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.data.write().await
    }
}
