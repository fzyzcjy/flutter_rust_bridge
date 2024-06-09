use crate::rust_auto_opaque::dart2rust_implicit::RustAutoOpaqueLockOrderInfo;

pub(super) fn check_no_immediate_invalid_borrow(
    sorted_infos: &[RustAutoOpaqueLockOrderInfo],
) -> bool {
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
                    object_order: crate::rust_auto_opaque::order::RustAutoOpaqueOrder::new_for_test(
                        100
                    ),
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
                        object_order:
                            crate::rust_auto_opaque::order::RustAutoOpaqueOrder::new_for_test(100),
                    },
                    RustAutoOpaqueLockOrderInfo {
                        index: 1,
                        mutable: mutable_b,
                        object_order:
                            crate::rust_auto_opaque::order::RustAutoOpaqueOrder::new_for_test(100),
                    }
                ]),
                expect
            );
        }

        assert!(check_no_immediate_invalid_borrow(&[
            RustAutoOpaqueLockOrderInfo {
                index: 0,
                mutable: true,
                object_order: crate::rust_auto_opaque::order::RustAutoOpaqueOrder::new_for_test(
                    100
                ),
            },
            RustAutoOpaqueLockOrderInfo {
                index: 1,
                mutable: true,
                object_order: crate::rust_auto_opaque::order::RustAutoOpaqueOrder::new_for_test(
                    101
                ),
            }
        ]));
    }
}
