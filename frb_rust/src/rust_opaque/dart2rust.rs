#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_opaque_dart2rust {
    () => {
        use $crate::for_generated::StdArc;
        use $crate::RustOpaque;

        /// # Safety
        ///
        /// This should never be called manually.
        pub unsafe fn decode_rust_opaque_nom<T>(ptr: usize) -> RustOpaque<T, StdArc<_>> {
            assert!(ptr != 0);
            RustOpaque::from_arc(StdArc::<T>::from_raw(ptr))
        }

        // This does not have `unsafe` keyword, thus is a separate function
        pub fn decode_rust_opaque_moi<T: MoiArcValue>(ptr: usize) -> RustOpaque<T, MoiArc<_>> {
            RustOpaque::from_arc(MoiArc::<T>::from_raw(ptr))
        }
    };
}
