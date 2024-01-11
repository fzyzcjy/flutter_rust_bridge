pub(crate) trait BaseArc<T> {
    fn new(value: T) -> Self;

    fn try_unwrap(self) -> Result<T, Self>
    where
        Self: Sized;

    fn into_inner(self) -> Option<T>;
}
