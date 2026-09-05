use crate::for_generated::{BaseArc, RustAutoOpaqueBase};
use crate::rust_auto_opaque::inner;

impl<T: 'static + Default, A: BaseArc<inner::RustAutoOpaqueInner<T>>> Default
    for RustAutoOpaqueBase<T, A>
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: 'static, A: BaseArc<inner::RustAutoOpaqueInner<T>>> Clone for RustAutoOpaqueBase<T, A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::RustAutoOpaqueNom;

    /// Creates the default payload inside an opaque wrapper.
    #[test]
    fn defaults_the_wrapped_value() {
        let opaque = RustAutoOpaqueNom::<Vec<u8>>::default();

        assert!(opaque.blocking_read().is_empty());
    }

    /// Keeps the payload alive after the original wrapper is dropped.
    #[test]
    fn clone_keeps_the_payload_alive() {
        let clone = RustAutoOpaqueNom::new(42).clone();

        assert_eq!(*clone.blocking_read(), 42);
    }
}
