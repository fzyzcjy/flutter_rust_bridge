use crate::generalized_isolate::Channel;
use crate::generalized_isolate::IntoDart;
use crate::misc::logs::log_warn_or_println;
use anyhow::anyhow;

#[derive(Clone)]
pub struct Rust2DartSender {
    pub(crate) channel: Channel,
}

impl Rust2DartSender {
    pub fn new(channel: Channel) -> Self {
        Rust2DartSender { channel }
    }

    pub fn send(&self, msg: impl IntoDart) -> anyhow::Result<()> {
        if self.channel.post(msg) {
            Ok(())
        } else {
            Err(anyhow!("Fail to post message to Dart"))
        }
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
