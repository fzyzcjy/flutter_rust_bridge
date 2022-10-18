pub use super::DartAbi;
pub use super::MessagePort;
use crate::DartSafe;
pub use allo_isolate::*;
use std::sync::{self, Arc};

#[cfg(feature = "chrono")]
#[inline]
pub fn wire2api_timestamp(ts: i64) -> Timestamp {
    let s = (ts / 1_000_000) as i64;
    let ns = (ts.rem_euclid(1_000_000) * 1_000) as u32;
    Timestamp { s, ns }
}
/// a timestamp with microseconds precision
#[cfg(feature = "chrono")]
pub struct Timestamp {
    /// seconds
    pub s: i64,
    /// nanoseconds
    pub ns: u32,
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn wire2api() {
        // input in microseconds
        let input: i64 = 3_496_567_123;
        let super::Timestamp { s, ns } = super::wire2api_timestamp(input);
        assert_eq!(s, 3_496);
        assert_eq!(ns, 567_123_000);
    }
}

/// A wrapper to transfer ownership of T to Dart.
///
/// This type is equivalent to an [`Option<Arc<T>>`]. The inner pointer may
/// be None if a nullptr is received from Dart, signifying that this pointer
/// has been disposed.
///
/// Extensions for [`sync::RwLock`] and [`sync::Mutex`] are provided.
///
/// ## Naming the inner type
/// When an `Opaque<T>` is transformed into a Dart type, T's string representation
/// undergoes some transformations to become a valid Dart type:
/// - Rust keywords (dyn, 'static, DartSafe, etc.) are automatically removed.
/// - ASCII alphanumerics are kept, all other characters are ignored.
///
/// ## Trait objects
/// Trait objects may be put behind opaque pointers, but they must implement [`DartSafe`] to
/// be safely sent to Dart. For example, this declaration can be used across the
/// FFI border:
/// ```rust
/// use flutter_rust_bridge::*;
/// use std::fmt::Debug;
/// use std::panic::{UnwindSafe, RefUnwindSafe};
/// // Rust does not allow multiple non-auto traits in trait objects, so
/// // this is one workaround.
/// pub trait DartDebug: DartSafe + Debug {}
/// impl<T: DartSafe + Debug> DartDebug for T {}
/// pub struct DebugWrapper(pub Opaque<Box<dyn DartDebug>>);
/// // creating a DebugWrapper using the opaque_dyn macro
/// let wrap = DebugWrapper(opaque_dyn!("foobar"));
/// // it's possible to name it directly
/// pub struct DebugWrapper2(pub Opaque<Box<dyn Debug + Send + Sync + UnwindSafe + RefUnwindSafe>>);
/// ```
#[repr(transparent)]
#[derive(Debug)]
pub struct Opaque<T: ?Sized + DartSafe> {
    pub(crate) ptr: Option<Arc<T>>,
}

impl<T: ?Sized + DartSafe> From<Arc<T>> for Opaque<T> {
    fn from(ptr: Arc<T>) -> Self {
        Self { ptr: Some(ptr) }
    }
}

impl<T: DartSafe> Opaque<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: Some(Arc::new(value)),
        }
    }
}

impl<T: DartSafe> Opaque<sync::RwLock<T>> {
    #[inline]
    pub fn read(&self) -> Option<sync::LockResult<sync::RwLockReadGuard<T>>> {
        self.as_deref().map(sync::RwLock::read)
    }
    #[inline]
    pub fn write(&self) -> Option<sync::LockResult<sync::RwLockWriteGuard<T>>> {
        self.as_deref().map(sync::RwLock::write)
    }
    #[inline]
    pub fn try_read(&self) -> Option<sync::TryLockResult<sync::RwLockReadGuard<T>>> {
        self.as_deref().map(sync::RwLock::try_read)
    }
    #[inline]
    pub fn try_write(&self) -> Option<sync::TryLockResult<sync::RwLockWriteGuard<T>>> {
        self.as_deref().map(sync::RwLock::try_write)
    }
}

impl<T: DartSafe> Opaque<sync::Mutex<T>> {
    #[inline]
    pub fn lock(&self) -> Option<sync::LockResult<sync::MutexGuard<T>>> {
        self.as_deref().map(sync::Mutex::lock)
    }
    #[inline]
    pub fn try_lock(&self) -> Option<sync::TryLockResult<sync::MutexGuard<T>>> {
        self.as_deref().map(sync::Mutex::try_lock)
    }
}

impl<T: DartSafe> Opaque<T> {
    /// Acquire a reference to the inner value, if the pointer has not already
    /// been disposed by Dart.
    pub fn as_deref(&self) -> Option<&T> {
        self.ptr.as_deref()
    }
}

extern "C" fn drop_arc<T>(ptr: *const T) {
    // Dart has ownership of this copy of Arc,
    // and can only lend out clones, so this is safe to call
    // exactly once.
    unsafe {
        Arc::decrement_strong_count(ptr);
    }
}

extern "C" fn lend_arc<T>(ptr: *const T) -> *const T {
    // Equivalent to a clone, but direcly in terms of raw pointers.
    unsafe {
        Arc::increment_strong_count(ptr);
        ptr
    }
}

type CArcDropper<T> = *const extern "C" fn(*const T);
type CArcLender<T> = *const extern "C" fn(*const T) -> *const T;

impl<T: DartSafe> Into<ffi::DartCObject> for Opaque<T> {
    fn into(self) -> ffi::DartCObject {
        // ffi.Pointer? type
        let ptr = match self.ptr {
            Some(arc) => Arc::into_raw(arc).into_dart(),
            _ => ().into_dart(),
        };
        let drop = drop_arc::<T> as CArcDropper<T>;
        let lend = lend_arc::<T> as CArcLender<T>;
        vec![ptr, drop.into_dart(), lend.into_dart()].into_dart()
    }
}

/// Macro helper to instantiate an `Opaque<dyn Trait>`, as Rust does not
/// support custom DSTs on stable.
///
/// Example:
/// ```rust
/// use std::fmt::Debug;
/// use flutter_rust_bridge::*;
/// pub trait MyDebug: DartSafe + Debug {}
/// impl<T: DartSafe + Debug> MyDebug for T {}
/// let opaque: Opaque<Box<dyn MyDebug>> = opaque_dyn!("foobar");
/// ```
#[macro_export]
macro_rules! opaque_dyn {
    ($ex:expr) => {
        Opaque::new(std::boxed::Box::new($ex))
    };
}
