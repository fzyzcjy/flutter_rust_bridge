use super::auto_drop_dart_persistent_handle::AutoDropDartPersistentHandle;
use crate::generalized_isolate::Channel;
use crate::platform_types::MessagePort;
use dart_sys::Dart_Handle;
use dart_sys::Dart_NewPersistentHandle_DL;
use dart_sys::{
    Dart_DeletePersistentHandle_DL, Dart_HandleFromPersistent_DL, Dart_PersistentHandle,
};
use std::ffi::c_void;

pub type GeneralizedAutoDropDartPersistentHandle = AutoDropDartPersistentHandle;

// TODO things below not migrated yet --------------------------------------------------------

// TODO rm?
// pub type GeneralizedDartPersistentHandle = Dart_PersistentHandle;
//
// // TODO remove or rename this?
// #[derive(Debug)]
// pub struct DartOpaqueBase {
//     inner: AutoDropDartPersistentHandle,
// }
//
// impl DartOpaqueBase {
//     pub fn new(handle: Dart_PersistentHandle) -> Self {
//         Self {
//             inner: unsafe { AutoDropDartPersistentHandle::from_raw(handle) },
//         }
//     }
//
//     pub fn unwrap(self) -> AutoDropDartPersistentHandle {
//         self.inner
//     }
//
//     pub fn into_raw(self) -> Dart_PersistentHandle {
//         self.inner.into_raw()
//     }
// }
//
// /// # Safety
// ///
// /// This function should never be called manually.
// #[no_mangle]
// pub unsafe extern "C" fn get_dart_object(ptr: usize) -> Dart_Handle {
//     let handle = AutoDropDartPersistentHandle::from_raw(ptr as _);
//     handle.create_dart_handle()
// }
//
// TODO rm
// /// # Safety
// ///
// /// This function should never be called manually.
// #[no_mangle]
// pub unsafe extern "C" fn drop_dart_object(ptr: usize) {
//     drop(AutoDropDartPersistentHandle::from_raw(ptr as _))
// }
//
// TODO rm
// /// # Safety
// ///
// /// This function should never be called manually.
// #[no_mangle]
// pub unsafe extern "C" fn dart_new_persistent_handle(handle: Dart_Handle) -> *const c_void {
//     Dart_NewPersistentHandle_DL.expect("dart_api_dl has not been initialized")(handle) as _
// }
