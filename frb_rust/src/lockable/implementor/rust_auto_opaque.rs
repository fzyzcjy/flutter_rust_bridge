use crate::for_generated::{BaseArc, RustAutoOpaqueInner, RustOpaqueBase};
use crate::lockable::base::Lockable;
use crate::lockable::order::LockableOrder;
#[cfg(target_family = "wasm")]
use crate::rust_auto_opaque::web_is_dedicated_worker_context;
#[cfg(target_family = "wasm")]
use crate::rust_auto_opaque::web_throw_lock_error;
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

    fn lockable_decode_sync_ref(&self) -> Self::RwLockReadGuard<'_> {
        // Web integration tests cover this, but Rust llvm coverage does not observe wasm.
        // frb-coverage:ignore-start
        #[cfg(target_family = "wasm")]
        {
            if web_is_dedicated_worker_context() {
                self.data.blocking_read()
            } else {
                self.data
                    .try_read()
                    .unwrap_or_else(|error| web_throw_lock_error("read", error))
            }
        }
        // frb-coverage:ignore-end
        #[cfg(not(target_family = "wasm"))]
        {
            self.data.blocking_read()
        }
    }

    fn lockable_decode_sync_ref_mut(&self) -> Self::RwLockWriteGuard<'_> {
        // Web integration tests cover this, but Rust llvm coverage does not observe wasm.
        // frb-coverage:ignore-start
        #[cfg(target_family = "wasm")]
        {
            if web_is_dedicated_worker_context() {
                self.data.blocking_write()
            } else {
                self.data
                    .try_write()
                    .unwrap_or_else(|error| web_throw_lock_error("write", error))
            }
        }
        // frb-coverage:ignore-end
        #[cfg(not(target_family = "wasm"))]
        {
            self.data.blocking_write()
        }
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

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use crate::for_generated::Lockable;
    use crate::RustAutoOpaqueNom;

    #[test]
    /// Decodes opaque values through synchronous lockable methods.
    fn decodes_synchronously() {
        let opaque = RustAutoOpaqueNom::new(42);

        assert_eq!(*opaque.0.lockable_decode_sync_ref(), 42);
        *opaque.0.lockable_decode_sync_ref_mut() = 100;

        assert_eq!(*opaque.0.lockable_decode_sync_ref(), 100);
    }

    #[tokio::test]
    /// Decodes opaque values through asynchronous lockable methods.
    async fn decodes_asynchronously() {
        let opaque = RustAutoOpaqueNom::new(42);

        assert_eq!(*opaque.0.lockable_decode_async_ref().await, 42);
        *opaque.0.lockable_decode_async_ref_mut().await = 100;

        assert_eq!(*opaque.0.lockable_decode_async_ref().await, 100);
    }
}
