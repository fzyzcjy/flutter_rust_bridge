use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{release_cached_channel_handle, SendableChannelHandle};
#[cfg(target_family = "wasm")]
use crate::misc::atomic::{AtomicU64, Ordering};
use std::marker::PhantomData;

// *NOT* cloneable, since it invokes stream-close when dropped
pub(crate) struct StreamSinkCloser<Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    #[cfg(target_family = "wasm")]
    next_sequence: AtomicU64,
    _phantom_data: PhantomData<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> StreamSinkCloser<Rust2DartCodec> {
    pub fn new(sendable_channel_handle: SendableChannelHandle) -> Self {
        Self {
            sendable_channel_handle,
            #[cfg(target_family = "wasm")]
            next_sequence: AtomicU64::new(0),
            _phantom_data: PhantomData,
        }
    }

    pub(super) fn next_sequence(&self) -> u64 {
        #[cfg(target_family = "wasm")]
        {
            self.next_sequence.fetch_add(1, Ordering::Relaxed)
        }

        #[cfg(not(target_family = "wasm"))]
        {
            0
        }
    }
}

impl<Rust2DartCodec: BaseCodec> Drop for StreamSinkCloser<Rust2DartCodec> {
    fn drop(&mut self) {
        let sequence = self.next_sequence();
        super::stream_sink::sender(&self.sendable_channel_handle).send_stream_or_warn(
            sequence,
            Rust2DartCodec::encode_close_stream().into_dart_abi(),
        );
        release_cached_channel_handle(&self.sendable_channel_handle);
    }
}
