use flutter_rust_bridge::frb;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicI32, Ordering};

lazy_static! {
    static ref DROP_COUNT: AtomicI32 = AtomicI32::new(0);
}

#[frb(opaque)]
pub struct DroppableTwinNormal;

impl Drop for DroppableTwinNormal {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

impl DroppableTwinNormal {
    pub fn new() -> DroppableTwinNormal {
        Self
    }

    pub fn simple_method_twin_normal(&self) {}

    pub fn get_drop_count_twin_normal() -> i32 {
        DROP_COUNT.load(Ordering::SeqCst)
    }
}
