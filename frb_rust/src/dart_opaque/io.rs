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
pub type GeneralizedDartHandle = Dart_Handle;
