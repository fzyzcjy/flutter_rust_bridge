use super::RustOpaqueBase;
use crate::generalized_arc::base_arc::BaseArc;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use std::mem;

impl<T, A: BaseArc<T>> RustOpaqueBase<T, A> {
    pub fn sse_encode_raw(self) -> (usize, i32) {
        let (ptr, size) = self.encode();
        (ptr as _, size as _)
    }

    fn encode(self) -> (usize, usize) {
        let ptr = A::into_raw(self.arc);
        let size = mem::size_of::<T>();
        (ptr, size)
    }
}

impl<T, A: BaseArc<T>> From<RustOpaqueBase<T, A>> for DartAbi {
    fn from(value: RustOpaqueBase<T, A>) -> Self {
        let (ptr, size) = value.encode();
        [ptr.into_dart(), size.into_dart()].into_dart()
    }
}

#[cfg(test)]
mod tests {
    use crate::for_generated::decode_rust_opaque_nom;
    use crate::RustOpaqueNom;

    /// Transfers a non-null raw pointer with the original value size.
    #[test]
    fn encodes_sse_raw_pointer_and_size() {
        let (raw, size) = RustOpaqueNom::new(42_u64).sse_encode_raw();
        let decoded = unsafe { decode_rust_opaque_nom::<u64>(raw) };

        assert_ne!(raw, 0);
        assert_eq!(size, size_of::<u64>() as i32);
        assert_eq!(*decoded, 42);
    }
}
