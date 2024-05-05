use crate::for_generated::BaseArc;
use crate::rust_async::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::rust_auto_opaque::order::RustAutoOpaqueOrder;
use crate::rust_auto_opaque::{RustAutoOpaqueBase, RustAutoOpaqueInner};
use std::iter::zip;

impl<T, A: BaseArc<RustAutoOpaqueInner<T>>> RustAutoOpaqueBase<T, A> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    pub fn rust_auto_opaque_decode_owned(self) -> T {
        // frb-coverage:ignore-end
        self.into_inner()
            .expect("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")
            .data
            .into_inner()
    }

    pub fn rust_auto_opaque_decode_sync_ref(&self) -> RwLockReadGuard<'_, T> {
        self.data.blocking_read()
    }

    pub fn rust_auto_opaque_decode_sync_ref_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.data.blocking_write()
    }

    pub async fn rust_auto_opaque_decode_async_ref(&self) -> RwLockReadGuard<'_, T> {
        self.data.read().await
    }

    pub async fn rust_auto_opaque_decode_async_ref_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.data.write().await
    }

    pub fn rust_auto_opaque_lock_order_info(
        &self,
        index: usize,
        mutable: bool,
    ) -> RustAutoOpaqueLockOrderInfo {
        RustAutoOpaqueLockOrderInfo {
            index,
            mutable,
            lock_order: self.order,
        }
    }
}

pub struct RustAutoOpaqueLockOrderInfo {
    index: usize,
    mutable: bool,
    lock_order: RustAutoOpaqueOrder,
}

pub fn rust_auto_opaque_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    value: T,
) -> RustAutoOpaqueBase<T, A> {
    RustAutoOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(value)))
}

pub fn rust_auto_opaque_decode_compute_order(lock_orders: &[RustAutoOpaqueOrder]) -> Vec<usize> {
    let mut sorted_index_and_lock_orders: Vec<_> =
        lock_orders.iter().cloned().enumerate().collect();
    sorted_index_and_lock_orders.sort_unstable_by_key(|(_index, order)| order);

    assert!(
        check_no_mut_multi_borrow(&sorted_index_and_lock_orders),
        "Cannot borrow an object mutably, and at the same time borrow again in another argument"
    );

    sorted_index_and_lock_orders
        .into_iter()
        .map(|index, _order| index)
        .collect()
}

fn check_no_mut_multi_borrow(
    sorted_index_and_lock_orders: &[(usize, RustAutoOpaqueOrder)],
) -> bool {
    TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_no_mut_multi_borrow() {
        check_no_mut_multi_borrow(&[()])
    }
}
