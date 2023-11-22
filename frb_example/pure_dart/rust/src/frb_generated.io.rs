use super::*;

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
pub extern "C" fn wire_function_with_comments_slash_star_star(port_: i64) {
    wire_function_with_comments_slash_star_star_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line(port_: i64) {
    wire_function_with_comments_triple_slash_multi_line_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line(port_: i64) {
    wire_function_with_comments_triple_slash_single_line_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder(port_: i64, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}
