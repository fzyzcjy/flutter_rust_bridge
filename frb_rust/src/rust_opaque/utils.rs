use super::RustOpaque;
use crate::generalized_arc::base_arc::BaseArc;
use crate::generalized_arc::std_arc::StdArc;
use crate::rust_opaque::codec::nom::NomRustOpaqueCodec;
use crate::rust_opaque::codec::BaseRustOpaqueCodec;
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

impl<T: ?Sized> From<Arc<T>> for RustOpaque<T, NomRustOpaqueCodec> {
    fn from(ptr: Arc<T>) -> Self {
        Self { arc: ptr.into() }
    }
}

impl<T, C: BaseRustOpaqueCodec> RustOpaque<T, C> {
    pub fn new(value: T) -> Self {
        Self {
            arc: C::Arc::new(value),
        }
    }
}

impl<T: ?Sized, C: BaseRustOpaqueCodec> ops::Deref for RustOpaque<T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.arc.as_ref()
    }
}

impl<T, C: BaseRustOpaqueCodec> RustOpaque<T, C> {
    pub fn try_unwrap(self) -> Result<T, Self> {
        C::Arc::try_unwrap(self.arc).map_err(RustOpaque::from)
    }

    pub fn into_inner(self) -> Option<T> {
        C::Arc::into_inner(self.arc)
    }
}

impl<T: ?Sized, C: BaseRustOpaqueCodec> Clone for RustOpaque<T, C> {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}
