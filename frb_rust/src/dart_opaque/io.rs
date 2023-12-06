use super::dart_persistent_handle_auto_drop::DartPersistentHandleAutoDrop;
use crate::generalized_isolate::Channel;
use crate::platform_types::MessagePort;
use dart_sys::Dart_Handle;
use dart_sys::Dart_NewPersistentHandle_DL;
use dart_sys::{
    Dart_DeletePersistentHandle_DL, Dart_HandleFromPersistent_DL, Dart_PersistentHandle,
};
use std::ffi::c_void;

pub type GeneralizedDartPersistentHandleWrapper = DartPersistentHandleAutoDrop;
pub type GeneralizedDartPersistentHandle = Dart_PersistentHandle;

// TODO remove or rename this?
#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: DartPersistentHandleAutoDrop,
}

impl DartOpaqueBase {
    pub fn new(handle: Dart_PersistentHandle) -> Self {
        Self {
            inner: unsafe { DartPersistentHandleAutoDrop::from_raw(handle) },
        }
    }

    pub fn unwrap(self) -> DartPersistentHandleAutoDrop {
        self.inner
    }

    pub fn into_raw(self) -> Dart_PersistentHandle {
        self.inner.into_raw()
    }
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn get_dart_object(ptr: usize) -> Dart_Handle {
    let handle = DartPersistentHandleAutoDrop::from_raw(ptr as _);
    handle.create_dart_handle()
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn drop_dart_object(ptr: usize) {
    drop(DartPersistentHandleAutoDrop::from_raw(ptr as _))
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn dart_new_persistent_handle(handle: Dart_Handle) -> *const c_void {
    Dart_NewPersistentHandle_DL.expect("dart_api_dl has not been initialized")(handle) as _
}
