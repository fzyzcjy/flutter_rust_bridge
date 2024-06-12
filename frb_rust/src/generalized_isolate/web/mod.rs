mod into_dart;
mod misc;

use crate::for_generated::DartNativeSendPort;

pub use into_dart::*;
pub use misc::*;

#[derive(Debug, Clone)]
pub struct DartSendPort(i64);

impl DartSendPort {
    pub fn new(native: DartNativeSendPort) -> Self {
        Self(todo!())
    }

    pub fn post(&self, msg: impl IntoDart) -> bool {
        todo!()
    }
}
