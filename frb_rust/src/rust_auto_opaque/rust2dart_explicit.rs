use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
use crate::rust_opaque::RustOpaqueBase;

pub fn rust_auto_opaque_explicit_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    raw: RustAutoOpaqueBase<T, A>,
) -> RustOpaqueBase<RustAutoOpaqueInner<T>, A> {
    raw.0
}
