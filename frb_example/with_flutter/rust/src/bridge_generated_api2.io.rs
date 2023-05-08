use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_get_app_settings_to_api2(port_: i64) {
    wire_get_app_settings_to_api2_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_fallible_app_settings_to_api2(port_: i64) {
    wire_get_fallible_app_settings_to_api2_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded_in_api2(
    port_: i64,
    app_settings: *mut wire_ApplicationSettings,
) {
    wire_is_app_embedded_in_api2_impl(port_, app_settings)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_application_env_1() -> *mut wire_ApplicationEnv {
    support::new_leak_box_ptr(wire_ApplicationEnv::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_env_1() -> *mut wire_ApplicationEnv {
    support::new_leak_box_ptr(wire_ApplicationEnv::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_settings_1() -> *mut wire_ApplicationSettings {
    support::new_leak_box_ptr(wire_ApplicationSettings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_application_env_var_1(len: i32) -> *mut wire_list_application_env_var {
    let wrap = wire_list_application_env_var {
        ptr: support::new_leak_vec_ptr(<wire_ApplicationEnvVar>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_1(len: i32) -> *mut wire_uint_8_list {
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

impl Wire2Api<Vec<ApplicationEnvVar>> for *mut wire_list_application_env_var {
    fn wire2api(self) -> Vec<ApplicationEnvVar> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
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
pub struct wire_list_application_env_var {
    ptr: *mut wire_ApplicationEnvVar,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
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
