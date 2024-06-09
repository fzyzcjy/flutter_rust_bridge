use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::inner::RustAutoOpaqueInner;
use crate::rust_auto_opaque::order::RustAutoOpaqueOrder;
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

pub fn rust_auto_opaque_lock_order_info<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    opaque: &RustOpaqueBase<RustAutoOpaqueInner<T>, A>,
    index: usize,
    mutable: bool,
) -> RustAutoOpaqueLockOrderInfo {
    RustAutoOpaqueLockOrderInfo {
        index,
        mutable,
        object_order: opaque.order,
    }
}

pub struct RustAutoOpaqueLockOrderInfo {
    index: usize,
    mutable: bool,
    object_order: RustAutoOpaqueOrder,
}

pub fn rust_auto_opaque_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    value: T,
) -> RustOpaqueBase<RustAutoOpaqueInner<T>, A> {
    RustOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(value)))
}

pub fn rust_auto_opaque_decode_compute_order(
    infos: Vec<RustAutoOpaqueLockOrderInfo>,
) -> Vec<usize> {
    let sorted_infos = {
        let mut x = infos;
        x.sort_unstable_by_key(|info| info.object_order);
        x
    };

    assert!(
        check_no_immediate_invalid_borrow(&sorted_infos),
        "Cannot borrow an object mutably, and at the same time borrow again in another argument"
    );

    sorted_infos.into_iter().map(|info| info.index).collect()
}

