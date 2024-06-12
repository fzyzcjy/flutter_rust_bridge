mod into_dart;
mod misc;

use crate::for_generated::DartNativeSendPort;

pub use into_dart::*;
pub use misc::*;

#[derive(Debug, Clone)]
pub struct DartSendPort(web_sys::BroadcastChannel);

impl DartSendPort {
    pub fn new(native: DartNativeSendPort) -> Self {
        Self(web_sys::BroadcastChannel::new(&native).unwrap())
    }

    pub fn post(&self, msg: impl IntoDart) -> bool {
        (self.0.post_message(&msg.into_dart()))
            .map_err(|err| crate::console_error!("error when DartSendPort.post: {:?}", err))
            .is_ok()
    }
}
