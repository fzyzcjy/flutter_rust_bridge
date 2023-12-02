pub use allo_isolate::IntoDart;
pub type Channel = allo_isolate::Isolate;
pub use allo_isolate::ZeroCopyBuffer;

/// A channel that implements `Send`
pub type SendableChannelHandle = Channel;
