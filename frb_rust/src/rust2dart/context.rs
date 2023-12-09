use crate::codec::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::misc::into_into_dart::IntoIntoDart;
use crate::rust2dart::sender::Rust2DartSender;
use crate::rust2dart::stream_sink::StreamSink;
use std::marker::PhantomData;

/// A context for task execution related to Rust2Dart
pub struct TaskRust2DartContext<Rust2DartCodec: BaseCodec> {
    sender: Rust2DartSender,
    _phantom: PhantomData<Rust2DartCodec>,
}

impl<Rust2DartCodec: BaseCodec> TaskRust2DartContext<Rust2DartCodec> {
    /// Create a new context.
    pub fn new(sender: Rust2DartSender) -> Self {
        Self {
            sender,
            _phantom: Default::default(),
        }
    }

    /// Create a new [StreamSink] of the specified type.
    pub fn stream_sink<T, D>(&self) -> StreamSink<T, Rust2DartCodec>
    where
        T: IntoIntoDart<D>,
        D: IntoDart,
    {
        StreamSink::<T, Rust2DartCodec>::new(self.sender.clone())
    }
}
