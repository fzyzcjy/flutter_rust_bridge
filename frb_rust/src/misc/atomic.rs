// For some reason Clippy thinks that the `AtomicBool` import is unused, but it's needed by
// `base_arc_generate_tests!`
#[cfg(feature = "portable-atomic")]
#[allow(unused_imports)]
pub(crate) use portable_atomic::{AtomicBool, AtomicI32, AtomicU64};
pub(crate) use std::sync::atomic::Ordering;
#[cfg(not(feature = "portable-atomic"))]
pub(crate) use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};

#[cfg(test)]
mod tests {
    use super::{AtomicBool, AtomicI32, AtomicU64, Ordering};

    /// Stores and loads every atomic alias used by the runtime.
    #[test]
    fn test_atomic_aliases_store_and_load_values() {
        let boolean = AtomicBool::new(false);
        let signed = AtomicI32::new(0);
        let unsigned = AtomicU64::new(0);

        boolean.store(true, Ordering::SeqCst);
        signed.store(-1, Ordering::SeqCst);
        unsigned.store(1, Ordering::SeqCst);

        assert!(boolean.load(Ordering::SeqCst));
        assert_eq!(signed.load(Ordering::SeqCst), -1);
        assert_eq!(unsigned.load(Ordering::SeqCst), 1);
    }
}
