//! Manages receiving and sending values across the FFI boundary.

pub use crate::ffi::*;
pub use crate::into_into_dart::IntoIntoDart;
/// The representation of a Dart object outside of the Dart heap.
///
/// Its implementation lies with the Dart language and therefore should not be
/// depended on to be stable.
use std::marker::PhantomData;

/// A wrapper around a Dart [`Isolate`].
#[derive(Clone)]
pub struct Rust2Dart {
    pub(crate) channel: Channel,
}

// api signatures is similar to Flutter Android's callback https://api.flutter.dev/javadoc/io/flutter/plugin/common/MethodChannel.Result.html
impl Rust2Dart {
    /// Create a new wrapper from a raw port.
    pub fn new(port: MessagePort) -> Self {
        Rust2Dart {
            channel: Channel::new(port),
        }
    }

    // TODO should we decouple this: (1) assemble real data (2) post the data
    /// Send a success message back to the specified port.
    pub fn success(&self, result: impl IntoDart) -> bool {
        self.channel.post(vec![
            Rust2DartAction::Success.into_dart(),
            result.into_dart(),
        ])
    }

    /// Send a panic back to the specified port.
    pub fn panic(&self, e: impl IntoDart) -> bool {
        self.channel
            .post(vec![Rust2DartAction::Panic.into_dart(), e.into_dart()])
    }

    /// Send a detailed error back to the specified port.
    pub fn error(&self, e: impl IntoDart) -> bool {
        self.channel
            .post(vec![Rust2DartAction::Error.into_dart(), e.into_dart()])
    }

    /// Close the stream and ignore further messages.
    pub fn close_stream(&self) -> bool {
        self.channel
            .post(vec![Rust2DartAction::CloseStream.into_dart()])
    }
}

/// A callback that receives the return value of Rust functions.
pub struct TaskContext {
    rust2dart: Rust2Dart,
}

impl TaskContext {
    /// Create a new callback from a port wrapper.
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

/// A handle to a [`web_sys::BroadcastChannel`].
#[derive(Clone)]
pub struct ChannelHandle(pub String);

impl ChannelHandle {
    #[cfg(wasm)]
    pub fn port(&self) -> MessagePort {
        PortLike::broadcast(&self.0)
    }
}

