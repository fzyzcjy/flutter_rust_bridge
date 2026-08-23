use crate::generalized_isolate::IntoDart;
use crate::platform_types::handle_to_message_port;
use crate::platform_types::release_message_port_handle;
use crate::platform_types::MessagePort;
use crate::platform_types::{message_port_to_handle, SendableMessagePortHandle};
use wasm_bindgen::JsCast;
use web_sys::BroadcastChannel;

#[derive(Clone)]
pub struct Channel {
    port: MessagePort,
}

impl Channel {
    pub fn new(port: MessagePort) -> Self {
        Self { port }
    }

    pub fn post(&self, msg: impl IntoDart) -> bool {
        let msg = msg.into_dart();
        let diagnostic_channel_name = self
            .port
            .dyn_ref::<BroadcastChannel>()
            .map(BroadcastChannel::name)
            .filter(|name| name.starts_with("__frb_streamsink_RustStreamSink_"));
        if let Some(name) = &diagnostic_channel_name {
            crate::for_generated::web_utils::js_console_log(&format!(
                "FRB_STREAMSINK_DIAGNOSTIC rust_post_start channel={name} payload={msg:?}"
            ));
        }

        let result = self.port.post_message(&msg).map_err(|err| {
            crate::console_error!("post: {:?}", err);
        });
        if let Some(name) = &diagnostic_channel_name {
            crate::for_generated::web_utils::js_console_log(&format!(
                "FRB_STREAMSINK_DIAGNOSTIC rust_post_finish channel={name} success={}",
                result.is_ok()
            ));
        }
        result.is_ok()
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

pub fn release_channel_handle(handle: &SendableChannelHandle) {
    release_message_port_handle(&handle.0);
}
