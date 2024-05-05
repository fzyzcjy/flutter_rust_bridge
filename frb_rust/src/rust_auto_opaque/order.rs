use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct RustAutoOpaqueOrder(u64);

static COUNTER: AtomicU64 = AtomicU64::new(0);

impl RustAutoOpaqueOrder {
    fn new() -> Self {
        let value = COUNTER.fetch_add(1, Ordering::Relaxed);
        Self(value)
    }
}
