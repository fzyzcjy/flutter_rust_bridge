use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
#[cfg(target_family = "wasm")]
use crate::generalized_isolate::release_cached_channel_handle;
use crate::generalized_isolate::{IntoDart, SendableChannelHandle};
#[cfg(any(target_family = "wasm", test))]
use crate::misc::atomic::{AtomicBool, AtomicU64, Ordering};
use crate::misc::logs::log_warn_or_println;
use crate::rust2dart::sender::{Rust2DartSendError, Rust2DartSender};
use std::marker::PhantomData;
#[cfg(any(target_family = "wasm", test))]
use std::sync::Mutex;

// *NOT* cloneable, since it invokes stream-close when dropped
pub(crate) struct StreamSinkCloser<Rust2DartCodec: BaseCodec> {
    sendable_channel_handle: SendableChannelHandle,
    #[cfg(target_family = "wasm")]
    sequence: StreamSequence,
    _phantom_data: PhantomData<Rust2DartCodec>,
}

#[cfg(any(target_family = "wasm", test))]
struct StreamSequence {
    next: AtomicU64,
    has_failed: AtomicBool,
    failed: Mutex<Vec<u64>>,
}

impl<Rust2DartCodec: BaseCodec> StreamSinkCloser<Rust2DartCodec> {
    pub fn new(sendable_channel_handle: SendableChannelHandle) -> Self {
        Self {
            sendable_channel_handle,
            #[cfg(target_family = "wasm")]
            sequence: StreamSequence::new(),
            _phantom_data: PhantomData,
        }
    }

    pub(super) fn send(
        &self,
        sender: Rust2DartSender,
        msg: impl IntoDart,
    ) -> Result<(), Rust2DartSendError> {
        self.send_inner(sender, msg, false)
    }

    fn send_inner(
        &self,
        sender: Rust2DartSender,
        msg: impl IntoDart,
        release_after_delivery: bool,
    ) -> Result<(), Rust2DartSendError> {
        #[cfg(target_family = "wasm")]
        {
            let sequence = self.sequence.next();
            let failed = self.sequence.take_failed_before(sequence);
            let result = sender.send_stream(sequence, &failed, msg, release_after_delivery);
            if result.is_err() {
                self.sequence.record_failed(sequence, failed);
            }
            result
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let _ = release_after_delivery;
            sender.send(msg)
        }
    }
}

#[cfg(any(target_family = "wasm", test))]
impl StreamSequence {
    fn new() -> Self {
        Self {
            next: AtomicU64::new(0),
            has_failed: AtomicBool::new(false),
            failed: Mutex::new(Vec::new()),
        }
    }

    fn next(&self) -> u64 {
        self.next.fetch_add(1, Ordering::Relaxed)
    }

    fn take_failed_before(&self, sequence: u64) -> Vec<u64> {
        if !self.has_failed.load(Ordering::Acquire) {
            return Vec::new();
        }

        let mut failed = self.failed.lock().unwrap();
        let (output, retained) = std::mem::take(&mut *failed)
            .into_iter()
            .partition(|failed_sequence| *failed_sequence < sequence);
        *failed = retained;
        self.has_failed.store(!failed.is_empty(), Ordering::Release);
        output
    }

    fn record_failed(&self, sequence: u64, mut carried: Vec<u64>) {
        let mut failed = self.failed.lock().unwrap();
        failed.append(&mut carried);
        failed.push(sequence);
        self.has_failed.store(true, Ordering::Release);
    }
}

impl<Rust2DartCodec: BaseCodec> Drop for StreamSinkCloser<Rust2DartCodec> {
    fn drop(&mut self) {
        #[cfg(target_family = "wasm")]
        let result = {
            let result = self.send_inner(
                super::stream_sink::sender(&self.sendable_channel_handle),
                Rust2DartCodec::encode_close_stream().into_dart_abi(),
                true,
            );
            if result.is_err() {
                self.send_inner(
                    super::stream_sink::uncached_sender(&self.sendable_channel_handle),
                    Rust2DartCodec::encode_close_stream().into_dart_abi(),
                    true,
                )
            } else {
                result
            }
        };

        #[cfg(not(target_family = "wasm"))]
        let result = self.send(
            super::stream_sink::sender(&self.sendable_channel_handle),
            Rust2DartCodec::encode_close_stream().into_dart_abi(),
        );

        if let Err(error) = result {
            log_warn_or_println(&format!("{error:?}"));
            #[cfg(target_family = "wasm")]
            release_cached_channel_handle(&self.sendable_channel_handle);
        } else {
            #[cfg(target_family = "wasm")]
            crate::console_error!("stream final close posted");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StreamSequence;

    /// Carries every failed sequence on the next attempted message.
    #[test]
    fn failed_sequences_are_carried_forward() {
        let sequence = StreamSequence::new();

        let first = sequence.next();
        sequence.record_failed(first, sequence.take_failed_before(first));
        let second = sequence.next();
        let carried = sequence.take_failed_before(second);
        sequence.record_failed(second, carried);
        let third = sequence.next();

        assert_eq!(sequence.take_failed_before(third), vec![first, second]);
    }

    /// Leaves failures from later concurrent senders for a later carrier.
    #[test]
    fn later_failed_sequences_are_not_carried_backwards() {
        let sequence = StreamSequence::new();

        let first = sequence.next();
        let second = sequence.next();
        sequence.record_failed(second, Vec::new());

        assert!(sequence.take_failed_before(first).is_empty());
        assert_eq!(sequence.take_failed_before(second + 1), vec![second]);
    }

    /// Carries a failed final sequence on a fresh close retry.
    #[test]
    fn failed_final_sequence_is_carried_by_retry() {
        let sequence = StreamSequence::new();

        let first_close = sequence.next();
        sequence.record_failed(first_close, Vec::new());
        let retry_close = sequence.next();

        assert_eq!(sequence.take_failed_before(retry_close), vec![first_close]);
    }
}
