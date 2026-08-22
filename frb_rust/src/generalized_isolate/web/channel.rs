use crate::generalized_isolate::IntoDart;
use crate::platform_types::post_cached_message_port;
use crate::platform_types::release_message_port_handle;
use crate::platform_types::MessagePort;
use crate::platform_types::{message_port_to_handle, SendableMessagePortHandle};

#[derive(Clone)]
pub struct Channel {
    port: ChannelPort,
}

#[derive(Clone)]
enum ChannelPort {
    Direct(MessagePort),
    Cached(SendableMessagePortHandle),
}

impl Channel {
    pub fn new(port: MessagePort) -> Self {
        Self {
            port: ChannelPort::Direct(port),
        }
    }

    pub fn post(&self, msg: impl IntoDart) -> bool {
        let msg = msg.into_dart();
        match &self.port {
            ChannelPort::Direct(port) => port
                .post_message(&msg)
                .map_err(|err| {
                    crate::console_error!("post: {:?}", err);
                })
                .is_ok(),
            ChannelPort::Cached(handle) => post_cached_message_port(handle, msg),
        }
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
    let ChannelPort::Direct(port) = &channel.port else {
        unreachable!()
    };
    SendableChannelHandle(message_port_to_handle(port))
}

pub fn handle_to_channel(handle: &SendableChannelHandle) -> Channel {
    Channel {
        port: ChannelPort::Cached(handle.0.clone()),
    }
}

pub fn release_channel_handle(handle: &SendableChannelHandle) {
    release_message_port_handle(&handle.0);
}
