use crate::generalized_isolate::{channel_to_handle, handle_to_channel, Channel, IntoDart};
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::platform_types::{MessagePort, WireSyncReturn};
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::api2wire::Api2Wire;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrc;

/// The default error handler used by generated code.
#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error(&self, port: MessagePort, error: Error) {
        let msg = match error {
            e @ Error::CustomError(_) => Api2Wire::error(e),
            e @ Error::Panic(_) => Api2Wire::panic(e),
        };

        // TODO HACK!!!
        log::warn!("hack!!!!!!!!!");
        crate::console_error!("hi port={:?}", port);
        let ch1 = Channel::new(port);
        let ch2 = handle_to_channel(&channel_to_handle(&ch1));
        crate::console_error!("hi (old)ch1={:?} (new)ch2={:?}", ch1, ch2);
        Rust2DartSender::new(ch2).send(msg);
    }

    fn handle_error_sync(&self, error: Error) -> WireSyncReturnSrc {
        let result_code = (&error).into();
        WireSyncReturnSrc::new_from_data(error.into_dart(), result_code)
    }
}
