use super::*;
// Section: file_attributes
// Section: code_header
// Section: imports
// Section: wire_funcs
// Section: wrapper_structs
// Section: static_checks
// Section: executor
// Section: allocate_funcs
// Section: related_funcs
// Section: impl_wire2api
// Section: wire2api_class
// Section: impl_new_with_nullptr
pub trait NewWithNullPtr {
        fn new_with_null_ptr() -> Self;
    }

    impl<T> NewWithNullPtr for *mut T {
        fn new_with_null_ptr() -> Self {
            std::ptr::null_mut()
        }
    }
    // Section: impl_into_dart


                #[no_mangle]
                pub extern "C" fn wire_simple_adder(port_: i64, a: i32, b: i32)  {
                    wire_simple_adder_impl(port_,a,b)
                }
            