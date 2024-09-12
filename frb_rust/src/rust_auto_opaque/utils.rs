use crate::for_generated::{BaseArc, RustAutoOpaqueBase};
use crate::rust_auto_opaque::inner;

impl<T: 'static + Default, A: BaseArc<inner::RustAutoOpaqueInner<T>>> Default
    for RustAutoOpaqueBase<T, A>
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: 'static, A: BaseArc<inner::RustAutoOpaqueInner<T>>> Clone for RustAutoOpaqueBase<T, A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
