use super::*;
// Section: wire functions

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_cross_shared_struct_in_block_1_and_2(
) -> *mut wire_CrossSharedStructInBlock1And2 {
    support::new_leak_box_ptr(wire_CrossSharedStructInBlock1And2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_cross_shared_struct_in_block_2_and_3(
) -> *mut wire_CrossSharedStructInBlock2And3 {
    support::new_leak_box_ptr(wire_CrossSharedStructInBlock2And3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f64(value: f64) -> *mut f64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_in_all_blocks() -> *mut wire_SharedStructInAllBlocks
{
    support::new_leak_box_ptr(wire_SharedStructInAllBlocks::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_in_block_1_and_2(
) -> *mut wire_SharedStructInBlock1And2 {
    support::new_leak_box_ptr(wire_SharedStructInBlock1And2::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_in_block_2_and_3(
) -> *mut wire_SharedStructInBlock2And3 {
    support::new_leak_box_ptr(wire_SharedStructInBlock2And3::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shared_struct_only_for_sync_test(
) -> *mut wire_SharedStructOnlyForSyncTest {
    support::new_leak_box_ptr(wire_SharedStructOnlyForSyncTest::new_with_null_ptr())
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
impl Wire2Api<CrossSharedStructInBlock1And2> for *mut wire_CrossSharedStructInBlock1And2 {
    fn wire2api(self) -> CrossSharedStructInBlock1And2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CrossSharedStructInBlock1And2>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CrossSharedStructInBlock2And3> for *mut wire_CrossSharedStructInBlock2And3 {
    fn wire2api(self) -> CrossSharedStructInBlock2And3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CrossSharedStructInBlock2And3>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<SharedStructInAllBlocks> for *mut wire_SharedStructInAllBlocks {
    fn wire2api(self) -> SharedStructInAllBlocks {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructInAllBlocks>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStructInBlock1And2> for *mut wire_SharedStructInBlock1And2 {
    fn wire2api(self) -> SharedStructInBlock1And2 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructInBlock1And2>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStructInBlock2And3> for *mut wire_SharedStructInBlock2And3 {
    fn wire2api(self) -> SharedStructInBlock2And3 {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructInBlock2And3>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SharedStructOnlyForSyncTest> for *mut wire_SharedStructOnlyForSyncTest {
    fn wire2api(self) -> SharedStructOnlyForSyncTest {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SharedStructOnlyForSyncTest>::wire2api(*wrap).into()
    }
}
impl Wire2Api<CrossSharedStructInBlock1And2> for wire_CrossSharedStructInBlock1And2 {
    fn wire2api(self) -> CrossSharedStructInBlock1And2 {
        CrossSharedStructInBlock1And2 {
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<CrossSharedStructInBlock2And3> for wire_CrossSharedStructInBlock2And3 {
    fn wire2api(self) -> CrossSharedStructInBlock2And3 {
        CrossSharedStructInBlock2And3 {
            name: self.name.wire2api(),
        }
    }
}

impl Wire2Api<SharedStructInAllBlocks> for wire_SharedStructInAllBlocks {
    fn wire2api(self) -> SharedStructInAllBlocks {
        SharedStructInAllBlocks {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
            u8_list: self.u8_list.wire2api(),
        }
    }
}
impl Wire2Api<SharedStructInBlock1And2> for wire_SharedStructInBlock1And2 {
    fn wire2api(self) -> SharedStructInBlock1And2 {
        SharedStructInBlock1And2 {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<SharedStructInBlock2And3> for wire_SharedStructInBlock2And3 {
    fn wire2api(self) -> SharedStructInBlock2And3 {
        SharedStructInBlock2And3 {
            id: self.id.wire2api(),
            num: self.num.wire2api(),
            name: self.name.wire2api(),
        }
    }
}
impl Wire2Api<SharedStructOnlyForSyncTest> for wire_SharedStructOnlyForSyncTest {
    fn wire2api(self) -> SharedStructOnlyForSyncTest {
        SharedStructOnlyForSyncTest {
            name: self.name.wire2api(),
            score: self.score.wire2api(),
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
pub struct wire_CrossSharedStructInBlock1And2 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CrossSharedStructInBlock2And3 {
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructInAllBlocks {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
    u8_list: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructInBlock1And2 {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructInBlock2And3 {
    id: i32,
    num: f64,
    name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SharedStructOnlyForSyncTest {
    name: *mut wire_uint_8_list,
    score: f64,
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

impl NewWithNullPtr for wire_CrossSharedStructInBlock1And2 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_CrossSharedStructInBlock1And2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_CrossSharedStructInBlock2And3 {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_CrossSharedStructInBlock2And3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructInAllBlocks {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
            u8_list: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStructInAllBlocks {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructInBlock1And2 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStructInBlock1And2 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructInBlock2And3 {
    fn new_with_null_ptr() -> Self {
        Self {
            id: Default::default(),
            num: Default::default(),
            name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SharedStructInBlock2And3 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SharedStructOnlyForSyncTest {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            score: Default::default(),
        }
    }
}

impl Default for wire_SharedStructOnlyForSyncTest {
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
