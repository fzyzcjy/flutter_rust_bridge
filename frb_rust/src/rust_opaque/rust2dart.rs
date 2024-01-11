use super::RustOpaque;
use crate::generalized_arc::base_arc::BaseArc;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust_opaque::codec::BaseRustOpaqueCodec;
use std::mem;

impl<T, C: BaseRustOpaqueCodec> RustOpaque<T, C> {
    pub fn sse_encode_raw(self) -> (usize, i32) {
        let (ptr, size) = self.encode();
        (ptr as _, size as _)
    }

    fn encode(self) -> (usize, usize) {
        let ptr = C::Arc::into_raw(self.arc);
        let size = mem::size_of::<T>();
        (ptr, size)
    }
}

impl<T, C: BaseRustOpaqueCodec> From<RustOpaque<T, C>> for DartAbi {
    fn from(value: RustOpaque<T, C>) -> Self {
        let (ptr, size) = value.encode();
        [ptr.into_dart(), size.into_dart()].into_dart()
    }
}
