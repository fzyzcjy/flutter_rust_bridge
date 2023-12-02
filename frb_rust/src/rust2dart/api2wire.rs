use allo_isolate::ffi::DartCObject;
use allo_isolate::IntoDart;
use crate::rust2dart::action::Rust2DartAction;

pub struct Api2wire;

impl Api2wire {
    /// A success
    pub fn success(result: impl IntoDart) -> DartCObject {
        vec![
            Rust2DartAction::Success.into_dart(),
            result.into_dart(),
        ].into_dart()
    }

    /// A panic error
    pub fn panic(e: impl IntoDart) -> DartCObject {
        vec![Rust2DartAction::Panic.into_dart(), e.into_dart()].into_dart()
    }

    /// A detailed error
    pub fn error(e: impl IntoDart) -> DartCObject {
        vec![Rust2DartAction::Error.into_dart(), e.into_dart()].into_dart()
    }

    /// Close the stream
    pub fn close_stream() -> DartCObject {
        vec![Rust2DartAction::CloseStream.into_dart()].into_dart()
    }
}
