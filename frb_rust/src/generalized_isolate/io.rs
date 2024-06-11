pub use allo_isolate::ZeroCopyBuffer;
pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};

pub(crate) type SerializedDartSendPort = i64;

pub(crate) struct DartSendPort(allo_isolate::Isolate);

impl DartSendPort {
    pub fn serialize(&self) -> String {
        port.to_owned()
    }

    pub fn dart_send_port_deserialize(port: &SerializedDartSendPort) -> DartSendPort {
        port.to_owned()
    }
}
