use std::sync::atomic::{AtomicU64, Ordering};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct RustAutoOpaqueOrder(u64);

static COUNTER: AtomicU64 = AtomicU64::new(0);

impl RustAutoOpaqueOrder {
    pub(crate) fn new() -> Self {
        let value = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(value)
    }

    #[cfg(test)]
    pub fn new_for_test(value: u64) -> Self {
        Self(value)
    }
}
