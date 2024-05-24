use crate::generalized_arc::base_arc::BaseArc;
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
