use crate::for_generated::{BaseArc, RustAutoOpaqueInner, RustOpaqueBase};
use crate::lockable::base::Lockable;
use crate::lockable::order::LockableOrder;
use std::future::Future;
use std::pin::Pin;

impl<T: Send + Sync, A: BaseArc<RustAutoOpaqueInner<T>>> Lockable
    for RustOpaqueBase<RustAutoOpaqueInner<T>, A>
{
    type RwLockReadGuard<'a>
        = crate::rust_async::RwLockReadGuard<'a, T>
    where
        A: 'a;
    type RwLockWriteGuard<'a>
        = crate::rust_async::RwLockWriteGuard<'a, T>
    where
        A: 'a;

    fn lockable_order(&self) -> LockableOrder {
        self.order
    }

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "rust-async-tokio", target_family = "wasm"))]  {
            fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_> {
                self.data.blocking_read()
            }

            fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_> {
                self.data.blocking_write()
            }
        } else if #[cfg(all(feature = "rust-async-async-std", not(target_family = "wasm")))] {
            fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_> {
                self.data.read_blocking()
            }

            fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_> {
                self.data.write_blocking()
            }
        }
    }

    fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_> {
        self.data.blocking_read()
    }

    fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_> {
        self.data.blocking_write()
    }

    fn lockable_decode_async_ref<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Self::RwLockReadGuard<'a>> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move { self.data.read().await })
    }

    fn lockable_decode_async_ref_mut<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Self::RwLockWriteGuard<'a>> + Send + 'a>>
    where
        Self: Sync + 'a,
    {
        Box::pin(async move { self.data.write().await })
    }
}
