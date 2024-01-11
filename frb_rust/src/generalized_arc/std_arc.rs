use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_opaque::codec::nom::NomRustOpaqueCodec;
use crate::RustOpaque;
use std::sync::Arc;

pub(crate) struct StdArc<T: ?Sized>(Arc<T>);

impl<T> BaseArc<T> for StdArc<T> {
    fn new(value: T) -> Self {
        Self(Arc::new(value))
    }

    fn try_unwrap(self) -> Result<T, Self> {
        Arc::try_unwrap(self.0)
    }

    fn into_inner(self) -> Option<T> {
        Arc::into_inner(self.0)
    }
}

impl<T: ?Sized> From<Arc<T>> for StdArc<T> {
    fn from(ptr: Arc<T>) -> Self {
        Self(ptr)
    }
}
