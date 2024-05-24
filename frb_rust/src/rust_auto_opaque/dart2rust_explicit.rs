use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};

impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> RustAutoOpaqueBase<T, A> {}
