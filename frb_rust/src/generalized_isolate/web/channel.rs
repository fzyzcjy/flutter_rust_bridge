#[derive(Clone)]
pub struct Channel {
    port: MessagePort,
}

impl Channel {
    pub fn new(port: MessagePort) -> Self {
        Self { port }
    }
    pub fn post(&self, msg: impl IntoDart) -> bool {
        self.port
            .post_message(&msg.into_dart())
            .map_err(|err| {
                crate::console_error!("post: {:?}", err);
            })
            .is_ok()
    }
    pub(crate) fn broadcast_name(&self) -> Option<String> {
        self.port
            .dyn_ref::<BroadcastChannel>()
            .map(|channel| channel.name())
    }
}

// TODO the name should reflect "broadcast" channel?
/// A handle to a [`web_sys::BroadcastChannel`] that implements `Send`.
#[derive(Clone)]
pub struct SendableChannelHandle(String);

impl From<&Channel> for SendableChannelHandle {
    fn from(value: &Channel) -> Self {
        Self(value.broadcast_name().expect("Not a BroadcastChannel"))
    }
}

impl From<&SendableChannelHandle> for Channel {
    fn from(value: &SendableChannelHandle) -> Self {
        PortLike::broadcast(&value.0)
    }
}
