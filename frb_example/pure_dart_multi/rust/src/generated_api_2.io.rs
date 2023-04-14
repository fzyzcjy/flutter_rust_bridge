use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_simple_adder_2(port_: i64, a: i32, b: i32) {
    wire_simple_adder_2_impl(port_, a, b)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: wire structs

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
