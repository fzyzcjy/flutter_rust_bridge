use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::for_generated::DartAbi;
use crate::generalized_isolate::DartSendPort;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::deserialize_dart_native_send_port;
use crate::rust2dart::sender::{Rust2DartSendError, Rust2DartSender};
use crate::stream::closer::StreamSinkCloser;
use std::marker::PhantomData;
use std::sync::Arc;

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSinkBase<T, Rust2DartCodec: BaseCodec> {
    dart_send_port: DartSendPort,
    _closer: Arc<StreamSinkCloser<Rust2DartCodec>>,
    _phantom_data: (PhantomData<T>, PhantomData<Rust2DartCodec>),
}

impl<T, Rust2DartCodec: BaseCodec> StreamSinkBase<T, Rust2DartCodec> {
    pub fn deserialize(raw: String) -> Self {
        let dart_send_port = DartSendPort::new(deserialize_dart_native_send_port(raw));
        Self {
            #[allow(clippy::clone_on_copy)]
            dart_send_port: dart_send_port.clone(),
            _closer: Arc::new(StreamSinkCloser::new(dart_send_port)),
            _phantom_data: Default::default(),
        }
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add_raw(&self, value: Rust2DartCodec::Message) -> Result<(), Rust2DartSendError> {
        sender(&self.dart_send_port).send(value.into_dart_abi())
    }
}

pub(super) fn sender(dart_send_port: &DartSendPort) -> Rust2DartSender {
    Rust2DartSender::new(dart_send_port.to_owned())
}

// frb-coverage:ignore-start
impl<T, Rust2DartCodec: BaseCodec> IntoDart for StreamSinkBase<T, Rust2DartCodec> {
    fn into_dart(self) -> DartAbi {
        unreachable!()
    }
}
// frb-coverage:ignore-end
