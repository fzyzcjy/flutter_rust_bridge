use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{release_channel_handle, SendableChannelHandle};
use crate::rust2dart::sender::{Rust2DartSendError, Rust2DartSender};
use std::marker::PhantomData;
#[cfg(target_family = "wasm")]
use std::sync::Mutex;

// *NOT* cloneable, since it invokes stream-close when dropped
pub(crate) struct StreamSinkCloser<Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    #[cfg(target_family = "wasm")]
    next_sequence: Mutex<u64>,
    _phantom_data: PhantomData<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> StreamSinkCloser<Rust2DartCodec> {
    pub fn new(sendable_channel_handle: SendableChannelHandle) -> Self {
        Self {
            sendable_channel_handle,
            #[cfg(target_family = "wasm")]
            next_sequence: Mutex::new(0),
            _phantom_data: PhantomData,
        }
    }

    pub(super) fn send(
        &self,
        message: impl crate::generalized_isolate::IntoDart,
    ) -> Result<(), Rust2DartSendError> {
        let sender = Rust2DartSender::new(crate::generalized_isolate::handle_to_channel(
            &self.sendable_channel_handle,
        ));

        #[cfg(target_family = "wasm")]
        {
            let mut next_sequence = self.next_sequence.lock().unwrap();
            sender.send_stream(*next_sequence, message)?;
            *next_sequence += 1;
            Ok(())
        }

        #[cfg(not(target_family = "wasm"))]
        sender.send(message)
    }
}

impl<Rust2DartCodec: BaseCodec> Drop for StreamSinkCloser<Rust2DartCodec> {
    fn drop(&mut self) {
        if let Err(error) = self.send(Rust2DartCodec::encode_close_stream().into_dart_abi()) {
            crate::misc::logs::log_warn_or_println(&format!("{error:?}"));
        }
        release_channel_handle(&self.sendable_channel_handle);
    }
}
