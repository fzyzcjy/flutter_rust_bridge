use crate::for_generated::StdArc;
use crate::rust_opaque::RustOpaqueNom;

/// # Safety
///
/// This should never be called manually.
// NOTE: We require `T: Send+Sync`, because for example, the Arc may `into_raw` in one thread
// and then `from_raw` in another one or multiple threads. Though this is often enforced in other places automatically.
pub unsafe fn decode_rust_opaque_nom<T: Send + Sync>(ptr: usize) -> RustOpaqueNom<T> {
    RustOpaqueNom::from_arc(StdArc::<T>::from_raw(ptr))
}

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_opaque_dart2rust {
    () => {
        use $crate::for_generated::decode_rust_opaque_nom;

        // This does not have `unsafe` keyword, thus is a separate function
        fn decode_rust_opaque_moi<T: MoiArcValue + Send + Sync>(ptr: usize) -> RustOpaqueMoi<T> {
            RustOpaqueMoi::from_arc(MoiArc::<T>::from_raw(ptr))
        }
    };
}
