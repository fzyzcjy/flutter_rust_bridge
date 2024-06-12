use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::DartSendPort;
use std::marker::PhantomData;

// *NOT* cloneable, since it invokes stream-close when dropped
pub(crate) struct StreamSinkCloser<Rust2DartCodec: BaseCodec> {
    dart_send_port: DartSendPort,
    _phantom_data: PhantomData<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> StreamSinkCloser<Rust2DartCodec> {
    pub fn new(dart_send_port: DartSendPort) -> Self {
        Self {
            dart_send_port,
            _phantom_data: PhantomData,
        }
    }
}

impl<Rust2DartCodec: BaseCodec> Drop for StreamSinkCloser<Rust2DartCodec> {
    fn drop(&mut self) {
        super::stream_sink::sender(&self.dart_send_port)
            .send_or_warn(Rust2DartCodec::encode_close_stream().into_dart_abi())
    }
}
