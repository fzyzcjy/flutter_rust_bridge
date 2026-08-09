use crate::generalized_isolate::Channel;
use crate::generalized_isolate::IntoDart;
use crate::misc::logs::log_warn_or_println;
use std::fmt;

#[derive(Clone)]
pub struct Rust2DartSender {
    pub(crate) channel: Channel,
}

impl Rust2DartSender {
    pub fn new(channel: Channel) -> Self {
        Rust2DartSender { channel }
    }

    pub fn send(&self, msg: impl IntoDart) -> Result<(), Rust2DartSendError> {
        if self.channel.post(msg) {
            Ok(())
        } else {
            Err(Rust2DartSendError)
        }
    }

    #[cfg(target_family = "wasm")]
    pub fn send_stream(
        &self,
        sequence: u64,
        skipped_sequences: &[u64],
        msg: impl IntoDart,
    ) -> Result<(), Rust2DartSendError> {
        let sequence = ((sequence >> 32) as u32, sequence as u32);
        let msg = msg.into_dart();
        if skipped_sequences.is_empty() {
            return self.send((sequence.0, sequence.1, msg));
        }

        let skipped_sequences = skipped_sequences
            .iter()
            .map(|sequence| ((sequence >> 32) as u32, *sequence as u32))
            .collect::<Vec<_>>();
        self.send((sequence.0, sequence.1, skipped_sequences, msg))
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    pub fn send_or_warn(&self, msg: impl IntoDart) {
        // frb-coverage:ignore-end
        if let Err(e) = self.send(msg) {
            log_warn_or_println(&format!("{e:?}"));
        }
    }
}

/// Error when sending message from rust to dart
#[derive(Clone)]
pub struct Rust2DartSendError;

impl fmt::Debug for Rust2DartSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl fmt::Display for Rust2DartSendError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to post message to Dart. (If happen during Flutter hot restart, this may not be a problem.)")
    }
}

#[cfg(test)]
mod tests {
    use crate::Rust2DartSendError;

    #[test]
    fn test_rust2dart_send_error() {
        assert!(format!("{Rust2DartSendError}").contains("post message"));
        assert!(format!("{Rust2DartSendError:?}").contains("post message"));
    }
}
