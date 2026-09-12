use flutter_rust_bridge::frb;

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[frb(sync)]
pub fn reproduce_moi_arc_release_contention() {
    contention_repro::release_while_pool_is_read_locked();
}

#[frb(sync)]
pub fn moi_arc_contention_value_was_dropped() -> bool {
    contention_repro::was_dropped()
}

#[frb(ignore)]
mod contention_repro {
    use flutter_rust_bridge::for_generated::BaseArc;
    use std::sync::atomic::{AtomicBool, Ordering};

    flutter_rust_bridge::frb_generated_moi_arc_def!();
    flutter_rust_bridge::frb_generated_moi_arc_impl_value!(ContentionValue);

    static DROPPED: AtomicBool = AtomicBool::new(false);

    pub(super) fn release_while_pool_is_read_locked() {
        DROPPED.store(false, Ordering::SeqCst);
        let raw = MoiArc::new(ContentionValue).into_raw();
        let guard = ContentionValue::get_pool().read().unwrap();
        MoiArc::<ContentionValue>::decrement_strong_count(raw);
        drop(guard);
    }

    pub(super) fn was_dropped() -> bool {
        DROPPED.load(Ordering::SeqCst)
    }

    struct ContentionValue;

    impl Drop for ContentionValue {
        fn drop(&mut self) {
            DROPPED.store(true, Ordering::SeqCst);
        }
    }
}
