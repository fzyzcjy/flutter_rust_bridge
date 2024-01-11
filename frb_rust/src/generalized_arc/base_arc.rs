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
