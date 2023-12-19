use super::auto_drop_dart_persistent_handle::AutoDropDartPersistentHandle;
use crate::dart_opaque::boxes::dart_isolate_box::DartIsolateBox;
use dart_sys_fork::Dart_Handle;

pub type GeneralizedAutoDropDartPersistentHandle = AutoDropDartPersistentHandle;
pub type GeneralizedDartHandleBox<T> = DartIsolateBox<T>;
pub type GeneralizedDartHandle = Dart_Handle;
