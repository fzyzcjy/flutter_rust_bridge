pub use allo_isolate::{IntoDart, IntoDartExceptPrimitive};
pub type Channel = allo_isolate::Isolate;
pub use allo_isolate::ZeroCopyBuffer;

/// A channel that implements `Send`
pub type SendableChannelHandle = Channel;

pub fn channel_to_handle(channel: &Channel) -> SendableChannelHandle {
    channel.to_owned()
}

pub fn handle_to_channel(handle: &SendableChannelHandle) -> Channel {
    handle.to_owned()
}
