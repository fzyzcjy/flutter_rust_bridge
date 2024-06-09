use crate::for_generated::{BaseArc, RustAutoOpaqueInner, RustOpaqueBase};
use crate::lockable::base::Lockable;
use std::future::Future;
use std::pin::Pin;
use crate::lockable::order::LockableOrderInfo;
use crate::lockable::order::LockableOrder;

impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> Lockable for RustOpaqueBase<RustAutoOpaqueInner<T>, A> {
    type RwLockReadGuard = crate::rust_async::RwLockReadGuard<'_, T>;
    type RwLockWriteGuard = crate::rust_async::RwLockWriteGuard<'_, T>;

    fn lockable_order(&self) -> LockableOrder {
        self.order
    }

    fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard {
        self.data.blocking_read()
    }

    fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard {
        self.data.blocking_write()
    }

    fn lockable_decode_async_ref<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Self::RwLockReadGuard> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move { self.data.read().await })
    }

    fn lockable_decode_async_ref_mut<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Self::RwLockWriteGuard> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move { self.data.write().await })
    }
}
