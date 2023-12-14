use dart_sys::*;

#[no_mangle]
pub unsafe extern "C" fn InitializeApiDL(data: *mut std::ffi::c_void) -> isize {
    Dart_InitializeApiDL(data)
}

#[no_mangle]
pub unsafe extern "C" fn NewPersistentHandle(
    non_persistent_handle: Dart_Handle,
) -> *mut std::ffi::c_void {
    Dart_NewPersistentHandle_DL.unwrap()(non_persistent_handle) as _
}

#[no_mangle]
pub unsafe extern "C" fn HandleFromPersistent(
    persistent_handle: *mut std::ffi::c_void,
) -> Dart_Handle {
    Dart_HandleFromPersistent_DL.unwrap()(persistent_handle as _)
}
