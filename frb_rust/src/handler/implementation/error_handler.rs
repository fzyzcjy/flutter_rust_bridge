use allo_isolate::IntoDart;
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::platform_types::{MessagePort, WireSyncReturn};
use crate::rust2dart::action::Rust2DartAction;

/// The default error handler used by generated code.
#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error(&self, port: MessagePort, error: Error) {
        match error {
            e @ Error::CustomError(_) => Rust2Dart::new(port).error(e),
            e @ Error::Panic(_) => Rust2Dart::new(port).panic(e),
        };
    }

    fn handle_error_sync(&self, error: Error) -> WireSyncReturn {
        let result_code = (&error).into();
        wire_sync_from_data(error.into_dart(), result_code)
    }
}
