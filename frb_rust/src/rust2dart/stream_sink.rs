use crate::codec::{BaseCodec, Rust2DartMessageTrait};
use crate::for_generated::DcoCodec;
use crate::generalized_isolate::{channel_to_handle, handle_to_channel, SendableChannelHandle};
use crate::rust2dart::sender::Rust2DartSender;
use std::marker::PhantomData;
use std::sync::Arc;

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
///
/// When it is dropped, the Dart stream will be closed automatically.
///
/// If it is cloned, then the stream is closed when all instances are dropped.
#[derive(Clone)]
pub struct StreamSinkBase<T, Rust2DartCodec: BaseCodec = DcoCodec>(
    Arc<StreamSinkNonClone<T, Rust2DartCodec>>,
);

impl<T, Rust2DartCodec: BaseCodec> StreamSinkBase<T, Rust2DartCodec> {
    /// Create a new sink from a port wrapper.
    pub fn new(sender: Rust2DartSender) -> Self {
        Self(Arc::new(StreamSinkNonClone::new(sender)))
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add(&self, value: Rust2DartCodec::Message) -> anyhow::Result<()> {
        self.0.add(value)
    }
}

// non-Clone, since we want to close the stream when dropping it
struct StreamSinkNonClone<T, Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    _phantom_data: (PhantomData<T>, PhantomData<Rust2DartCodec>),
}

impl<T, Rust2DartCodec: BaseCodec> StreamSinkNonClone<T, Rust2DartCodec> {
    fn new(sender: Rust2DartSender) -> Self {
        Self {
            sendable_channel_handle: channel_to_handle(&sender.channel),
            _phantom_data: Default::default(),
        }
    }

    fn add(&self, value: Rust2DartCodec::Message) -> anyhow::Result<()> {
        self.sender().send(value.into_dart_abi())
    }

    fn sender(&self) -> Rust2DartSender {
        Rust2DartSender::new(handle_to_channel(&self.sendable_channel_handle))
    }
}

impl<T, Rust2DartCodec: BaseCodec> Drop for StreamSinkNonClone<T, Rust2DartCodec> {
    fn drop(&mut self) {
        self.sender()
            .send_or_warn(Rust2DartCodec::encode_close_stream().into_dart_abi())
    }
}
