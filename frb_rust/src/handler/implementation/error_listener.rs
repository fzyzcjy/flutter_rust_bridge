use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::handler::error::Error;
use crate::handler::error_listener::ErrorListener;
use crate::platform_types::MessagePort;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::sender::Rust2DartSender;

/// The default one.
#[derive(Clone, Copy)]
pub struct NoOpErrorListener;

impl ErrorListener for NoOpErrorListener {
    fn on_error(&self, error: Error) {
        // nothing
    }

    // TODO
    // fn handle_error<Rust2DartCodec>(&self, port: MessagePort, error: Error)
    // where
    //     Rust2DartCodec: BaseCodec,
    // {
    //     let msg = match error {
    //         e @ Error::CustomError(_) => Rust2DartCodec::encode(e, Rust2DartAction::Error),
    //         e @ Error::Panic(_) => Rust2DartCodec::encode(e, Rust2DartAction::Panic),
    //     };
    //     Rust2DartSender::new(Channel::new(port)).send(msg.into_dart_abi());
    // }
}
