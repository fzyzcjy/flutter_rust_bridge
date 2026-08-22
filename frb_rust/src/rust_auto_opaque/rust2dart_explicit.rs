use crate::for_generated::BaseArc;
use crate::rust_auto_opaque::{inner::RustAutoOpaqueInner, RustAutoOpaqueBase};
use crate::rust_opaque::RustOpaqueBase;

pub fn rust_auto_opaque_explicit_encode<T, A: BaseArc<RustAutoOpaqueInner<T>>>(
    raw: RustAutoOpaqueBase<T, A>,
) -> RustOpaqueBase<RustAutoOpaqueInner<T>, A> {
    raw.0
}

#[cfg(test)]
mod tests {
    use super::rust_auto_opaque_explicit_encode;
    use crate::for_generated::rust_auto_opaque_explicit_decode;
    use crate::RustAutoOpaqueNom;

    /// Preserves the opaque value through an explicit encode and decode.
    #[test]
    fn encodes_explicit_opaque_value() {
        let opaque = RustAutoOpaqueNom::new(42);
        let decoded = rust_auto_opaque_explicit_decode(rust_auto_opaque_explicit_encode(opaque));

        assert_eq!(*decoded.blocking_read(), 42);
    }
}
