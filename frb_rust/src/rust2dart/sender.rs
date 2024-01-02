use crate::generalized_isolate::Channel;
use crate::generalized_isolate::IntoDart;
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
}
