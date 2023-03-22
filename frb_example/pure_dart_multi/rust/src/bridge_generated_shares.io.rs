use super::*;
// Section: wire functions

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_cross_shared_struct() -> *mut wire_CrossSharedStruct {
    support::new_leak_box_ptr(wire_CrossSharedStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct() -> *mut wire_SharedStruct {
    support::new_leak_box_ptr(wire_SharedStruct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
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
impl Wire2Api<CrossSharedStruct> for *mut wire_CrossSharedStruct {
    fn wire2api(self) -> CrossSharedStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CrossSharedStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStruct> for *mut wire_SharedStruct {
    fn wire2api(self) -> SharedStruct {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CrossSharedStruct> for wire_CrossSharedStruct {
    fn wire2api(self) -> CrossSharedStruct {
        CrossSharedStruct {
            name: self.name.wire2api(),
        }
    }
}

impl Wire2Api<SharedStruct> for wire_SharedStruct {
    fn wire2api(self) -> SharedStruct {
        SharedStruct {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
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
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_CrossSharedStruct {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStruct {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
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

impl NewWithNullPtr for wire_CrossSharedStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_CrossSharedStruct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStruct {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStruct {
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
