use crate::lockable::order::LockableOrder;
use std::pin::Pin;

// Only for generated code, not for normal users
pub trait Lockable {
    type RwLockReadGuard<'a>
    where
        Self: 'a;
    type RwLockWriteGuard<'a>
    where
        Self: 'a;

    fn lockable_order(&self) -> LockableOrder;

    fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_>;

    fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_>;

    // Manually mimic async_trait's output to avoid introducing another runtime dependency
    fn lockable_decode_async_ref<'a>(
        &'a self,
    ) -> Pin<Box<dyn core::future::Future<Output = Self::RwLockReadGuard<'_>> + Send + 'a>>
    where
        Self: Sync + 'a;

    fn lockable_decode_async_ref_mut<'a>(
        &'a self,
    ) -> Pin<Box<dyn core::future::Future<Output = Self::RwLockWriteGuard<'_>> + Send + 'a>>
    where
        Self: Sync + 'a;
}
