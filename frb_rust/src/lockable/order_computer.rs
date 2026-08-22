use crate::lockable::immediate_invalid_borrow_checker::check_no_immediate_invalid_borrow;
use crate::lockable::order_info::LockableOrderInfo;

pub fn lockable_compute_decode_order(infos: Vec<LockableOrderInfo>) -> Vec<usize> {
    let sorted_infos = {
        let mut x = infos;
        x.sort_unstable_by_key(|info| info.object_order);
        x
    };

    if !check_no_immediate_invalid_borrow(&sorted_infos) {
        panic_or_web_throw(
            "Cannot borrow an object mutably, and at the same time borrow again in another argument",
        );
    }

    sorted_infos.into_iter().map(|info| info.index).collect()
}

#[cfg(not(target_family = "wasm"))]
fn panic_or_web_throw(message: &str) -> ! {
    panic!("{message}")
}

#[cfg(target_family = "wasm")]
fn panic_or_web_throw(message: &str) -> ! {
    wasm_bindgen::throw_str(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lockable::order::LockableOrder;

    fn info(order: u64, index: usize, mutable: bool) -> LockableOrderInfo {
        LockableOrderInfo {
            object_order: LockableOrder::new_for_test(order),
            index,
            mutable,
        }
    }

    #[test]
    /// Sorts decode arguments by their lockable order.
    fn sorts_indices_by_lockable_order() {
        assert_eq!(
            lockable_compute_decode_order(vec![info(2, 0, false), info(1, 1, false)]),
            [1, 0]
        );
    }

    #[cfg(not(target_family = "wasm"))]
    #[test]
    /// Panics when one object is borrowed mutably more than once.
    fn rejects_invalid_mutable_borrow() {
        let error = std::panic::catch_unwind(|| {
            lockable_compute_decode_order(vec![info(1, 0, true), info(1, 1, false)]);
        })
        .unwrap_err();
        let message = error
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| error.downcast_ref::<&str>().copied());

        assert_eq!(
            message,
            Some("Cannot borrow an object mutably, and at the same time borrow again in another argument")
        );
    }
}
