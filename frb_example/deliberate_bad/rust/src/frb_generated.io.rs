// Section: imports

use super::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::Handler;

// Section: impl_new_with_nullptr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn wire_make_data_race(port_: i64) {
    wire_make_data_race_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_make_heap_use_after_free(port_: i64) {
    wire_make_heap_use_after_free_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_make_memory_leak(port_: i64) {
    wire_make_memory_leak_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_make_stack_buffer_overflow(port_: i64) {
    wire_make_stack_buffer_overflow_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_make_use_of_uninitialized_value(port_: i64) {
    wire_make_use_of_uninitialized_value_impl(port_)
}
