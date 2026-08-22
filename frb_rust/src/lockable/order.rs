use crate::misc::atomic::{AtomicU64, Ordering};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct LockableOrder(u64);

static COUNTER: AtomicU64 = AtomicU64::new(0);

impl LockableOrder {
    pub(crate) fn new() -> Self {
        let value = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(value)
    }

    #[cfg(test)]
    pub fn new_for_test(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Allocates monotonically increasing lockable orders.
    fn allocates_monotonically_increasing_orders() {
        let first = LockableOrder::new();
        let second = LockableOrder::new();

        assert!(second > first);
    }
}
