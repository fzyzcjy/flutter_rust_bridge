use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;
use std::{mem, ops};

/// Macro helper to instantiate an `RustOpaque<dyn Trait>`, as Rust does not
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
/// let opaque: RustOpaque<Box<dyn MyDebug>> = opaque_dyn!("foobar");
/// ```
#[macro_export]
macro_rules! opaque_dyn {
    ($ex:expr) => {
        $crate::RustOpaque::new(::std::boxed::Box::new($ex))
    };
}

impl<T: ?Sized + DartSafe> From<Arc<T>> for RustOpaque<T> {
    fn from(ptr: Arc<T>) -> Self {
        Self { ptr: Some(ptr) }
    }
}

impl<T: DartSafe> RustOpaque<T> {
    pub fn new(value: T) -> Self {
        Self {
            ptr: Some(Arc::new(value)),
        }
    }
}

impl<T: ?Sized + DartSafe> ops::Deref for RustOpaque<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if let Some(ptr) = &self.ptr {
            ptr.as_ref()
        } else {
            panic!("Use after free.")
        }
    }
}

impl<T: DartSafe> RustOpaque<T> {
    pub fn try_unwrap(self) -> Result<T, Self> {
        if let Some(ptr) = self.ptr {
            Arc::try_unwrap(ptr).map_err(RustOpaque::from)
        } else {
            panic!("Use after free.")
        }
    }
}

impl<T: ?Sized + DartSafe> Clone for RustOpaque<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone(),
        }
    }
}
