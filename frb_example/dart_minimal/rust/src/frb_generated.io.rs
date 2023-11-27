use super::*;
// Section: imports
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
pub extern "C" fn wire_minimal_adder(port_: i64, a: i32, b: i32) {
    wire_minimal_adder_impl(port_, a, b)
}
