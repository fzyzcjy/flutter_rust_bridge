// NOTE: Some functions are not in this trait, because different implementors have different `unsafe` keywords
// and that is not supported in Rust yet
pub trait BaseArc<T: ?Sized>: Clone + AsRef<T> {
    fn new(value: T) -> Self
    where
        T: Sized;

    fn try_unwrap(self) -> Result<T, Self>
    where
        Self: Sized,
        T: Sized;

    fn into_inner(self) -> Option<T>
    where
        T: Sized;

    fn into_raw(self) -> usize;
}

#[doc(hidden)]
#[macro_export]
macro_rules! base_arc_generate_tests {
    ($T:tt) => {
        use std::fmt::Debug;
        use std::sync::atomic::{AtomicBool, Ordering};
        use $crate::generalized_arc::base_arc::BaseArc;

        // Do NOT make it `clone` (to test non-clone behavior)
        struct DummyType {
            value: i32,
            on_drop: std::sync::Arc<AtomicBool>,
        }

        impl DummyType {
            fn new(value: i32, on_drop: std::sync::Arc<AtomicBool>) -> Self {
                Self { value, on_drop }
            }
        }

        impl Debug for DummyType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "DummyType::new({})", self.value)
            }
        }

        impl Drop for DummyType {
            fn drop(&mut self) {
                self.on_drop.store(true, Ordering::SeqCst)
            }
        }

        fn create() -> ($T<DummyType>, std::sync::Arc<AtomicBool>) {
            let dropped = std::sync::Arc::new(AtomicBool::new(false));
            let arc = <$T<DummyType>>::new(DummyType::new(100, dropped.clone()));
            (arc, dropped)
        }

        #[test]
        fn simple_drop() {
            let (a, dropped) = create();
            assert_eq!(a.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(a);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn simple_clone() {
            let (a, dropped) = create();
            let b = a.clone();
            assert_eq!(a.as_ref().value, 100);
            assert_eq!(b.as_ref().value, 100);

            drop(a);
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            drop(b);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn try_unwrap_when_1_ref_should_succeed() {
            let (a, dropped) = create();
            let inner = a.try_unwrap().unwrap();
            assert_eq!(inner.value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(inner);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn try_unwrap_when_2_ref_should_fail() {
            let (a, dropped) = create();
            let b = a.clone();
            assert!(a.try_unwrap().is_err());
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(b);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn into_inner_when_1_ref_should_succeed() {
            let (a, dropped) = create();
            let inner = a.into_inner().unwrap();
            assert_eq!(inner.value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(inner);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn into_inner_when_2_ref_should_fail() {
            let (a, dropped) = create();
            let b = a.clone();
            assert!(a.into_inner().is_none());
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(b);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn from_raw_and_into_raw_simple() {
            let (a, dropped) = create();
            let a_raw = a.into_raw();
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            #[allow(unused_unsafe)]
            let a_recovered = unsafe { <$T<DummyType>>::from_raw(a_raw) };
            assert_eq!(a_recovered.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(a_recovered);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn from_raw_and_into_raw_with_another_ref() {
            let (a, dropped) = create();
            let b = a.clone();
            let a_raw = a.into_raw();
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            #[allow(unused_unsafe)]
            let a_recovered = unsafe { <$T<DummyType>>::from_raw(a_raw) };
            assert_eq!(a_recovered.as_ref().value, 100);
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            drop(b);
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(a_recovered);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn increment_strong_count_and_decrement_strong_count() {
            let (a, dropped) = create();
            let b = a.clone();
            let a_raw = a.into_raw();
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            #[allow(unused_unsafe)]
            unsafe {
                <$T<DummyType>>::increment_strong_count(a_raw)
            };
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            #[allow(unused_unsafe)]
            unsafe {
                <$T<DummyType>>::decrement_strong_count(a_raw)
            };
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            #[allow(unused_unsafe)]
            let a_recovered = unsafe { <$T<DummyType>>::from_raw(a_raw) };
            assert_eq!(a_recovered.as_ref().value, 100);
            assert_eq!(b.as_ref().value, 100);
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            drop(b);
            assert_eq!(dropped.load(Ordering::SeqCst), false);
            drop(a_recovered);
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }

        #[test]
        fn simple_decrement_strong_count() {
            let (a, dropped) = create();
            let a_raw = a.into_raw();
            assert_eq!(dropped.load(Ordering::SeqCst), false);

            #[allow(unused_unsafe)]
            unsafe {
                <$T<DummyType>>::decrement_strong_count(a_raw)
            };
            assert_eq!(dropped.load(Ordering::SeqCst), true);
        }
    };
}
