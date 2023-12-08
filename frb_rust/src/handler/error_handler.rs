use crate::codec::BaseCodec;
use crate::handler::error::Error;
use crate::platform_types::MessagePort;
use std::panic::{RefUnwindSafe, UnwindSafe};

/// A handler model that sends back the error to a Dart `SendPort`.
///
/// For example, instead of using the default [`ReportDartErrorHandler`],
/// you could implement your own handler that logs each error to stderr,
/// or to an external logging service.
pub(crate) trait ErrorHandler: UnwindSafe + RefUnwindSafe + Copy + Send + 'static {
    fn on_error(&self, error: Error);
}
