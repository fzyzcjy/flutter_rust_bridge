// Section: imports

use super::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::Handler;

// Section: impl_wire2api

impl Wire2Api<String> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
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

#[no_mangle]
pub extern "C" fn wire_greet(
    name: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_greet_impl(name)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_8(len: i32) -> *mut wire_list_prim_u_8 {
    let ans = wire_list_prim_u_8 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}
