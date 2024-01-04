use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{channel_to_handle, handle_to_channel, SendableChannelHandle};
use crate::rust2dart::sender::Rust2DartSender;
use std::marker::PhantomData;
use std::sync::Arc;

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSinkBase<T, Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    _closer: Arc<StreamSinkCloser<Rust2DartCodec>>,
    _phantom_data: (PhantomData<T>, PhantomData<Rust2DartCodec>),
}

impl<T, Rust2DartCodec: BaseCodec> StreamSinkBase<T, Rust2DartCodec> {
    /// Create a new sink from a port wrapper.
    pub(crate) fn new(
        sender: Rust2DartSender,
        closer: Arc<StreamSinkCloser<Rust2DartCodec>>,
    ) -> Self {
        Self {
            sendable_channel_handle: channel_to_handle(&sender.channel),
            _closer: closer,
            _phantom_data: Default::default(),
        }
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add(&self, value: Rust2DartCodec::Message) -> anyhow::Result<()> {
        sender(&self.sendable_channel_handle).send(value.into_dart_abi())
    }
}

// *NOT* cloneable, since it invokes stream-close when dropped
pub(crate) struct StreamSinkCloser<Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    _phantom_data: PhantomData<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> StreamSinkCloser<Rust2DartCodec> {
    pub fn new(sendable_channel_handle: SendableChannelHandle) -> Self {
        Self {
            sendable_channel_handle,
            _phantom_data: PhantomData,
        }
    }
}

impl<Rust2DartCodec: BaseCodec> Drop for StreamSinkCloser<Rust2DartCodec> {
    fn drop(&mut self) {
        sender(&self.sendable_channel_handle)
            .send_or_warn(Rust2DartCodec::encode_close_stream().into_dart_abi())
    }
}

fn sender(sendable_channel_handle: &SendableChannelHandle) -> Rust2DartSender {
    Rust2DartSender::new(handle_to_channel(sendable_channel_handle))
}
