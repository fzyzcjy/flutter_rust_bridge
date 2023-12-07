use crate::generalized_isolate::{Channel, IntoDart};
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::platform_types::{MessagePort, WireSyncReturn};
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::encoder::Encoder;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrc;

/// The default error handler used by generated code.
#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error(&self, port: MessagePort, error: Error) {
        let msg = match error {
            e @ Error::CustomError(_) => Encoder::error(e),
            e @ Error::Panic(_) => Encoder::panic(e),
        };
        Rust2DartSender::new(Channel::new(port)).send(msg);
    }

    fn handle_error_sync(&self, error: Error) -> WireSyncReturnSrc {
        let result_code = (&error).into();
        WireSyncReturnSrc::new_from_data(error.into_dart(), result_code)
    }
}
