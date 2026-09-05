use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{release_channel_handle, SendableChannelHandle};
use std::marker::PhantomData;
#[cfg(target_family = "wasm")]
use std::sync::atomic::{AtomicU64, Ordering};

// *NOT* cloneable, since it invokes stream-close when dropped
pub(crate) struct StreamSinkCloser<Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    _phantom_data: PhantomData<Rust2DartCodec>,
    #[cfg(target_family = "wasm")]
    pub(super) sent: AtomicU64,
}

impl<Rust2DartCodec: BaseCodec> StreamSinkCloser<Rust2DartCodec> {
    pub fn new(sendable_channel_handle: SendableChannelHandle) -> Self {
        Self {
            sendable_channel_handle,
            _phantom_data: PhantomData,
            #[cfg(target_family = "wasm")]
            sent: AtomicU64::new(0),
        }
    }
}

impl<Rust2DartCodec: BaseCodec> Drop for StreamSinkCloser<Rust2DartCodec> {
    fn drop(&mut self) {
        let message = Rust2DartCodec::encode_close_stream().into_dart_abi();
        #[cfg(target_family = "wasm")]
        let message: wasm_bindgen::JsValue = js_sys::Array::of3(
            &"__frb_stream_close".into(),
            &(self.sent.load(Ordering::Relaxed) as f64).into(),
            &message,
        )
        .into();
        super::stream_sink::sender(&self.sendable_channel_handle).send_or_warn(message);
        release_channel_handle(&self.sendable_channel_handle);
    }
}
