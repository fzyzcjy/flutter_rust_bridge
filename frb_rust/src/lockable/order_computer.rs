use crate::lockable::immediate_invalid_borrow_checker::check_no_immediate_invalid_borrow;
use crate::lockable::order_info::LockableOrderInfo;

pub fn lockable_compute_decode_order(infos: Vec<LockableOrderInfo>) -> Vec<usize> {
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
