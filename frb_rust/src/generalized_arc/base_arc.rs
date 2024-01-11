use std::fmt::Debug;

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
    () => {
        use crate::generalized_arc::base_arc::BaseArc;
        use crate::generalized_arc::map_based_arc::MapBasedArc;
        use crate::generalized_arc::std_arc::StdArc;
        use std::fmt::Debug;

        // Do NOT make it `clone` (to test non-clone behavior)
        #[derive(Debug)]
        struct DummyType(i32);

        #[test]
        fn test_std_arc() {
            body::<StdArc<DummyType>>();
        }

        #[test]
        fn test_map_based_arc() {
            body::<MapBasedArc<DummyType>>();
        }

        fn body<T: BaseArc<DummyType> + Debug>() {
            // Simple drop
            {
                let a = T::new(DummyType(100));
                assert_eq!(a.as_ref().0, 100);
                drop(a);
            }

            // Simple clone
            {
                let a = T::new(DummyType(100));
                let b = a.clone();
                assert_eq!(a.as_ref().0, 100);
                assert_eq!(b.as_ref().0, 100);

                drop(a);
                assert_eq!(b.as_ref().0, 100);

                drop(b);
            }

            // try_unwrap succeed when only 1 ref
            {
                let a = T::new(DummyType(100));
                assert_eq!(a.try_unwrap().unwrap().0, 100);
            }

            // try_unwrap fail when multi ref
            {
                let a = T::new(DummyType(100));
                let b = a.clone();
                assert!(a.try_unwrap().is_err());
                assert!(b.try_unwrap().is_err());

                drop(a);
                assert_eq!(b.try_unwrap().unwrap().0, 100);
            }

            // into_inner succeed when only 1 ref
            {
                let a = T::new(DummyType(100));
                assert_eq!(a.into_inner().unwrap().0, 100);
            }

            // into_inner fail when multi ref
            {
                let a = T::new(DummyType(100));
                let b = a.clone();
                assert!(a.into_inner().is_none());
                assert!(b.into_inner().is_none());

                drop(a);
                assert_eq!(b.into_inner().unwrap().0, 100);
            }

            // from_raw & into_raw
            {
                todo!();
            }

            // increment_strong_count & decrement_strong_count
            {
                todo!();
            }
        }
    };
}
