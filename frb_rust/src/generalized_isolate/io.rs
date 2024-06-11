pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
pub type DartSendPort = allo_isolate::Isolate;
pub use allo_isolate::ZeroCopyBuffer;

pub type SerializedDartSendPort = DartSendPort;

pub fn dart_send_port_serialize(port: &DartSendPort) -> SerializedDartSendPort {
    port.to_owned()
}

pub fn dart_send_port_deserialize(port: &SerializedDartSendPort) -> DartSendPort {
    port.to_owned()
}
