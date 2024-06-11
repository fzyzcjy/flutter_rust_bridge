pub use allo_isolate::ZeroCopyBuffer;
pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};

pub type SendableDartSendPort = DartSendPort;

#[derive(Debug, Clone)]
pub struct DartSendPort(allo_isolate::Isolate);

impl DartSendPort {
    pub fn to_sendable(&self) -> SendableDartSendPort {
        self.to_owned()
    }

    pub fn from_sendable(port: &SendableDartSendPort) -> DartSendPort {
        port.to_owned()
    }
}
