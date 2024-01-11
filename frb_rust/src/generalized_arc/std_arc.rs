use crate::generalized_arc::base_arc::BaseArc;

pub(crate) struct StdArc<T: ?Sized>(std::sync::Arc<T>);

impl<T: ?Sized> BaseArc<T> for StdArc<T> {}
