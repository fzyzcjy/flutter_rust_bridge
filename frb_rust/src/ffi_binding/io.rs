use crate::for_generated::box_from_leak_ptr;
use crate::platform_types::WireSyncReturn;
pub use allo_isolate::*;
use dart_sys::Dart_DeletePersistentHandle_DL;
use dart_sys::Dart_Handle;
use dart_sys::Dart_HandleFromPersistent_DL;
use dart_sys::Dart_InitializeApiDL;
use dart_sys::Dart_NewPersistentHandle_DL;
use dart_sys::Dart_PersistentHandle;
use libc::c_void;

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn init_frb_dart_api_dl(data: *mut c_void) -> isize {
    Dart_InitializeApiDL(data)
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn free_wire_sync_return(ptr: WireSyncReturn) {
    let _ = box_from_leak_ptr(ptr);
}

#[no_mangle]
pub unsafe extern "C" fn naive_NewPersistentHandle(non_persistent_handle: Dart_Handle) -> usize {
    let persistent_handle = Dart_NewPersistentHandle_DL.unwrap()(non_persistent_handle);
    println!("hi naive_NewPersistentHandle non_persistent_handle={non_persistent_handle:?} persistent_handle={persistent_handle:?}");
    persistent_handle as _
}

#[no_mangle]
pub unsafe extern "C" fn naive_HandleFromPersistent(persistent_handle: usize) -> usize {
    let ans = Dart_HandleFromPersistent_DL.unwrap()(persistent_handle as _);
    println!("hi naive_HandleFromPersistent persistent_handle={persistent_handle:?} ans={ans:?}");
    ans as _
}
