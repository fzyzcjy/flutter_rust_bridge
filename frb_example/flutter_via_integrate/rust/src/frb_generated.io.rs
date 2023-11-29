// Section: imports

use super::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::Handler;

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
pub extern "C" fn wire_add(left: i32, right: i32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_add_impl(left, right)
}
