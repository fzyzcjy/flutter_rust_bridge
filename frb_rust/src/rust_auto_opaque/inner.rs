use crate::rust_auto_opaque::order::RustAutoOpaqueOrder;
use tokio::sync::RwLock;

pub struct RustAutoOpaqueInner<T> {
    pub(crate) data: RwLock<T>,
    pub(crate) order: RustAutoOpaqueOrder,
}

impl<T> RustAutoOpaqueInner<T> {
    pub(crate) fn new(data: RwLock<T>) -> Self {
        Self {
            data,
            order: RustAutoOpaqueOrder::new(),
        }
    }
}
