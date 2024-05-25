use crate::generalized_arc::base_arc::BaseArc;
use crate::platform_types::DartAbi;
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};

impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> From<RustAutoOpaqueBase<T, A>> for DartAbi {
    fn from(value: RustAutoOpaqueBase<T, A>) -> Self {
        value.0.into()
    }
}
