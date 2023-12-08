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
    /// The default error handler.
    fn handle_error<Rust2DartCodec>(&self, port: MessagePort, error: Error)
    where
        Rust2DartCodec: BaseCodec;

    /// Special handler only used for synchronous code.
    fn handle_error_sync<Rust2DartCodec>(&self, error: Error) -> Rust2DartCodec::WireSyncReturnSrc
    where
        Rust2DartCodec: BaseCodec;
}
