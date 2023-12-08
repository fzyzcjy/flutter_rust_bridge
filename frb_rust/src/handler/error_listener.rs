use crate::codec::BaseCodec;
use crate::handler::error::Error;
use crate::platform_types::MessagePort;
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Listens when error happens
pub(crate) trait ErrorListener: UnwindSafe + RefUnwindSafe + Copy + Send + 'static {
    fn on_error(&self, error: Error);
}
