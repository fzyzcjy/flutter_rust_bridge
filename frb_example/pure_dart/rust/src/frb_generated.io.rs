use super::*;

// Section: impl_wire2api

impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f32> for *mut f32 {
    fn wire2api(self) -> f32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i16> for *mut i16 {
    fn wire2api(self) -> i16 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i8> for *mut i8 {
    fn wire2api(self) -> i8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<StructWithComments> for *mut wire_struct_with_comments {
    fn wire2api(self) -> StructWithComments {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithComments>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u16> for *mut u16 {
    fn wire2api(self) -> u16 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *support::box_from_leak_ptr(self) }
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
pub extern "C" fn wire_example_optional_primitive_type_bool(port_: i64, arg: *mut bool) {
    wire_example_optional_primitive_type_bool_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32(port_: i64, arg: *mut f32) {
    wire_example_optional_primitive_type_f32_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64(port_: i64, arg: *mut f64) {
    wire_example_optional_primitive_type_f64_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16(port_: i64, arg: *mut i16) {
    wire_example_optional_primitive_type_i16_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32(port_: i64, arg: *mut i32) {
    wire_example_optional_primitive_type_i32_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64(port_: i64, arg: *mut i64) {
    wire_example_optional_primitive_type_i64_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8(port_: i64, arg: *mut i8) {
    wire_example_optional_primitive_type_i8_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16(port_: i64, arg: *mut u16) {
    wire_example_optional_primitive_type_u16_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32(port_: i64, arg: *mut u32) {
    wire_example_optional_primitive_type_u32_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64(port_: i64, arg: *mut u64) {
    wire_example_optional_primitive_type_u64_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8(port_: i64, arg: *mut u8) {
    wire_example_optional_primitive_type_u8_impl(port_, arg)
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
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_32(value: f32) -> *mut f32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_16(value: i16) -> *mut i16 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_32(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_64(value: i64) -> *mut i64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_8(value: i8) -> *mut i8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments() -> *mut wire_struct_with_comments {
    support::new_leak_box_ptr(wire_struct_with_comments::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_16(value: u16) -> *mut u16 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_32(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_64(value: u64) -> *mut u64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_8(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}
