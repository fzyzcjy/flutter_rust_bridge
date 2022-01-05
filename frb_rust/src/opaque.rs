use allo_isolate::ffi::DartCObject;
use allo_isolate::IntoDart;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockResult};

/// A wrapper to immutably share T to the Dart side.
/// To mutate this wrapper's contents, use a smart pointer
/// that enables interior mutability, e.g. [`RwLock`](std::sync::RwLock),
/// [`Mutex`](std::sync::Mutex) or one of the Atomic types.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Opaque<T: ?Sized> {
    pub(crate) ptr: Option<Arc<T>>,
}

impl<T: Default> Default for Opaque<T> {
    fn default() -> Self {
        Self {
            ptr: Some(Arc::new(T::default())),
        }
    }
}

impl<T> Opaque<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        let ptr = Some(Arc::new(value));
        Self { ptr }
    }
    /// Obtains a reference to the inner value.
    /// This will be None if the pointer has been disposed by Dart.
    pub fn as_ref(&self) -> Option<&T> {
        self.ptr.as_deref()
    }
    pub fn dropper() -> *const unsafe extern "C" fn(*mut T) {
        unsafe extern "C" fn drop<T>(ptr: *mut T) {
            Arc::from_raw(ptr);
        }
        drop::<T> as *const _
    }
}

impl<T> Opaque<RwLock<T>> {
    /// See [`RwLock::read`](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.read)
    pub fn read(&self) -> Option<LockResult<RwLockReadGuard<T>>> {
        self.as_ref().map(RwLock::read)
    }
    /// See [`RwLock::write`](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.write)
    pub fn write(&self) -> Option<LockResult<RwLockWriteGuard<T>>> {
        self.as_ref().map(RwLock::write)
    }
    /// See [`RwLock::try_read`](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.try_read)
    pub fn try_read(&self) -> Option<TryLockResult<RwLockReadGuard<T>>> {
        self.as_ref().map(RwLock::try_read)
    }
    /// See [`RwLock::try_write`](https://doc.rust-lang.org/std/sync/struct.RwLock.html#method.try_write)
    pub fn try_write(&self) -> Option<TryLockResult<RwLockWriteGuard<T>>> {
        self.as_ref().map(RwLock::try_write)
    }
}

impl<T> IntoDart for Opaque<T> {
    #[inline]
    fn into_dart(self) -> DartCObject {
        let drop = Opaque::<T>::dropper();
        // ptr is leaked here
        let ptr = self.ptr.map(Arc::into_raw).unwrap_or_else(core::ptr::null);
        vec![ptr.into_dart(), drop.into_dart()].into_dart()
    }
}
