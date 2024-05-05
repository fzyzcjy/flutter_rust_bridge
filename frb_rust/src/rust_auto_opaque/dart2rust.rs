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
            object_order: self.order,
        }
    }
}

pub struct RustAutoOpaqueLockOrderInfo {
    index: usize,
    mutable: bool,
    object_order: RustAutoOpaqueOrder,
}

pub fn rust_auto_opaque_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    value: T,
) -> RustAutoOpaqueBase<T, A> {
    RustAutoOpaqueBase::new(RustAutoOpaqueInner::new(RwLock::new(value)))
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

fn check_no_immediate_invalid_borrow(sorted_infos: &[RustAutoOpaqueLockOrderInfo]) -> bool {
    let mut last_object = None;
    let mut checker = ImmediateInvalidBorrowChecker::default();

    for info in sorted_infos {
        if Some(info.object_order) != last_object {
            last_object = Some(info.object_order);
            checker = ImmediateInvalidBorrowChecker::default();
        }

        if !checker.update(info.mutable) {
            return false;
        }
    }

    true
}

#[derive(Default)]
struct ImmediateInvalidBorrowChecker {
    count: usize,
    has_mutable: bool,
}

impl ImmediateInvalidBorrowChecker {
    fn update(&mut self, mutable: bool) -> bool {
        self.count += 1;
        self.has_mutable |= mutable;
        (!self.has_mutable) || (self.count <= 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_no_immediate_invalid_borrow() {
        assert!(check_no_immediate_invalid_borrow(&[]));

        for mutable in [false, true] {
            assert!(check_no_immediate_invalid_borrow(&[
                RustAutoOpaqueLockOrderInfo {
                    index: 0,
                    mutable,
                    object_order: RustAutoOpaqueOrder::new_for_test(100),
                }
            ]));
        }

        for (mutable_a, mutable_b, expect) in [
            (false, false, true),
            (false, true, false),
            (true, false, false),
            (true, true, false),
        ] {
            assert_eq!(
                check_no_immediate_invalid_borrow(&[
                    RustAutoOpaqueLockOrderInfo {
                        index: 0,
                        mutable: mutable_a,
                        object_order: RustAutoOpaqueOrder::new_for_test(100),
                    },
                    RustAutoOpaqueLockOrderInfo {
                        index: 1,
                        mutable: mutable_b,
                        object_order: RustAutoOpaqueOrder::new_for_test(100),
                    }
                ]),
                expect
            );
        }

        assert!(check_no_immediate_invalid_borrow(&[
            RustAutoOpaqueLockOrderInfo {
                index: 0,
                mutable: true,
                object_order: RustAutoOpaqueOrder::new_for_test(100),
            },
            RustAutoOpaqueLockOrderInfo {
                index: 1,
                mutable: true,
                object_order: RustAutoOpaqueOrder::new_for_test(101),
            }
        ]));
    }
}
