#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_opaque_dart2rust {
    () => {
        use $crate::for_generated::{BaseRustOpaqueCodec, NomRustOpaqueCodec};
        use $crate::RustOpaque;

        /// # Safety
        ///
        /// This should never be called manually.
        pub unsafe fn decode_rust_opaque_nom<T>(ptr: usize) -> RustOpaque<T, NomRustOpaqueCodec> {
            assert!(ptr != 0);
            RustOpaque::from_arc(<NomRustOpaqueCodec as BaseRustOpaqueCodec<T>>::Arc::from_raw(ptr))
        }

        // This does not have `unsafe` keyword, thus is a separate function
        pub fn decode_rust_opaque_moi<T: MapBasedArcValue>(
            ptr: usize,
        ) -> RustOpaque<T, MoiRustOpaqueCodec> {
            RustOpaque::from_arc(<MoiRustOpaqueCodec as BaseRustOpaqueCodec<T>>::Arc::from_raw(ptr))
        }
    };
}
