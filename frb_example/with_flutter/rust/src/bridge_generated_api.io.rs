use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_draw_mandelbrot(
    port_: i64,
    image_size: *mut wire_Size,
    zoom_point: *mut wire_Point,
    scale: f64,
    num_threads: i32,
) {
    wire_draw_mandelbrot_impl(port_, image_size, zoom_point, scale, num_threads)
}

#[no_mangle]
pub extern "C" fn wire_passing_complex_structs(port_: i64, root: *mut wire_TreeNode) {
    wire_passing_complex_structs_impl(port_, root)
}

#[no_mangle]
pub extern "C" fn wire_returning_structs_with_boxed_fields(port_: i64) {
    wire_returning_structs_with_boxed_fields_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_input_array(port_: i64, input: *mut wire_uint_8_list) {
    wire_off_topic_memory_test_input_array_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_zero_copy_buffer(port_: i64, len: i32) {
    wire_off_topic_memory_test_output_zero_copy_buffer_impl(port_, len)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_vec_u8(port_: i64, len: i32) {
    wire_off_topic_memory_test_output_vec_u8_impl(port_, len)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_input_vec_of_object(
    port_: i64,
    input: *mut wire_list_size,
) {
    wire_off_topic_memory_test_input_vec_of_object_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_vec_of_object(port_: i64, len: i32) {
    wire_off_topic_memory_test_output_vec_of_object_impl(port_, len)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_input_complex_struct(
    port_: i64,
    input: *mut wire_TreeNode,
) {
    wire_off_topic_memory_test_input_complex_struct_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_memory_test_output_complex_struct(port_: i64, len: i32) {
    wire_off_topic_memory_test_output_complex_struct_impl(port_, len)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_deliberately_return_error(port_: i64) {
    wire_off_topic_deliberately_return_error_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_off_topic_deliberately_panic(port_: i64) {
    wire_off_topic_deliberately_panic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_next_user_id(port_: i64, user_id: *mut wire_UserId) {
    wire_next_user_id_impl(port_, user_id)
}

#[no_mangle]
pub extern "C" fn wire_get_app_settings(port_: i64) {
    wire_get_app_settings_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_fallible_app_settings(port_: i64) {
    wire_get_fallible_app_settings_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded(port_: i64, app_settings: *mut wire_ApplicationSettings) {
    wire_is_app_embedded_impl(port_, app_settings)
}

#[no_mangle]
pub extern "C" fn wire_test_method__method__BoxedPoint(port_: i64, that: *mut wire_BoxedPoint) {
    wire_test_method__method__BoxedPoint_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_sum__method__SumWith(port_: i64, that: *mut wire_SumWith, y: u32) {
    wire_sum__method__SumWith_impl(port_, that, y)
}

#[no_mangle]
pub extern "C" fn wire_sum_static__static_method__SumWith(port_: i64, x: u32, y: u32) {
    wire_sum_static__static_method__SumWith_impl(port_, x, y)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_application_env_0() -> *mut wire_ApplicationEnv {
    support::new_leak_box_ptr(wire_ApplicationEnv::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_env_0() -> *mut wire_ApplicationEnv {
    support::new_leak_box_ptr(wire_ApplicationEnv::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_settings_0() -> *mut wire_ApplicationSettings {
    support::new_leak_box_ptr(wire_ApplicationSettings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_boxed_point_0() -> *mut wire_BoxedPoint {
    support::new_leak_box_ptr(wire_BoxedPoint::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_point_0() -> *mut wire_Point {
    support::new_leak_box_ptr(wire_Point::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_size_0() -> *mut wire_Size {
    support::new_leak_box_ptr(wire_Size::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sum_with_0() -> *mut wire_SumWith {
    support::new_leak_box_ptr(wire_SumWith::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tree_node_0() -> *mut wire_TreeNode {
    support::new_leak_box_ptr(wire_TreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_id_0() -> *mut wire_UserId {
    support::new_leak_box_ptr(wire_UserId::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_point_0() -> *mut wire_Point {
    support::new_leak_box_ptr(wire_Point::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_application_env_var_0(len: i32) -> *mut wire_list_application_env_var {
    let wrap = wire_list_application_env_var {
        ptr: support::new_leak_vec_ptr(<wire_ApplicationEnvVar>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_size_0(len: i32) -> *mut wire_list_size {
    let wrap = wire_list_size {
        ptr: support::new_leak_vec_ptr(<wire_Size>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_tree_node_0(len: i32) -> *mut wire_list_tree_node {
    let wrap = wire_list_tree_node {
        ptr: support::new_leak_vec_ptr(<wire_TreeNode>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<ApplicationEnv> for wire_ApplicationEnv {
    fn wire2api(self) -> ApplicationEnv {
        ApplicationEnv {
            vars: self.vars.wire2api(),
        }
    }
}
impl Wire2Api<ApplicationEnvVar> for wire_ApplicationEnvVar {
    fn wire2api(self) -> ApplicationEnvVar {
        ApplicationEnvVar(self.field0.wire2api(), self.field1.wire2api())
    }
}

impl Wire2Api<ApplicationSettings> for wire_ApplicationSettings {
    fn wire2api(self) -> ApplicationSettings {
        ApplicationSettings {
            name: self.name.wire2api(),
            version: self.version.wire2api(),
            mode: self.mode.wire2api(),
            env: self.env.wire2api(),
            env_optional: self.env_optional.wire2api(),
        }
    }
}

impl Wire2Api<Box<ApplicationEnv>> for *mut wire_ApplicationEnv {
    fn wire2api(self) -> Box<ApplicationEnv> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationEnv>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApplicationEnv> for *mut wire_ApplicationEnv {
    fn wire2api(self) -> ApplicationEnv {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationEnv>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ApplicationSettings> for *mut wire_ApplicationSettings {
    fn wire2api(self) -> ApplicationSettings {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ApplicationSettings>::wire2api(*wrap).into()
    }
}
impl Wire2Api<BoxedPoint> for *mut wire_BoxedPoint {
    fn wire2api(self) -> BoxedPoint {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<BoxedPoint>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Point> for *mut wire_Point {
    fn wire2api(self) -> Point {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Point>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Size> for *mut wire_Size {
    fn wire2api(self) -> Size {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Size>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SumWith> for *mut wire_SumWith {
    fn wire2api(self) -> SumWith {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SumWith>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TreeNode> for *mut wire_TreeNode {
    fn wire2api(self) -> TreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TreeNode>::wire2api(*wrap).into()
    }
}
impl Wire2Api<UserId> for *mut wire_UserId {
    fn wire2api(self) -> UserId {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<UserId>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Point>> for *mut wire_Point {
    fn wire2api(self) -> Box<Point> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Point>::wire2api(*wrap).into()
    }
}
impl Wire2Api<BoxedPoint> for wire_BoxedPoint {
    fn wire2api(self) -> BoxedPoint {
        BoxedPoint {
            point: self.point.wire2api(),
        }
    }
}

impl Wire2Api<Vec<ApplicationEnvVar>> for *mut wire_list_application_env_var {
    fn wire2api(self) -> Vec<ApplicationEnvVar> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Size>> for *mut wire_list_size {
    fn wire2api(self) -> Vec<Size> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<TreeNode>> for *mut wire_list_tree_node {
    fn wire2api(self) -> Vec<TreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<Point> for wire_Point {
    fn wire2api(self) -> Point {
        Point {
            x: self.x.wire2api(),
            y: self.y.wire2api(),
        }
    }
}
impl Wire2Api<Size> for wire_Size {
    fn wire2api(self) -> Size {
        Size {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<SumWith> for wire_SumWith {
    fn wire2api(self) -> SumWith {
        SumWith {
            x: self.x.wire2api(),
        }
    }
}
impl Wire2Api<TreeNode> for wire_TreeNode {
    fn wire2api(self) -> TreeNode {
        TreeNode {
            name: self.name.wire2api(),
            children: self.children.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<UserId> for wire_UserId {
    fn wire2api(self) -> UserId {
        UserId {
            value: self.value.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationEnv {
    vars: *mut wire_list_application_env_var,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationEnvVar {
    field0: *mut wire_uint_8_list,
    field1: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ApplicationSettings {
    name: *mut wire_uint_8_list,
    version: *mut wire_uint_8_list,
    mode: i32,
    env: *mut wire_ApplicationEnv,
    env_optional: *mut wire_ApplicationEnv,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BoxedPoint {
    point: *mut wire_Point,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_application_env_var {
    ptr: *mut wire_ApplicationEnvVar,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_size {
    ptr: *mut wire_Size,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_tree_node {
    ptr: *mut wire_TreeNode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Point {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Size {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SumWith {
    x: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_TreeNode {
    name: *mut wire_uint_8_list,
    children: *mut wire_list_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_UserId {
    value: u32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_ApplicationEnv {
    fn new_with_null_ptr() -> Self {
        Self {
            vars: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ApplicationEnv {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ApplicationEnvVar {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}

impl Default for wire_ApplicationEnvVar {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ApplicationSettings {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            version: core::ptr::null_mut(),
            mode: Default::default(),
            env: core::ptr::null_mut(),
            env_optional: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_ApplicationSettings {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_BoxedPoint {
    fn new_with_null_ptr() -> Self {
        Self {
            point: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_BoxedPoint {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_Point {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Default for wire_Point {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_Size {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl Default for wire_Size {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SumWith {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
        }
    }
}

impl Default for wire_SumWith {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_TreeNode {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            children: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_TreeNode {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_UserId {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

impl Default for wire_UserId {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
