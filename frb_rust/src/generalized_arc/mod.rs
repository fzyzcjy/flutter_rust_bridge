pub(crate) mod base_arc;
pub(crate) mod map_based_arc;
pub(crate) mod std_arc;

#[cfg(test)]
mod tests {
    use crate::generalized_arc::base_arc::BaseArc;
    use crate::generalized_arc::map_based_arc::MapBasedArc;
    use crate::generalized_arc::std_arc::StdArc;

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

    fn body<T: BaseArc<DummyType>>() {
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
}
