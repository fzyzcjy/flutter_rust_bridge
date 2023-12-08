use crate::codec::BaseCodec;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::platform_types::MessagePort;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnSrc;

/// The default error handler used by generated code.
#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error<Rust2DartCodec>(&self, port: MessagePort, error: Error)
    where
        Rust2DartCodec: BaseCodec,
    {
        let msg = match error {
            e @ Error::CustomError(_) => Rust2DartCodec::encode(e, Rust2DartAction::Error),
            e @ Error::Panic(_) => Rust2DartCodec::encode(e, Rust2DartAction::Panic),
        };
        Rust2DartSender::new(Channel::new(port)).send(msg);
    }

    fn handle_error_sync<Rust2DartCodec>(&self, error: Error) -> WireSyncReturnSrc
    where
        Rust2DartCodec: BaseCodec,
    {
        let result_code = (&error).into();
        WireSyncReturnSrc::new(Rust2DartCodec::encode(error, result_code))
    }
}
