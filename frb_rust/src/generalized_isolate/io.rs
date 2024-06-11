pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
pub type DartSendPort = allo_isolate::Isolate;
pub use allo_isolate::ZeroCopyBuffer;

/// A channel that implements `Send`
pub type SerializedDartSendPort = DartSendPort;

pub fn channel_to_handle(channel: &DartSendPort) -> SerializedDartSendPort {
    channel.to_owned()
}

pub fn handle_to_channel(handle: &SerializedDartSendPort) -> DartSendPort {
    handle.to_owned()
}
