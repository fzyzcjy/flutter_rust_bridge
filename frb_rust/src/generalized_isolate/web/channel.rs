use crate::generalized_isolate::IntoDart;
use crate::generalized_isolate::PortLike;
use crate::platform_types::MessagePort;
use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

#[derive(Clone, Debug)]
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

pub fn channel_to_handle(channel: &Channel) -> SendableChannelHandle {
    SendableChannelHandle(channel.broadcast_name().expect("Not a BroadcastChannel"))
}

pub fn handle_to_channel(handle: &SendableChannelHandle) -> Channel {
    Channel::new(PortLike::broadcast(&handle.0))
}
