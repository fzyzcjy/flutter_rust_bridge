use crate::platform_types::DartNativeSendPort;
pub use allo_isolate::ZeroCopyBuffer;
pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};

#[derive(Debug, Clone)]
pub struct DartSendPort(allo_isolate::Isolate);

impl DartSendPort {
    pub fn new(native: DartNativeSendPort) -> Self {
        Self(allo_isolate::Isolate::new(native))
    }

    pub fn post(&self, msg: impl IntoDart) -> bool {
        self.0.post(msg)
    }
}
