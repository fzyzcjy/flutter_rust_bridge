use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::handler::error::Error;
use crate::handler::error_handler::ErrorHandler;
use crate::platform_types::MessagePort;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::sender::Rust2DartSender;

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

    fn handle_error_sync<Rust2DartCodec>(&self, error: Error) -> Rust2DartCodec::Message
    where
        Rust2DartCodec: BaseCodec,
    {
        let result_code = (&error).into();
        Rust2DartCodec::Message::new(Rust2DartCodec::encode(error, result_code))
    }
}
