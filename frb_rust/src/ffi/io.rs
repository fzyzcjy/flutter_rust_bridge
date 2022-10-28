use std::{mem, ops, sync::Arc};

use super::drop_arc;
use super::share_arc;
pub use super::DartAbi;
pub use super::MessagePort;
pub use allo_isolate::*;

use crate::DartSafe;

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

/// A wrapper to transfer ownership of T to Dart.
///
/// This type is equivalent to an [`Option<Arc<T>>`]. The inner pointer may
/// be None if a nullptr is received from Dart, signifying that this pointer
/// has been disposed.
///
/// Extensions for [`sync::RwLock`] and [`sync::Mutex`] are provided.
///
/// ## Naming the inner type
///
/// When an `Opaque<T>` is transformed into a Dart type, T's string
/// representation undergoes some transformations to become a valid Dart type:
/// - Rust keywords (dyn, 'static, DartSafe, etc.) are automatically removed.
/// - ASCII alphanumerics are kept, all other characters are ignored.
///
/// ## Trait objects
///
/// Trait objects may be put behind opaque pointers, but they must implement
/// [`DartSafe`] to be safely sent to Dart. For example, this declaration can
/// be used across the FFI border:
///
/// ```rust
/// use flutter_rust_bridge::*;
/// use std::fmt::Debug;
/// use std::panic::{UnwindSafe, RefUnwindSafe};
///
/// // Rust does not allow multiple non-auto traits in trait objects, so this
/// // is one workaround.
/// pub trait DartDebug: DartSafe + Debug {}
///
/// impl<T: DartSafe + Debug> DartDebug for T {}
///
/// pub struct DebugWrapper(pub Opaque<Box<dyn DartDebug>>);
///
/// // creating a DebugWrapper using the opaque_dyn macro
/// let wrap = DebugWrapper(opaque_dyn!("foobar"));
/// // it's possible to name it directly
/// pub struct DebugWrapper2(pub Opaque<Box<dyn Debug + Send + Sync + UnwindSafe + RefUnwindSafe>>);
/// ```
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Opaque<T: ?Sized + DartSafe> {
    pub(crate) ptr: Arc<T>,
}

impl<T: ?Sized + DartSafe> ops::Deref for Opaque<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.ptr.as_ref()
    }
}

impl<T: ?Sized + DartSafe> From<Arc<T>> for Opaque<T> {
    fn from(ptr: Arc<T>) -> Self {
        Self { ptr }
    }
}

impl<T: DartSafe> Opaque<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: Arc::new(value),
        }
    }
}

impl<T: DartSafe> From<Opaque<T>> for ffi::DartCObject {
    fn from(value: Opaque<T>) -> Self {
        let ptr = Arc::into_raw(value.ptr);
        let drop = drop_arc::<T> as *const ();
        let share = share_arc::<T> as *const ();
        let size = mem::size_of::<T>();

        vec![
            ptr.into_dart(),
            drop.into_dart(),
            share.into_dart(),
            size.into_dart(),
        ]
        .into_dart()
    }
}

/// Macro helper to instantiate an `Opaque<dyn Trait>`, as Rust does not
/// support custom DSTs on stable.
///
/// Example:
/// ```rust
/// use std::fmt::Debug;
/// use flutter_rust_bridge::*;
///
/// pub trait MyDebug: DartSafe + Debug {}
///
/// impl<T: DartSafe + Debug> MyDebug for T {}
///
/// let opaque: Opaque<Box<dyn MyDebug>> = opaque_dyn!("foobar");
/// ```
#[macro_export]
macro_rules! opaque_dyn {
    ($ex:expr) => {
        Opaque::new(std::boxed::Box::new($ex))
    };
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
