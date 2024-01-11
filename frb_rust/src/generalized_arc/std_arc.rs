use crate::generalized_arc::base_arc::BaseArc;
use std::sync::Arc;

pub(crate) struct StdArc<T: ?Sized>(Arc<T>);

impl<T> BaseArc<T> for StdArc<T> {
    fn new(value: T) -> Self {
        Self(Arc::new(value))
    }

    fn try_unwrap(self) -> Result<T, Self> {
        Arc::try_unwrap(self.0)
    }

    fn into_inner(self) -> Option<T> {
        Arc::into_inner(self.0)
    }
}
