use super::{DartSafe, RustOpaque};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use std::mem;
use std::sync::Arc;

impl<T: DartSafe> RustOpaque<T> {
    pub fn sse_encode_raw(self) -> (usize, i32) {
        let (ptr, size) = value.encode();
        (ptr as _, size as _)
    }

    fn encode(self) -> (*const T, usize) {
        let ptr = Arc::into_raw(value.arc);
        let size = mem::size_of::<T>();
        (ptr, size)
    }
}

impl<T: DartSafe> From<RustOpaque<T>> for DartAbi {
    fn from(value: RustOpaque<T>) -> Self {
        let (ptr, size) = value.encode();
        vec![ptr.into_dart(), size.into_dart()].into_dart()
    }
}
