use allo_isolate::IntoDart;
use crate::generalized_isolate::Channel;
use crate::platform_types::MessagePort;

#[derive(Clone)]
pub struct Rust2DartSender {
    pub(crate) channel: Channel,
}

impl Rust2DartSender {
    pub fn new(port: MessagePort) -> Self {
        Rust2DartSender {
            channel: Channel::new(port),
        }
    }

    pub fn send(&self, msg: impl IntoDart) -> bool {
        self.channel.post(msg)
    }
}