use std::fmt::Debug;

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

    unsafe fn from_raw(raw: usize) -> Self
    where
        T: Sized;

    fn into_raw(self) -> usize;

    unsafe fn increment_strong_count(raw: usize);

    unsafe fn decrement_strong_count(raw: usize);
}
