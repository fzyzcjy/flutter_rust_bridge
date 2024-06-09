use crate::for_generated::{BaseArc, Lockable, LockableOrder, RustAutoOpaqueBase};
use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
use crate::rust_opaque::RustOpaqueBase;
use tokio::sync::RwLock;

// NOTE: Make these functions instead of methods, thus we can control its visibility by exporting
// only through `for_generated::...` and do not expose to end users.

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
pub fn rust_auto_opaque_decode_owned<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    opaque: RustOpaqueBase<RustAutoOpaqueInner<T>, A>,
) -> T {
    // frb-coverage:ignore-end
    opaque.into_inner()
            .expect("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")
            .data
            .into_inner()
}

pub fn rust_auto_opaque_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    value: T,
) -> RustOpaqueBase<RustAutoOpaqueInner<T>, A> {
    RustOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(value)))
}

pub fn rust_auto_opaque_lockable_order<T: Send + Sync, A: BaseArc<RustAutoOpaqueInner<T>>>(
    opaque: &RustAutoOpaqueBase<T, A>,
) -> LockableOrder {
    opaque.0.lockable_order()
}
