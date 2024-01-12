#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_opaque_dart2rust {
    () => {
        use $crate::for_generated::StdArc;
        use $crate::RustOpaqueNom;

        /// # Safety
        ///
        /// This should never be called manually.
        // NOTE: We require `T: Send+Sync`, because for example, the Arc may `into_raw` in one thread
        // and then `from_raw` in another one or multiple threads.
        //
        // Without this extra bound, users still will not misuse this, because the generated function call wrapper
        // already moves the `RustOpaque<T>` across thread boundary, and as is verified by experiments, compilers
        // correctly catch scenarios when T is not Send or Sync. However, we add this to be extra safe.
        pub unsafe fn decode_rust_opaque_nom<T: Send + Sync>(ptr: usize) -> RustOpaqueNom<T> {
            RustOpaqueNom::from_arc(StdArc::<T>::from_raw(ptr))
        }

        // This does not have `unsafe` keyword, thus is a separate function
        pub fn decode_rust_opaque_moi<T: MoiArcValue + Send + Sync>(
            ptr: usize,
        ) -> RustOpaqueMoi<T> {
            RustOpaqueMoi::from_arc(MoiArc::<T>::from_raw(ptr))
        }
    };
}
