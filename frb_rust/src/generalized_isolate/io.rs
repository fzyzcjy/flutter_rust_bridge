pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
pub type DartSendPort = allo_isolate::Isolate;
pub use allo_isolate::ZeroCopyBuffer;

pub type SendableDartSendPort = DartSendPort;

pub fn dart_send_port_serialize(port: &DartSendPort) -> SendableDartSendPort {
    port.to_owned()
}

pub fn dart_send_port_deserialize(port: &SendableDartSendPort) -> DartSendPort {
    port.to_owned()
}
