use crate::for_generated::{BaseArc, RustAutoOpaqueInner, RustOpaqueBase};
use crate::lockable::base::Lockable;
use crate::lockable::order::LockableOrder;
use std::future::Future;
use std::pin::Pin;

impl<T: Send + Sync, A: BaseArc<RustAutoOpaqueInner<T>>> Lockable
    for RustOpaqueBase<RustAutoOpaqueInner<T>, A>
{
    type RwLockReadGuard<'a> = crate::rust_async::RwLockReadGuard<'a, T> where A: 'a;
    type RwLockWriteGuard<'a> = crate::rust_async::RwLockWriteGuard<'a, T>where A: 'a;

    fn lockable_order(&self) -> LockableOrder {
        self.order
    }

    fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_> {
        self.data.blocking_read()
    }

    fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_> {
        self.data.blocking_write()
    }

    fn lockable_decode_async_ref<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Self::RwLockReadGuard<'_>> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move { self.data.read().await })
    }

    fn lockable_decode_async_ref_mut<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Self::RwLockWriteGuard<'_>> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move { self.data.write().await })
    }
}
