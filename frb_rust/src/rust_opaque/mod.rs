pub(crate) mod dart2rust;
pub(crate) mod rust2dart;
pub(crate) mod utils;

use std::sync::Arc;

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
/// When an `RustOpaque<T>` is transformed into a Dart type, T's string
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
/// pub struct DebugWrapper(pub RustOpaque<Box<dyn DartDebug>>);
///
/// // creating a DebugWrapper using the opaque_dyn macro
/// let wrap = DebugWrapper(opaque_dyn!("foobar"));
/// // it's possible to name it directly
/// pub struct DebugWrapper2(pub RustOpaque<Box<dyn Debug + Send + Sync + UnwindSafe + RefUnwindSafe>>);
/// ```
#[repr(transparent)]
#[derive(Debug)]
pub struct RustOpaque<T: ?Sized + DartSafe> {
    arc: Arc<T>,
}

// TODO remove it?
pub trait DartSafe {}
impl<T> DartSafe for T {}
// /// Marker trait for types that are safe to share with Dart and can be dropped
// /// safely in case of a panic.
// pub trait DartSafe: UnwindSafe + RefUnwindSafe {}
//
// impl<T: UnwindSafe + RefUnwindSafe> DartSafe for T {}
