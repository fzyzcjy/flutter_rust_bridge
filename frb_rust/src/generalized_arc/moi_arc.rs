struct MoiArcPoolValue<T: ?Sized> {
    // Real reference counting of this MoiArc
    ref_count: i32,
    // Use (std) Arc merely for lifetime
    value: std::sync::Arc<T>,
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_moi_arc_def {
    () => {
        use std::collections::HashMap;
        use std::marker::PhantomData;
        use std::sync::Arc;

        #[derive(Debug)]
        pub struct MoiArc<T: ?Sized + MoiArcValue> {
            // `Option` for correct dropping
            object_id: Option<ObjectId>,
            // Mainly for `as_ref`. `Option` for correct dropping
            value: Option<Arc<T>>,
            _phantom: PhantomData<T>,
        }

        impl<T: ?Sized + MoiArcValue> Drop for MoiArc<T> {
            fn drop(&mut self) {
                if let Some(object_id) = self.object_id {
                    Self::decrement_strong_count(object_id);
                }
            }
        }

        impl<T: ?Sized + MoiArcValue> AsRef<T> for MoiArc<T> {
            fn as_ref(&self) -> &T {
                self.value.as_ref().unwrap().as_ref()
            }
        }

        impl<T: ?Sized + MoiArcValue> $crate::for_generated::BaseArc<T> for MoiArc<T> {
            fn new(value: T) -> Self
            where
                T: Sized,
            {
                let mut pool = T::get_pool().write();
                let object_id = pool.next_id();

                let value = Arc::new(value);

                let old_value = pool.map.insert(
                    object_id,
                    $crate::for_generated::moi_arc::MoiArcPoolValue {
                        ref_count: 1,
                        value: value.clone(),
                    },
                );
                assert!(old_value.is_none());

                Self {
                    object_id: Some(object_id),
                    value: Some(value),
                    _phantom: PhantomData,
                }
            }

            fn try_unwrap(mut self) -> Result<T, Self>
            where
                T: Sized,
            {
                // NOTE: Ensure lock is held during all operations to avoid racing conditions
                let pool = &mut T::get_pool().write();

                if pool.map.get(&self.object_id.unwrap()).unwrap().ref_count == 1 {
                    Self::decrement_strong_count_raw(self.object_id.unwrap(), pool);
                    // `take`, such that the `drop` will not decrease ref count
                    self.object_id.take().unwrap();
                    Ok(Arc::into_inner(self.value.take().unwrap()).unwrap())
                } else {
                    Err(self)
                }
            }

            fn into_inner(self) -> Option<T>
            where
                T: Sized,
            {
                self.try_unwrap().ok()
            }

            fn into_raw(mut self) -> usize {
                // `take`, such that the `drop` will not decrease ref count
                self.object_id.take().unwrap()
            }
        }

        impl<T: ?Sized + MoiArcValue> Clone for MoiArc<T> {
            fn clone(&self) -> Self {
                Self::increment_strong_count(self.object_id.unwrap());

                Self {
                    object_id: self.object_id,
                    value: self.value.clone(),
                    _phantom: PhantomData,
                }
            }
        }

        impl<T: ?Sized + MoiArcValue> MoiArc<T> {
            pub(crate) fn from_raw(raw: usize) -> Self
            where
                T: Sized,
            {
                let map = &T::get_pool().read().map;

                Self {
                    object_id: Some(raw),
                    value: Some(map.get(&raw).unwrap().value.clone()),
                    _phantom: PhantomData,
                }
            }

            pub fn increment_strong_count(raw: usize) {
                let map = &mut T::get_pool().write().map;
                map.get_mut(&raw).unwrap().ref_count += 1;
            }

            pub fn decrement_strong_count(raw: usize) {
                Self::decrement_strong_count_raw(raw, &mut T::get_pool().write())
            }

            fn decrement_strong_count_raw(raw: usize, pool: &mut MoiArcPoolInner<T>) {
                let value = pool.map.get_mut(&raw).unwrap();
                value.ref_count -= 1;
                if value.ref_count == 0 {
                    pool.map.remove(&raw).unwrap();
                }
            }
        }

        pub trait MoiArcValue: 'static {
            fn get_pool() -> &'static MoiArcPool<Self>;
        }

        type ObjectId = usize;

        pub type MoiArcPool<T> = $crate::for_generated::parking_lot::RwLock<MoiArcPoolInner<T>>;

        pub struct MoiArcPoolInner<T: ?Sized> {
            map: HashMap<ObjectId, $crate::for_generated::moi_arc::MoiArcPoolValue<T>>,
            next_id: ObjectId,
        }

        impl<T: ?Sized> Default for MoiArcPoolInner<T> {
            fn default() -> Self {
                Self {
                    map: HashMap::new(),
                    next_id: Self::MIN_ID,
                }
            }
        }

        impl<T: ?Sized> MoiArcPoolInner<T> {
            const MIN_ID: ObjectId = 1;

            fn next_id(&mut self) -> ObjectId {
                let ans = self.next_id;

                self.next_id = if self.next_id == ObjectId::MAX {
                    Self::MIN_ID
                } else {
                    self.next_id + 1
                };

                ans
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_moi_arc_impl_value {
    ($T:ty) => {
        impl MoiArcValue for $T {
            fn get_pool() -> &'static MoiArcPool<Self> {
                $crate::for_generated::lazy_static! {
                    static ref POOL: MoiArcPool<$T> = MoiArcPool::new(Default::default());
                }
                &POOL
            }
        }
    };
}

#[cfg(test)]
mod tests {
    crate::frb_generated_moi_arc_def!();
    crate::frb_generated_moi_rust_opaque_codec_def!();

    #[test]
    fn test_next_id() {
        let mut pool = MoiArcPoolInner::<String>::default();
        assert_eq!(pool.next_id(), 1);
        assert_eq!(pool.next_id(), 2);
        assert_eq!(pool.next_id(), 3);

        pool.next_id = ObjectId::MAX - 2;
        assert_eq!(pool.next_id(), ObjectId::MAX - 2);
        assert_eq!(pool.next_id(), ObjectId::MAX - 1);
        assert_eq!(pool.next_id(), ObjectId::MAX);
        assert_eq!(pool.next_id(), 1); // NOTE: still not zero
        assert_eq!(pool.next_id(), 2);
        assert_eq!(pool.next_id(), 3);
    }

    // Do NOT make it `clone` (to test non-clone behavior)
    #[derive(Debug)]
    struct DummyType(i32);

    frb_generated_moi_arc_impl_value!(DummyType);

    crate::base_arc_generate_tests!(MoiArc::<DummyType>);
}
