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
pub struct NoOpErrorHandler;

impl ErrorHandler for NoOpErrorHandler {
    fn on_error(&self, error: Error) {
        // nothing
    }
}

fn handle_error<Rust2DartCodec>(error_handler: impl ErrorHandler, port: MessagePort, error: Error)
where
    Rust2DartCodec: BaseCodec,
{
    error_handler.on_error(error);

    let msg = match error {
        e @ Error::CustomError => Rust2DartCodec::encode(e, Rust2DartAction::Error),
        e @ Error::Panic(_) => Rust2DartCodec::encode(e, Rust2DartAction::Panic),
    };
    Rust2DartSender::new(Channel::new(port)).send(msg.into_dart_abi());
}
