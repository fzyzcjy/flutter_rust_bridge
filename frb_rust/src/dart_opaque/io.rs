use dart_sys::Dart_PersistentHandle;
use crate::platform_types::MessagePort;

#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: DartHandleWrap,
    drop_port: Option<MessagePort>,
}

impl DartOpaqueBase {
    pub fn new(handle: Dart_PersistentHandle, drop_port: Option<MessagePort>) -> Self {
        Self {
            inner: DartHandleWrap::from_raw(handle),
            drop_port,
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
