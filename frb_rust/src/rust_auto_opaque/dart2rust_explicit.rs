use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
use crate::rust_opaque::RustOpaqueBase;

pub fn rust_auto_opaque_explicit_decode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    raw: RustOpaqueBase<RustAutoOpaqueInner<T>, A>,
) -> RustAutoOpaqueBase<T, A> {
    RustAutoOpaqueBase(raw)
}
