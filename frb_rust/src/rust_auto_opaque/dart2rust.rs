use crate::for_generated::BaseArc;
use crate::rust_async::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::rust_auto_opaque::RustAutoOpaqueBase;

impl<T, A: BaseArc<RwLock<T>>> RustAutoOpaqueBase<T, A> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    pub fn rust_auto_opaque_decode_owned(self) -> T {
        // frb-coverage:ignore-end
        self.into_inner()
            .expect("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")
            .into_inner()
    }

    pub fn rust_auto_opaque_decode_sync_ref(&self) -> RwLockReadGuard<'_, T> {
        self.blocking_read()
    }

    pub fn rust_auto_opaque_decode_sync_ref_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.blocking_write()
    }

    pub async fn rust_auto_opaque_decode_async_ref(&self) -> RwLockReadGuard<'_, T> {
        self.read().await
    }

    pub async fn rust_auto_opaque_decode_async_ref_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.write().await
    }

    // TODO surely not i32
    pub fn rust_auto_opaque_lock_order(&self) -> i32 {
        todo!()
    }
}

pub fn rust_auto_opaque_encode<T, A: BaseArc<RwLock<T>>>(value: T) -> RustAutoOpaqueBase<T, A> {
    RustAutoOpaqueBase::new(RwLock::new(value))
}

// TODO surely not i32
pub fn rust_auto_opaque_decode_compute_order(orders: &[i32]) -> Vec<usize> {
    todo!()
}
