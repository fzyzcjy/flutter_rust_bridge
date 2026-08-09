pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
pub type Channel = allo_isolate::Isolate;
pub use allo_isolate::ZeroCopyBuffer;

/// A channel that implements `Send`
pub type SendableChannelHandle = Channel;

pub fn deserialize_sendable_channel_handle(raw: String) -> SendableChannelHandle {
    allo_isolate::Isolate::new(raw.parse().unwrap())
}

pub fn handle_to_cached_channel(handle: &SendableChannelHandle) -> Channel {
    handle.to_owned()
}

pub fn release_cached_channel_handle(_handle: &SendableChannelHandle) {}
