//! Manages receiving and sending values across the FFI boundary.

use std::marker::PhantomData;

/// The representation of a Dart object outside of the Dart heap.
///
/// Its implementation lies with the Dart language and therefore should not be
/// depended on to be stable.
pub use crate::ffi::*;

/// A wrapper around a Dart [`Isolate`].
#[derive(Clone)]
pub struct Rust2Dart {
    pub(crate) isolate: Isolate,
}

const RUST2DART_ACTION_SUCCESS: i32 = 0;
const RUST2DART_ACTION_ERROR: i32 = 1;
const RUST2DART_ACTION_CLOSE_STREAM: i32 = 2;

// api signatures is similar to Flutter Android's callback https://api.flutter.dev/javadoc/io/flutter/plugin/common/MethodChannel.Result.html
impl Rust2Dart {
    /// Create a new wrapper from a raw port.
    pub fn new(port: MessagePort) -> Self {
        Rust2Dart {
            isolate: Isolate::new(port),
        }
    }

    /// Send a success message back to the specified port.
    pub fn success(&self, result: impl IntoDart) -> bool {
        self.isolate.post(vec![
            RUST2DART_ACTION_SUCCESS.into_dart(),
            result.into_dart(),
        ])
    }

    /// Send an error back to the specified port.
    pub fn error(&self, error_code: String, error_message: String) -> bool {
        self.error_full(error_code, error_message, ())
    }

    /// Send a detailed error back to the specified port.
    pub fn error_full(
        &self,
        error_code: String,
        error_message: String,
        error_details: impl IntoDart,
    ) -> bool {
        self.isolate.post(vec![
            RUST2DART_ACTION_ERROR.into_dart(),
            error_code.into_dart(),
            error_message.into_dart(),
            error_details.into_dart(),
        ])
    }

    /// Close the stream and ignore further messages.
    pub fn close_stream(&self) -> bool {
        self.isolate
            .post(vec![RUST2DART_ACTION_CLOSE_STREAM.into_dart()])
    }
}

/// A callback that receives the return value of Rust functions.
pub struct TaskCallback {
    rust2dart: Rust2Dart,
}

impl TaskCallback {
    /// Create a new callback from a port wrapper.
    pub fn new(rust2dart: Rust2Dart) -> Self {
        Self { rust2dart }
    }

    /// Create a new [StreamSink] of the specified type.
    pub fn stream_sink<T: IntoDart>(&self) -> StreamSink<T> {
        StreamSink::new(self.rust2dart.clone())
    }
}

#[derive(Clone)]
pub struct Channel(pub String);

impl Channel {
    #[cfg(wasm)]
    pub fn port(&self) -> MessagePort {
        PortLike::broadcast(&self.0)
    }
}

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSink<T: IntoDart> {
    #[cfg(not(wasm))]
    rust2dart: Rust2Dart,
    /// On WASM, [`Rust2Dart`] is not [`Send`] so we only
    /// carry the name of the broadcast channel.
    #[cfg(wasm)]
    channel: Channel,
    _phantom_data: PhantomData<T>,
}

impl<T: IntoDart> StreamSink<T> {
    /// Create a new sink from a port wrapper.
    pub fn new(rust2dart: Rust2Dart) -> Self {
        Self {
            #[cfg(not(wasm))]
            rust2dart,
            #[cfg(wasm)]
            channel: Channel(
                rust2dart
                    .isolate
                    .broadcast_name()
                    .expect("Not a BroadcastChannel"),
            ),
            _phantom_data: PhantomData,
        }
    }

    fn rust2dart(&self) -> Rust2Dart {
        #[cfg(not(wasm))]
        return self.rust2dart.clone();

        #[cfg(wasm)]
        Rust2Dart::new(self.channel.port())
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add(&self, value: T) -> bool {
        self.rust2dart().success(value)
    }

    /// Close the stream and ignore further messages. Returns false when
    /// the stream could not be closed, or when it has already been closed.
    pub fn close(&self) -> bool {
        self.rust2dart().close_stream()
    }
}
