use super::RustOpaque;
use std::ops;
use std::sync::Arc;

/// Macro helper to instantiate an `RustOpaque<dyn Trait>`, as Rust does not
/// support custom DSTs on stable.
///
/// Example:
/// ```rust
/// use std::fmt::Debug;
/// use flutter_rust_bridge::*;
///
/// let opaque: RustOpaque<Box<dyn Debug>> = opaque_dyn!("foobar");
/// ```
#[macro_export]
macro_rules! opaque_dyn {
    ($ex:expr) => {
        $crate::RustOpaque::new(::std::boxed::Box::new($ex))
    };
}

impl<T: ?Sized> From<Arc<T>> for RustOpaque<T> {
    fn from(ptr: Arc<T>) -> Self {
        Self { arc: ptr }
    }
}

impl<T> RustOpaque<T> {
    pub fn new(value: T) -> Self {
        Self {
            arc: Arc::new(value),
        }
    }
}

impl<T: ?Sized> ops::Deref for RustOpaque<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.arc.as_ref()
    }
}

impl<T> RustOpaque<T> {
    pub fn try_unwrap(self) -> Result<T, Self> {
        Arc::try_unwrap(self.arc).map_err(RustOpaque::from)
    }

    pub fn into_inner(self) -> Option<T> {
        Arc::into_inner(self.arc)
    }
}

impl<T: ?Sized> Clone for RustOpaque<T> {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}
