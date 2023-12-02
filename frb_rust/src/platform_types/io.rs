pub type WireSyncReturn = *mut allo_isolate::ffi::DartCObject;

pub type MessagePort = i64;

/// Use this type to represent Dart `dynamic` values
pub type DartAbi = allo_isolate::ffi::DartCObject;
