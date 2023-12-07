use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;

pub struct Encoder;

impl Encoder {
    /// A success
    pub fn success(result: impl IntoDart) -> DartAbi {
        vec![Rust2DartAction::Success.into_dart(), result.into_dart()].into_dart()
    }

    /// A panic error
    pub fn panic(e: impl IntoDart) -> DartAbi {
        vec![Rust2DartAction::Panic.into_dart(), e.into_dart()].into_dart()
    }

    /// A detailed error
    pub fn error(e: impl IntoDart) -> DartAbi {
        vec![Rust2DartAction::Error.into_dart(), e.into_dart()].into_dart()
    }

    /// Close the stream
    pub fn close_stream() -> DartAbi {
        vec![Rust2DartAction::CloseStream.into_dart()].into_dart()
    }
}
