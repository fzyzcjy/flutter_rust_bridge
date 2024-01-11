use crate::generalized_arc::base_arc::BaseArc;
use std::sync::Arc;

#[derive(Debug)]
pub struct MapBasedArc<T: ?Sized>(u64);

impl<T: ?Sized> AsRef<T> for MapBasedArc<T> {
    fn as_ref(&self) -> &T {
        todo!()
    }
}

impl<T: ?Sized> BaseArc<T> for MapBasedArc<T> {
    fn new(value: T) -> Self
    where
        T: Sized,
    {
        todo!()
    }

    fn try_unwrap(self) -> Result<T, Self>
    where
        T: Sized,
    {
        todo!()
    }

    fn into_inner(self) -> Option<T>
    where
        T: Sized,
    {
        todo!()
    }

    unsafe fn from_raw(raw: usize) -> Self
    where
        T: Sized,
    {
        todo!()
    }

    fn into_raw(self) -> usize {
        todo!()
    }
}

impl<T: ?Sized> Clone for MapBasedArc<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}
