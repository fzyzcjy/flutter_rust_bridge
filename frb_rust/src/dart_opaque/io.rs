use crate::generalized_isolate::Channel;
use crate::platform_types::MessagePort;
use dart_sys::{Dart_DeletePersistentHandle_DL, Dart_PersistentHandle};

pub type OpaqueMessagePort = i64;
pub type DartWrapObject = DartHandleWrap;
pub type DartObject = Dart_PersistentHandle;

#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: DartHandleWrap,
}

impl DartOpaqueBase {
    pub fn new(handle: Dart_PersistentHandle) -> Self {
        Self {
            inner: DartHandleWrap::from_raw(handle),
        }
    }

    pub fn into_raw(self) -> Dart_PersistentHandle {
        self.inner.into_raw()
    }

    pub fn unwrap(self) -> DartHandleWrap {
        self.inner
    }

    pub fn channel(&self) -> Option<Channel> {
        Some(Channel::new(self.drop_port?))
    }
}

#[derive(Debug)]
/// Option for correct drop.
pub struct DartHandleWrap(Option<Dart_PersistentHandle>);

impl DartHandleWrap {
    pub fn from_raw(ptr: Dart_PersistentHandle) -> Self {
        Self(Some(ptr))
    }

    pub fn into_raw(mut self) -> Dart_PersistentHandle {
        self.0.take().unwrap()
    }
}

impl From<DartHandleWrap> for Dart_PersistentHandle {
    fn from(warp: DartHandleWrap) -> Self {
        warp.into_raw()
    }
}

impl Drop for DartHandleWrap {
    fn drop(&mut self) {
        if let Some(inner) = self.0 {
            unsafe {
                Dart_DeletePersistentHandle_DL.expect("dart_api_dl has not been initialized")(inner)
            }
        }
    }
}
