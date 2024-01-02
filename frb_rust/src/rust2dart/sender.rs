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

    pub fn send(&self, msg: impl IntoDart) -> Result<(), Rust2DartSendError> {
        if self.channel.post(msg) {
            Ok(())
        } else {
            Err(Rust2DartSendError)
        }
    }

    pub fn send_or_warn(&self, msg: impl IntoDart) {
        if let Err(_) = self.send(msg) {
            log_warn_or_println(TODO);
        }
    }
}

pub struct Rust2DartSendError;
