use crate::for_generated::BaseArc;
use crate::rust_async::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::rust_opaque::RustOpaqueBase;

impl<T, A: BaseArc<RwLock<T>>> From<T> for RustOpaqueBase<RwLock<T>, A> {
    fn from(value: T) -> Self {
        Self::new(RwLock::new(value))
    }
}
