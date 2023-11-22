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
impl Wire2Api<StructWithCommentsTwinNormal> for *mut wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for *mut wire_struct_with_comments_twin_sync {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithZeroField> for *mut wire_struct_with_zero_field {
    fn wire2api(self) -> StructWithZeroField {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithZeroField>::wire2api(*wrap).into()
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
impl Wire2Api<Vec<bool>> for *mut wire_list_bool {
    fn wire2api(self) -> Vec<bool> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<f32>> for *mut wire_list_prim_f_32 {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i16>> for *mut wire_list_prim_i_16 {
    fn wire2api(self) -> Vec<i16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i32>> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i64>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_list_prim_i_8 {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u16>> for *mut wire_list_prim_u_16 {
    fn wire2api(self) -> Vec<u16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u32>> for *mut wire_list_prim_u_32 {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u64>> for *mut wire_list_prim_u_64 {
    fn wire2api(self) -> Vec<u64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        StructWithCommentsTwinNormal {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<StructWithCommentsTwinSync> for wire_struct_with_comments_twin_sync {
    fn wire2api(self) -> StructWithCommentsTwinSync {
        StructWithCommentsTwinSync {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<StructWithZeroField> for wire_struct_with_zero_field {
    fn wire2api(self) -> StructWithZeroField {
        StructWithZeroField {}
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_bool {
    ptr: *mut bool,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_32 {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_64 {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_16 {
    ptr: *mut i16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_32 {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_64 {
    ptr: *mut i64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_8 {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_16 {
    ptr: *mut u16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_32 {
    ptr: *mut u32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_64 {
    ptr: *mut u64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_normal {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_sync {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_zero_field {}

// Section: impl_new_with_nullptr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_zero_field {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_struct_with_zero_field {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: i64,
    that: *mut wire_struct_with_comments_twin_normal,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_: i64) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_normal(port_: i64) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: *mut wire_struct_with_comments_twin_sync,
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_sync() -> support::WireSyncReturn
{
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_normal(
    port_: i64,
    arg: *mut bool,
) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_normal(port_: i64, arg: *mut f32) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_normal(port_: i64, arg: *mut f64) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_normal(port_: i64, arg: *mut i16) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_normal(port_: i64, arg: *mut i32) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_normal(port_: i64, arg: *mut i64) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_normal(port_: i64, arg: *mut i8) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_normal(port_: i64, arg: *mut u16) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_normal(port_: i64, arg: *mut u32) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_normal(port_: i64, arg: *mut u64) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_normal(port_: i64, arg: *mut u8) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: *mut bool,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: *mut f32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: *mut f64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: *mut i16,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: *mut i32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: *mut i64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: *mut i8,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: *mut u16,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: *mut u32,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: *mut u64,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: *mut u8,
) -> support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_normal(port_: i64, arg: bool) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_normal(port_: i64, arg: f32) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_normal(port_: i64, arg: f64) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_normal(port_: i64, arg: i16) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_normal(port_: i64, arg: i32) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_normal(port_: i64, arg: i64) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_normal(port_: i64, arg: i8) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_normal(port_: i64, arg: u16) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_normal(port_: i64, arg: u32) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_normal(port_: i64, arg: u64) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_normal(port_: i64, arg: u8) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_normal(
    port_: i64,
    arg: *mut wire_list_bool,
) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_32,
) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_64,
) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_16,
) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_32,
) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_64,
) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_8,
) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_16,
) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_32,
) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_64,
) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_8,
) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_sync(
    arg: *mut wire_list_bool,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_sync(
    arg: *mut wire_list_prim_f_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_sync(
    arg: *mut wire_list_prim_f_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_sync(
    arg: *mut wire_list_prim_i_16,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_sync(
    arg: *mut wire_list_prim_i_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_sync(
    arg: *mut wire_list_prim_i_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_sync(
    arg: *mut wire_list_prim_i_8,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_sync(
    arg: *mut wire_list_prim_u_16,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_sync(
    arg: *mut wire_list_prim_u_32,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_sync(
    arg: *mut wire_list_prim_u_64,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_sync(arg: bool) -> support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_sync(arg: f32) -> support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_sync(arg: f64) -> support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_sync(arg: i16) -> support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_sync(arg: i32) -> support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_sync(arg: i64) -> support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_sync(arg: i8) -> support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_sync(arg: u16) -> support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_sync(arg: u32) -> support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_sync(arg: u64) -> support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_sync(arg: u8) -> support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_sync(a: i32, b: i32) -> support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_normal(port_: i64, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_zero_field,
) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
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
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_normal(
) -> *mut wire_struct_with_comments_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_sync(
) -> *mut wire_struct_with_comments_twin_sync {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field() -> *mut wire_struct_with_zero_field {
    support::new_leak_box_ptr(wire_struct_with_zero_field::new_with_null_ptr())
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

#[no_mangle]
pub extern "C" fn new_list_bool(len: i32) -> *mut wire_list_bool {
    let wrap = wire_list_bool {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_32(len: i32) -> *mut wire_list_prim_f_32 {
    let ans = wire_list_prim_f_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_64(len: i32) -> *mut wire_list_prim_f_64 {
    let ans = wire_list_prim_f_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_16(len: i32) -> *mut wire_list_prim_i_16 {
    let ans = wire_list_prim_i_16 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_32(len: i32) -> *mut wire_list_prim_i_32 {
    let ans = wire_list_prim_i_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_64(len: i32) -> *mut wire_list_prim_i_64 {
    let ans = wire_list_prim_i_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_8(len: i32) -> *mut wire_list_prim_i_8 {
    let ans = wire_list_prim_i_8 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_16(len: i32) -> *mut wire_list_prim_u_16 {
    let ans = wire_list_prim_u_16 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_32(len: i32) -> *mut wire_list_prim_u_32 {
    let ans = wire_list_prim_u_32 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_64(len: i32) -> *mut wire_list_prim_u_64 {
    let ans = wire_list_prim_u_64 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_8(len: i32) -> *mut wire_list_prim_u_8 {
    let ans = wire_list_prim_u_8 {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}
