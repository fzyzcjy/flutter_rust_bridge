use super::RustOpaque;
use crate::for_generated::StdArc;
use crate::generalized_arc::base_arc::BaseArc;
use std::marker::PhantomData;
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

impl<T: ?Sized + 'static> From<Arc<T>> for RustOpaque<T, StdArc<T>> {
    fn from(ptr: Arc<T>) -> Self {
        Self::from_arc(ptr.into())
    }
}

impl<T, A: BaseArc<T>> RustOpaque<T, A> {
    pub fn new(value: T) -> Self {
        Self::from_arc(A::new(value))
    }
}

impl<T: ?Sized, A: BaseArc<T>> RustOpaque<T, A> {
    // `pub` mainly because dart2rust.rs needs it
    #[doc(hidden)]
    pub fn from_arc(arc: A) -> Self {
        Self {
            arc,
            _phantom: PhantomData,
        }
    }
}

impl<T: ?Sized, A: BaseArc<T>> ops::Deref for RustOpaque<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.arc.as_ref()
    }
}

impl<T, A: BaseArc<T>> RustOpaque<T, A> {
    pub fn try_unwrap(self) -> Result<T, Self> {
        A::try_unwrap(self.arc).map_err(Self::from_arc)
    }

    pub fn into_inner(self) -> Option<T> {
        A::into_inner(self.arc)
    }
}

impl<T: ?Sized + 'static, A: BaseArc<T>> Clone for RustOpaque<T, A> {
    fn clone(&self) -> Self {
        Self::from_arc(self.arc.clone())
    }
}
