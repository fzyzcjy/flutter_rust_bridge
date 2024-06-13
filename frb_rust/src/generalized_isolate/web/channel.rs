use crate::generalized_isolate::IntoDart;
use crate::platform_types::handle_to_message_port;
use crate::platform_types::MessagePort;
use crate::platform_types::{message_port_to_handle, SendableMessagePortHandle};

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

    // TODO unused, rm?
    // pub(crate) fn broadcast_name(&self) -> Option<String> {
    //     self.port
    //         .dyn_ref::<BroadcastChannel>()
    //         .map(|channel| channel.name())
    // }
}

// TODO the name should reflect "broadcast" channel?
/// A handle to a [`web_sys::BroadcastChannel`] that implements `Send`.
#[derive(Clone)]
pub struct SendableChannelHandle(SendableMessagePortHandle);

pub fn channel_to_handle(channel: &Channel) -> SendableChannelHandle {
    SendableChannelHandle(message_port_to_handle(&channel.port))
}

pub fn handle_to_channel(handle: &SendableChannelHandle) -> Channel {
    Channel::new(handle_to_message_port(&handle.0))
}
