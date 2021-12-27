use allo_isolate::{ffi::DartCObject, IntoDart};
use std::sync::Arc;
pub use std::sync::RwLock;

/// Newtype wrapper around [`Arc<RwLock<T>>`] to be converted into an opaque pointer.
/// If interior mutability is not required, [`Arc<T>`] is also [IntoDart].
#[repr(transparent)]
pub struct Opaque<T>(pub Arc<RwLock<T>>);

impl<T> Opaque<T> {
    pub fn new(value: T) -> Self {
        Opaque(Arc::new(RwLock::new(value)))
    }
}

impl<T> IntoDart for Opaque<T> {
    fn into_dart(self) -> DartCObject {
        self.0.into_dart()
    }
}

impl<T> std::ops::Deref for Opaque<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
