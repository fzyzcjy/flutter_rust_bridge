use crate::rust_opaque::codec::BaseRustOpaqueCodec;

/// # Safety
///
/// This should never be called manually.
pub unsafe fn rust_arc_increment_strong_count<T, C: BaseRustOpaqueCodec>(
    ptr: *const std::ffi::c_void,
) {
    C::Arc::<T>::increment_strong_count(ptr as _)
}

/// # Safety
///
/// This should never be called manually.
pub unsafe fn rust_arc_decrement_strong_count<T, C: BaseRustOpaqueCodec>(
    ptr: *const std::ffi::c_void,
) {
    C::Arc::<T>::decrement_strong_count(ptr as _)
}
