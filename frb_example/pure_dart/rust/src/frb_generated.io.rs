use super::*;

// Section: impl_wire2api

impl Wire2Api<StructWithComments> for *mut wire_struct_with_comments {
    fn wire2api(self) -> StructWithComments {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithComments>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithComments> for wire_struct_with_comments {
    fn wire2api(self) -> StructWithComments {
        StructWithComments {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments {
    field_with_comments: i32,
}

// Section: impl_new_with_nullptr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
impl NewWithNullPtr for wire_struct_with_comments {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn wire_StructWithComments_instance_method(
    port_: i64,
    that: *mut wire_struct_with_comments,
) {
    wire_StructWithComments_instance_method_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithComments_static_method(port_: i64) {
    wire_StructWithComments_static_method_impl(port_)
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
pub extern "C" fn wire_example_primitive_type_bool(port_: i64, arg: bool) {
    wire_example_primitive_type_bool_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32(port_: i64, arg: f32) {
    wire_example_primitive_type_f32_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64(port_: i64, arg: f64) {
    wire_example_primitive_type_f64_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16(port_: i64, arg: i16) {
    wire_example_primitive_type_i16_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32(port_: i64, arg: i32) {
    wire_example_primitive_type_i32_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64(port_: i64, arg: i64) {
    wire_example_primitive_type_i64_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8(port_: i64, arg: i8) {
    wire_example_primitive_type_i8_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16(port_: i64, arg: u16) {
    wire_example_primitive_type_u16_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32(port_: i64, arg: u32) {
    wire_example_primitive_type_u32_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64(port_: i64, arg: u64) {
    wire_example_primitive_type_u64_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8(port_: i64, arg: u8) {
    wire_example_primitive_type_u8_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder(port_: i64, a: i32, b: i32) {
    wire_simple_adder_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments() -> *mut wire_struct_with_comments {
    support::new_leak_box_ptr(wire_struct_with_comments::new_with_null_ptr())
}
