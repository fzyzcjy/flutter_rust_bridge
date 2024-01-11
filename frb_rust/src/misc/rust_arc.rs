use std::sync::Arc;

/// # Safety
///
/// This should never be called manually.
pub unsafe fn rust_arc_increment_strong_count<T>(ptr: *const std::ffi::c_void) {
    Arc::<T>::increment_strong_count(ptr as _)
}

/// # Safety
///
/// This should never be called manually.
pub unsafe fn rust_arc_decrement_strong_count<T>(ptr: *const std::ffi::c_void) {
    Arc::<T>::decrement_strong_count(ptr as _)
}
