use crate::generalized_isolate::IntoDart;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::stream_sink::StreamSink;

/// A context for task execution related to Rust2Dart
pub struct TaskRust2DartContext {
    rust2dart: Rust2Dart,
}

impl TaskRust2DartContext {
    /// Create a new context.
    pub fn new(rust2dart: Rust2Dart) -> Self {
        Self { rust2dart }
    }

    /// Create a new [StreamSink] of the specified type.
    pub fn stream_sink<T, D>(&self) -> StreamSink<T>
        where
            T: IntoIntoDart<D>,
            D: IntoDart,
    {
        StreamSink::new(self.rust2dart.clone())
    }
}
