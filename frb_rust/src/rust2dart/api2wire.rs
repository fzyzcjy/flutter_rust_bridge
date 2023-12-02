use allo_isolate::IntoDart;

pub struct Api2wire;

impl Api2wire {
    /// A success
    pub fn success(&self, result: impl IntoDart) -> bool {
        vec![
            Rust2DartAction::Success.into_dart(),
            result.into_dart(),
        ]
    }

    /// A panic error
    pub fn panic(&self, e: impl IntoDart) -> bool {
        vec![Rust2DartAction::Panic.into_dart(), e.into_dart()]
    }

    /// A detailed error
    pub fn error(&self, e: impl IntoDart) -> bool {
        vec![Rust2DartAction::Error.into_dart(), e.into_dart()]
    }

    /// Close the stream
    pub fn close_stream(&self) -> bool {
        vec![Rust2DartAction::CloseStream.into_dart()]
    }
}
