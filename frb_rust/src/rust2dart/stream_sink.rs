use std::marker::PhantomData;
use crate::generalized_isolate::IntoDart;
use crate::misc::into_into_dart::IntoIntoDart;

/// A sink to send asynchronous data back to Dart.
/// Represented as a Dart
/// [`Stream`](https://api.dart.dev/stable/dart-async/Stream-class.html).
#[derive(Clone)]
pub struct StreamSink<T> {
    // TODO refactor this `cfg`?
    // TODO e.g. extract sth like a "Sendable Rust2Dart"
    #[cfg(not(wasm))]
    rust2dart: Rust2Dart,
    #[cfg(wasm)]
    handle: ChannelHandle,
    _phantom_data: PhantomData<T>,
}

impl<T> StreamSink<T> {
    /// Create a new sink from a port wrapper.
    pub fn new(rust2dart: Rust2Dart) -> Self {
        #[cfg(wasm)]
        Self {
            #[cfg(not(wasm))]
            rust2dart,
            #[cfg(wasm)]
            handle: ChannelHandle(name),
            _phantom_data: PhantomData,
        }
    }

    fn rust2dart(&self) -> Rust2Dart {
        #[cfg(not(wasm))]
        return self.rust2dart.clone();

        #[cfg(wasm)]
        Rust2Dart::new(self.handle.port())
    }

    /// Add data to the stream. Returns false when data could not be sent,
    /// or the stream has been closed.
    pub fn add<D: IntoDart>(&self, value: T) -> bool
        where
            T: IntoIntoDart<D>,
    {
        self.rust2dart().success(value.into_into_dart().into_dart())
    }

    /// Close the stream and ignore further messages. Returns false when
    /// the stream could not be closed, or when it has already been closed.
    pub fn close(&self) -> bool {
        self.rust2dart().close_stream()
    }
}
