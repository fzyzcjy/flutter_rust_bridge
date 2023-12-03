use std::sync::Arc;

pub unsafe fn rust_arc_increment_strong_count<T>(ptr: *const std::ffi::c_void) {
    Arc::<T>::increment_strong_count(ptr as _)
}

pub unsafe fn rust_arc_decrement_strong_count<T>(ptr: *const std::ffi::c_void) {
    Arc::<T>::decrement_strong_count(ptr as _)
}
