use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_handle_opaque_aaa(port_: i64) {
    wire_handle_opaque_aaa_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_magic(port_: i64) {
    wire_magic_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_magic(port_: i64, magic: *mut wire_BoxRwLockWtffi) {
    wire_handle_magic_impl(port_, magic)
}

#[no_mangle]
pub extern "C" fn wire_handle_opaque_bbb(port_: i64, value: *mut wire_TestOpaque) {
    wire_handle_opaque_bbb_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_handle_opaque(port_: i64, value: *mut wire_OpaqueBag) {
    wire_handle_opaque_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_handle_opaque_repr(port_: i64, value: *mut wire_RwLockI32) {
    wire_handle_opaque_repr_impl(port_, value)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_BoxDartDebug() -> *mut wire_BoxDartDebug {
    support::new_leak_box_ptr(wire_BoxDartDebug::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_BoxRwLockWtffi() -> *mut wire_BoxRwLockWtffi {
    support::new_leak_box_ptr(wire_BoxRwLockWtffi::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_BoxWtffi() -> *mut wire_BoxWtffi {
    support::new_leak_box_ptr(wire_BoxWtffi::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RwLockI32() -> *mut wire_RwLockI32 {
    support::new_leak_box_ptr(wire_RwLockI32::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_RwLockIsize10() -> *mut wire_RwLockIsize10 {
    support::new_leak_box_ptr(wire_RwLockIsize10::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_Str() -> *mut wire_Str {
    support::new_leak_box_ptr(wire_Str::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opaque_bag_0() -> *mut wire_OpaqueBag {
    support::new_leak_box_ptr(wire_OpaqueBag::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_test_opaque_0() -> *mut wire_TestOpaque {
    support::new_leak_box_ptr(wire_TestOpaque::new_with_null_ptr())
}

// Section: impl Wire2Api

impl Wire2Api<Opaque<Box<dyn DartDebug>>> for *mut wire_BoxDartDebug {
    fn wire2api(self) -> Opaque<Box<dyn DartDebug>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<Box<RwLock<dyn wtffi>>>> for *mut wire_BoxRwLockWtffi {
    fn wire2api(self) -> Opaque<Box<RwLock<dyn wtffi>>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<Box<dyn wtffi>>> for *mut wire_BoxWtffi {
    fn wire2api(self) -> Opaque<Box<dyn wtffi>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RwLock<i32>>> for *mut wire_RwLockI32 {
    fn wire2api(self) -> Opaque<RwLock<i32>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<RwLock<[isize; 10]>>> for *mut wire_RwLockIsize10 {
    fn wire2api(self) -> Opaque<RwLock<[isize; 10]>> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<Opaque<&'static str>> for *mut wire_Str {
    fn wire2api(self) -> Opaque<&'static str> {
        unsafe {
            let ans = support::box_from_leak_ptr(self);
            support::opaque_from_dart(ans.ptr as _)
        }
    }
}
impl Wire2Api<OpaqueBag> for *mut wire_OpaqueBag {
    fn wire2api(self) -> OpaqueBag {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OpaqueBag>::wire2api(*wrap).into()
    }
}
impl Wire2Api<TestOpaque> for *mut wire_TestOpaque {
    fn wire2api(self) -> TestOpaque {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<TestOpaque>::wire2api(*wrap).into()
    }
}
impl Wire2Api<OpaqueBag> for wire_OpaqueBag {
    fn wire2api(self) -> OpaqueBag {
        OpaqueBag {
            primitive: self.primitive.wire2api(),
            array: self.array.wire2api(),
            lifetime: self.lifetime.wire2api(),
            trait_obj: self.trait_obj.wire2api(),
        }
    }
}

impl Wire2Api<TestOpaque> for wire_TestOpaque {
    fn wire2api(self) -> TestOpaque {
        TestOpaque {
            magic: self.magic.wire2api(),
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_BoxDartDebug {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BoxRwLockWtffi {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BoxWtffi {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockI32 {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RwLockIsize10 {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Str {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OpaqueBag {
    primitive: *mut wire_RwLockI32,
    array: *mut wire_RwLockIsize10,
    lifetime: *mut wire_Str,
    trait_obj: *mut wire_BoxDartDebug,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_TestOpaque {
    magic: *mut wire_BoxWtffi,
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

impl NewWithNullPtr for wire_BoxDartDebug {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_BoxRwLockWtffi {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_BoxWtffi {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockI32 {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RwLockIsize10 {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_Str {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_OpaqueBag {
    fn new_with_null_ptr() -> Self {
        Self {
            primitive: core::ptr::null_mut(),
            array: core::ptr::null_mut(),
            lifetime: core::ptr::null_mut(),
            trait_obj: core::ptr::null_mut(),
        }
    }
}

impl NewWithNullPtr for wire_TestOpaque {
    fn new_with_null_ptr() -> Self {
        Self {
            magic: core::ptr::null_mut(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}
