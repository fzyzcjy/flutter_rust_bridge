use crate::generalized_isolate::{channel_to_handle, handle_to_channel, SendableChannelHandle};
use crate::rust2dart::sender::Rust2DartSender;
use std::marker::PhantomData;

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSinkBase<T, Rust2DartCodec: BaseCodec = DcoCodec> {
    sendable_channel_handle: SendableChannelHandle,
    _phantom_data: (PhantomData<T>, PhantomData<Rust2DartCodec>),
}

impl<T, Rust2DartCodec: BaseCodec> StreamSinkBase<T, Rust2DartCodec> {
    /// Create a new sink from a port wrapper.
    pub fn new(sender: Rust2DartSender) -> Self {
        Self {
            sendable_channel_handle: channel_to_handle(&sender.channel),
            _phantom_data: Default::default(),
        }
    }

    fn sender(&self) -> Rust2DartSender {
        Rust2DartSender::new(handle_to_channel(&self.sendable_channel_handle))
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add(&self, value: Rust2DartCodec::Message) -> anyhow::Result<()> {
        self.sender().send(value.into_dart_abi())
    }

    /// Close the stream and ignore further messages. Returns false when
    /// the stream could not be closed, or when it has already been closed.
    pub fn close(&self) -> anyhow::Result<()> {
        self.sender()
            .send(Rust2DartCodec::encode_close_stream().into_dart_abi())
    }
}
