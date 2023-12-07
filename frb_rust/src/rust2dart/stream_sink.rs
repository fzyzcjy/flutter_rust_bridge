use crate::codec::BaseCodec;
use crate::for_generated::DcoCodec;
use crate::generalized_isolate::{
    channel_to_handle, handle_to_channel, IntoDart, SendableChannelHandle,
};
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::sender::Rust2DartSender;
use std::marker::PhantomData;

trait StreamSink<T> {
    fn add<D: IntoDart>(&self, value: T) -> bool
    where
        T: IntoIntoDart<D>;

    fn close(&self) -> bool;
}

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSinkImpl<T> {
    sendable_channel_handle: SendableChannelHandle,
    _phantom_data: PhantomData<T>,
}

impl<T> StreamSinkImpl<T> {
    /// Create a new sink from a port wrapper.
    pub fn new(sender: Rust2DartSender) -> Self {
        Self {
            sendable_channel_handle: channel_to_handle(&sender.channel),
            _phantom_data: PhantomData,
        }
    }

    fn sender(&self) -> Rust2DartSender {
        Rust2DartSender::new(handle_to_channel(&self.sendable_channel_handle))
    }
}

impl<T> StreamSink<T> for StreamSinkImpl<T> {
    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add<D: IntoDart>(&self, value: T) -> bool
    where
        T: IntoIntoDart<D>,
    {
        // TODO do not hardcode!
        self.sender().send(DcoCodec::encode(
            value.into_into_dart(),
            Rust2DartAction::Success,
        ))
    }

    /// Close the stream and ignore further messages. Returns false when
    /// the stream could not be closed, or when it has already been closed.
    pub fn close(&self) -> bool {
        // TODO do not hardcode!
        self.sender()
            .send(DcoCodec::encode((), Rust2DartAction::CloseStream))
    }
}
