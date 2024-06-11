use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::for_generated::DartAbi;
use crate::generalized_isolate::IntoDart;
use crate::generalized_isolate::{
    dart_send_port_serialize, dart_send_port_deserialize, DartSendPort, SerializedDartSendPort,
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
    serialized_dart_send_port: SerializedDartSendPort,
    _closer: Arc<StreamSinkCloser<Rust2DartCodec>>,
    _phantom_data: (PhantomData<T>, PhantomData<Rust2DartCodec>),
}

impl<T, Rust2DartCodec: BaseCodec> StreamSinkBase<T, Rust2DartCodec> {
    pub fn deserialize(raw: String) -> Self {
        let serialized_dart_send_port = dart_send_port_serialize(&DartSendPort::new(handle_to_message_port(
            &deserialize_sendable_message_port_handle(raw),
        )));
        Self {
            #[allow(clippy::clone_on_copy)]
            serialized_dart_send_port: serialized_dart_send_port.clone(),
            _closer: Arc::new(StreamSinkCloser::new(serialized_dart_send_port)),
            _phantom_data: Default::default(),
        }
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add_raw(&self, value: Rust2DartCodec::Message) -> Result<(), Rust2DartSendError> {
        sender(&self.serialized_dart_send_port).send(value.into_dart_abi())
    }
}

pub(super) fn sender(serialized_dart_send_port: &SerializedDartSendPort) -> Rust2DartSender {
    Rust2DartSender::new(dart_send_port_deserialize(serialized_dart_send_port))
}

// frb-coverage:ignore-start
impl<T, Rust2DartCodec: BaseCodec> IntoDart for StreamSinkBase<T, Rust2DartCodec> {
    fn into_dart(self) -> DartAbi {
        unreachable!()
    }
}
// frb-coverage:ignore-end
