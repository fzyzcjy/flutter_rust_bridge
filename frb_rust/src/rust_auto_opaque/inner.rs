use crate::lockable::order::LockableOrder;
use std::fmt;
use std::fmt::Formatter;
use tokio::sync::RwLock;

pub struct RustAutoOpaqueInner<T> {
    pub(crate) data: RwLock<T>,
    pub(crate) order: LockableOrder,
}

impl<T> RustAutoOpaqueInner<T> {
    pub(crate) fn new(data: RwLock<T>) -> Self {
        Self {
            data,
            order: LockableOrder::new(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for RustAutoOpaqueInner<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.data.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::RustAutoOpaqueInner;
    use tokio::sync::RwLock;

    /// Formats the guarded payload for debugging.
    #[test]
    fn formats_inner_value() {
        let inner = RustAutoOpaqueInner::new(RwLock::new(42));

        assert_eq!(format!("{inner:?}"), "RwLock { data: 42 }");
    }
}
