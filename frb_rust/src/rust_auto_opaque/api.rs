use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
use crate::rust_auto_opaque::RustAutoOpaqueBase;
use crate::rust_opaque::RustOpaqueBase;
use tokio::sync::RwLock;

impl<T: 'static, A: BaseArc<RustAutoOpaqueInner<T>>> RustAutoOpaqueBase<T, A> {
    pub fn new(value: T) -> Self {
        Self(RustOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(
            value,
        ))))
    }
}
