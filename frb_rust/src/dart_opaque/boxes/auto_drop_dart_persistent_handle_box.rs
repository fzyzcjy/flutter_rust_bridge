use crate::dart_opaque::auto_drop_dart_persistent_handle::AutoDropDartPersistentHandle;
use crate::dart_opaque::boxes::dart_isolate_box::DartIsolateBox;

#[derive(Debug)]
pub(crate) struct AutoDropDartPersistentHandleBox(DartIsolateBox<AutoDropDartPersistentHandle>);
