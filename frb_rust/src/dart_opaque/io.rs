use super::auto_drop_dart_persistent_handle::AutoDropDartPersistentHandle;
use crate::dart_opaque::boxes::dart_isolate_box::DartIsolateBox;
use dart_sys::Dart_Handle;

pub type GeneralizedAutoDropDartPersistentHandle = AutoDropDartPersistentHandle;
pub type GeneralizedDartHandleBox = DartIsolateBox;
pub type GeneralizedDartHandle = Dart_Handle;
