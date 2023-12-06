use super::dart_persistent_handle_auto_drop::DartPersistentHandleWrapper;
use crate::generalized_isolate::Channel;
use crate::platform_types::MessagePort;
use dart_sys::{Dart_DeletePersistentHandle_DL, Dart_PersistentHandle};

pub type GeneralizedDartPersistentHandleWrapper = DartPersistentHandleWrapper;
pub type GeneralizedDartPersistentHandle = Dart_PersistentHandle;

// TODO remove or rename this?
#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: DartPersistentHandleWrapper,
}

impl DartOpaqueBase {
    pub fn new(handle: Dart_PersistentHandle) -> Self {
        Self {
            inner: unsafe { DartPersistentHandleWrapper::from_raw(handle) },
        }
    }

    pub fn unwrap(self) -> DartPersistentHandleWrapper {
        self.inner
    }

    pub fn into_raw(self) -> Dart_PersistentHandle {
        self.inner.into_raw()
    }
}
