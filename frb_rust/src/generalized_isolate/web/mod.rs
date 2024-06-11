mod into_dart;

pub use into_dart::*;

#[derive(Debug)]
pub struct ZeroCopyBuffer<T>(pub T);

impl<T> ZeroCopyBuffer<Vec<T>> {
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }
}

pub type DartSendPort = web_sys::BroadcastChannel;

pub type SerializedDartSendPort = String;

pub fn dart_send_port_serialize(port: &DartSendPort) -> SerializedDartSendPort {
    port.name()
}

pub fn dart_send_port_deserialize(port: &SerializedDartSendPort) -> DartSendPort {
    DartSendPort::new(port).unwrap()
}
