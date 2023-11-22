use super::*;

// Section: impl_wire2api

impl Wire2Api<StructWithCommentsTwinNormal> for *mut wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<StructWithCommentsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<StructWithCommentsTwinNormal> for wire_struct_with_comments_twin_normal {
    fn wire2api(self) -> StructWithCommentsTwinNormal {
        StructWithCommentsTwinNormal {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_normal {
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

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_instance_method(
    port_: i64,
    that: *mut wire_struct_with_comments_twin_normal,
) {
    wire_StructWithCommentsTwinNormal_instance_method_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_static_method(port_: i64) {
    wire_StructWithCommentsTwinNormal_static_method_impl(port_)
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
pub extern "C" fn wire_simple_adder_twin_normal(port_: i64, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_normal(
) -> *mut wire_struct_with_comments_twin_normal {
    support::new_leak_box_ptr(wire_struct_with_comments_twin_normal::new_with_null_ptr())
}
