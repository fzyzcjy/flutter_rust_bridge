use crate::codec::BaseCodec;
use crate::codec::Rust2DartMessageTrait;
use crate::generalized_isolate::{Channel, IntoDart};
use crate::handler::error::{encode_panic_error, Error};
use crate::handler::error_listener::ErrorListener;
use crate::platform_types::MessagePort;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::sender::Rust2DartSender;
use std::any::Any;

/// The default one.
#[derive(Clone, Copy)]
pub struct NoOpErrorListener;

impl ErrorListener for NoOpErrorListener {
    fn on_error(&self, error: Error) {
        // nothing
    }
}

pub(crate) fn handle_non_sync_panic_error<Rust2DartCodec: BaseCodec>(
    error_listener: impl ErrorListener,
    port: MessagePort,
    error: Box<dyn Any + Send>,
) {
    let message = encode_panic_error::<Rust2DartCodec>(&error).into_dart_abi();
    error_listener.on_error(Error::Panic(error));
    Rust2DartSender::new(Channel::new(port)).send(message);
}
