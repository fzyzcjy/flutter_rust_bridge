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

pub type SendableDartSendPort = String;

#[derive(Debug, Clone)]
pub struct DartSendPort(web_sys::BroadcastChannel);

impl DartSendPort {
    pub fn to_sendable(&self) -> SendableDartSendPort {
        self.0.name()
    }

    pub fn from_sendable(port: SendableDartSendPort) -> DartSendPort {
        Self(web_sys::BroadcastChannel::new(port).unwrap())
    }
}
