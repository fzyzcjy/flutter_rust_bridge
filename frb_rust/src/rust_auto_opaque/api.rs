use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_async::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
use crate::rust_auto_opaque::RustAutoOpaqueBase;
use crate::rust_opaque::RustOpaqueBase;

impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> RustAutoOpaqueBase<T, A> {
    pub fn new(value: T) -> Self {
        Self(RustOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(
            value,
        ))))
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.data.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.data.write().await
    }

    cfg_if::cfg_if! {
        if #[cfg(any(feature = "rust-async-tokio", target_family = "wasm"))]  {
            pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
                self.0.data.blocking_read()
            }

            pub fn blocking_write(&self) -> RwLockWriteGuard<'_, T> {
                    self.0.data.blocking_write()
            }

            pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, tokio::sync::TryLockError> {
                self.0.data.try_read()
            }

            pub fn try_write(&self) -> Result<RwLockWriteGuard<'_, T>, tokio::sync::TryLockError> {
                self.0.data.try_write()
            }
        } else if #[cfg(all(feature = "rust-async-async-std", not(target_family = "wasm")))] {
            pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
                self.0.data.read_blocking()
            }

            pub fn blocking_write(&self) -> RwLockWriteGuard<'_, T> {
                    self.0.data.write_blocking()
            }

            pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
                self.0.data.try_read()
            }

            pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
                self.0.data.try_write()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::RustAutoOpaqueNom;

    #[test]
    fn test_api_sync() {
        let opaque = RustAutoOpaqueNom::new(42);
        assert_eq!(*opaque.blocking_read(), 42);
        assert_eq!(*opaque.blocking_write(), 42);
        assert_eq!(*opaque.try_read().unwrap(), 42);
        assert_eq!(*opaque.try_write().unwrap(), 42);
    }

    #[cfg(not(target_family = "wasm"))]
    #[tokio::test]
    async fn test_api_async() {
        let opaque = RustAutoOpaqueNom::new(42);
        assert_eq!(*opaque.read().await, 42);
        assert_eq!(*opaque.write().await, 42);
    }

    #[test]
    fn test_clone() {
        let a = RustAutoOpaqueNom::new(42);
        let b = a.clone();
        *a.blocking_write() = 200;
        assert_eq!(*b.blocking_read(), 200);
    }
}
