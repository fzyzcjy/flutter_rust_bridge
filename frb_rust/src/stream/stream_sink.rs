use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::for_generated::DartAbi;
use crate::generalized_isolate::IntoDart;
use crate::generalized_isolate::{
    channel_to_handle, handle_to_channel, Channel, SendableChannelHandle,
};
use crate::platform_types::{deserialize_sendable_message_port_handle, handle_to_message_port};
use crate::rust2dart::sender::{Rust2DartSendError, Rust2DartSender};
use crate::stream::closer::StreamSinkCloser;
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
    pub fn deserialize(raw: String) -> Self {
        let sendable_channel_handle = channel_to_handle(&Channel::new(handle_to_message_port(
            &deserialize_sendable_message_port_handle(raw),
        )));
        Self {
            #[allow(clippy::clone_on_copy)]
            sendable_channel_handle: sendable_channel_handle.clone(),
            _closer: Arc::new(StreamSinkCloser::new(sendable_channel_handle)),
            _phantom_data: Default::default(),
        }
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add_raw(&self, value: Rust2DartCodec::Message) -> Result<(), Rust2DartSendError> {
        sender(&self.sendable_channel_handle).send(value.into_dart_abi())
    }
}

pub(super) fn sender(sendable_channel_handle: &SendableChannelHandle) -> Rust2DartSender {
    Rust2DartSender::new(handle_to_channel(sendable_channel_handle))
}

// frb-coverage:ignore-start
impl<T, Rust2DartCodec: BaseCodec> IntoDart for StreamSinkBase<T, Rust2DartCodec> {
    fn into_dart(self) -> DartAbi {
        unreachable!()
    }
}
// frb-coverage:ignore-end
