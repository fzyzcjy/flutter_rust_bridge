use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
use crate::rust_opaque::RustOpaqueBase;

pub fn rust_auto_opaque_explicit_decode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    raw: RustOpaqueBase<RustAutoOpaqueInner<T>, A>,
) -> RustAutoOpaqueBase<T, A> {
    RustAutoOpaqueBase(raw)
}

#[cfg(test)]
mod tests {
    use super::rust_auto_opaque_explicit_decode;
    use crate::for_generated::rust_auto_opaque_explicit_encode;
    use crate::RustAutoOpaqueNom;

    /// Retains access to the same value after an explicit decode.
    #[test]
    fn decodes_explicit_opaque_value() {
        let opaque = RustAutoOpaqueNom::new(42);
        let decoded = rust_auto_opaque_explicit_decode(rust_auto_opaque_explicit_encode(opaque));

        assert_eq!(*decoded.blocking_read(), 42);
    }
}
