use super::RustOpaque;
use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_opaque::codec::nom::NomRustOpaqueCodec;
use crate::rust_opaque::codec::BaseRustOpaqueCodec;
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

impl<T: ?Sized + 'static> From<Arc<T>> for RustOpaque<T, NomRustOpaqueCodec> {
    fn from(ptr: Arc<T>) -> Self {
        Self::from_arc(ptr.into())
    }
}

impl<T, C: BaseRustOpaqueCodec<T>> RustOpaque<T, C> {
    pub fn new(value: T) -> Self {
        Self::from_arc(C::Arc::new(value))
    }
}

impl<T: ?Sized, C: BaseRustOpaqueCodec<T>> RustOpaque<T, C> {
    // `pub` mainly because dart2rust.rs needs it
    #[doc(hidden)]
    pub fn from_arc(arc: C::Arc) -> Self {
        Self {
            arc,
            _phantom_t: PhantomData,
            _phantom_c: PhantomData,
        }
    }
}

impl<T: ?Sized, C: BaseRustOpaqueCodec<T>> ops::Deref for RustOpaque<T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.arc.as_ref()
    }
}

impl<T, C: BaseRustOpaqueCodec<T>> RustOpaque<T, C> {
    pub fn try_unwrap(self) -> Result<T, Self> {
        C::Arc::try_unwrap(self.arc).map_err(Self::from_arc)
    }

    pub fn into_inner(self) -> Option<T> {
        C::Arc::into_inner(self.arc)
    }
}

impl<T: ?Sized + 'static, C: BaseRustOpaqueCodec<T>> Clone for RustOpaque<T, C> {
    fn clone(&self) -> Self {
        Self::from_arc(self.arc.clone())
    }
}
