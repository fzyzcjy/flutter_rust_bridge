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
        use crate::generalized_arc::base_arc::BaseArc;
        use std::fmt::Debug;

        // Do NOT make it `clone` (to test non-clone behavior)
        #[derive(Debug)]
        struct DummyType {
            value: i32,
            on_drop: Box<dyn Fn()>,
        }

        impl DummyType {
            fn new(value: i32, on_drop: Box<dyn Fn()>) -> Self {
                Self { value, on_drop }
            }
        }

        impl Drop for DummyType {
            fn drop(&mut self) {
                self.on_drop()
            }
        }

        #[test]
        fn simple_drop() {
            let a = <$T<DummyType>>::new(DummyType(100));
            assert_eq!(a.as_ref().0, 100);
            drop(a);
        }

        #[test]
        fn simple_clone() {
            let a = <$T<DummyType>>::new(DummyType(100));
            let b = a.clone();
            assert_eq!(a.as_ref().0, 100);
            assert_eq!(b.as_ref().0, 100);

            drop(a);
            assert_eq!(b.as_ref().0, 100);

            drop(b);
        }

        #[test]
        fn try_unwrap_when_1_ref_should_succeed() {
            let a = <$T<DummyType>>::new(DummyType(100));
            assert_eq!(a.try_unwrap().unwrap().0, 100);
        }

        #[test]
        fn try_unwrap_when_2_ref_should_fail() {
            let a = <$T<DummyType>>::new(DummyType(100));
            let _b = a.clone();
            assert!(a.try_unwrap().is_err());
        }

        #[test]
        fn into_inner_when_1_ref_should_succeed() {
            let a = <$T<DummyType>>::new(DummyType(100));
            assert_eq!(a.into_inner().unwrap().0, 100);
        }

        #[test]
        fn into_inner_when_2_ref_should_fail() {
            let a = <$T<DummyType>>::new(DummyType(100));
            let _b = a.clone();
            assert!(a.into_inner().is_none());
        }

        #[test]
        fn from_raw_and_into_raw_simple() {
            let a = <$T<DummyType>>::new(DummyType(100));
            let a_raw = a.into_raw();
            let a_recovered = unsafe { <$T<DummyType>>::from_raw(a_raw) };
            assert_eq!(a_recovered.as_ref().0, 100);
        }

        #[test]
        fn from_raw_and_into_raw_with_another_ref() {
            let a = <$T<DummyType>>::new(DummyType(100));
            let b = a.clone();
            let a_raw = a.into_raw();
            assert_eq!(b.as_ref().0, 100);

            let a_recovered = unsafe { <$T<DummyType>>::from_raw(a_raw) };
            assert_eq!(a_recovered.as_ref().0, 100);
            assert_eq!(b.as_ref().0, 100);
        }

        #[test]
        fn increment_strong_count_and_decrement_strong_count() {
            let a = <$T<DummyType>>::new(DummyType(100));
            let b = a.clone();
            let a_raw = a.into_raw();
            assert_eq!(b.as_ref().0, 100);

            unsafe { <$T<DummyType>>::increment_strong_count(a_raw) };
            assert_eq!(b.as_ref().0, 100);

            unsafe { <$T<DummyType>>::decrement_strong_count(a_raw) };
            assert_eq!(b.as_ref().0, 100);

            let a_recovered = unsafe { <$T<DummyType>>::from_raw(a_raw) };
            assert_eq!(a_recovered.as_ref().0, 100);
            assert_eq!(b.as_ref().0, 100);
        }
    };
}
