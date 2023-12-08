use crate::for_generated::{box_from_leak_ptr, new_leak_vec_ptr, vec_from_leak_ptr};
use crate::platform_types::{WireSyncReturnDco, WireSyncReturnSse};
pub use allo_isolate::*;
use dart_sys::Dart_DeletePersistentHandle_DL;
use dart_sys::Dart_Handle;
use dart_sys::Dart_HandleFromPersistent_DL;
use dart_sys::Dart_InitializeApiDL;
use dart_sys::Dart_NewPersistentHandle_DL;
use dart_sys::Dart_PersistentHandle;
use std::ffi::c_uchar;

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn init_frb_dart_api_dl(data: *mut std::ffi::c_void) -> isize {
    Dart_InitializeApiDL(data)
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn free_wire_sync_return_dco(ptr: WireSyncReturnDco) {
    let _ = box_from_leak_ptr(ptr);
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn free_wire_sync_return_sse(ptr: WireSyncReturnSse) {
    todo!()
}
