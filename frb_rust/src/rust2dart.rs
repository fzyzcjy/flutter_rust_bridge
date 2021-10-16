use std::marker::PhantomData;

pub use allo_isolate::ffi::DartCObject;
pub use allo_isolate::IntoDart;
use allo_isolate::Isolate;

#[derive(Copy, Clone)]
pub struct Rust2Dart {
    isolate: Isolate,
}

const RUST2DART_ACTION_SUCCESS: i32 = 0;
const RUST2DART_ACTION_ERROR: i32 = 1;

// api signatures is similar to Flutter Android's callback https://api.flutter.dev/javadoc/io/flutter/plugin/common/MethodChannel.Result.html
impl Rust2Dart {
    pub fn new(port: i64) -> Self {
        Rust2Dart {
            isolate: Isolate::new(port),
        }
    }

    pub fn success<T: IntoDart>(&self, result: T) -> bool {
        self.isolate.post(vec![
            RUST2DART_ACTION_SUCCESS.into_dart(),
            result.into_dart(),
        ])
    }

    pub fn error(&self, error_code: String, error_message: String) -> bool {
        self.error_full(error_code, error_message, ())
    }

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
}

pub struct TaskCallback {
    rust2dart: Rust2Dart,
}

impl TaskCallback {
    pub fn new(rust2dart: Rust2Dart) -> Self {
        Self { rust2dart }
    }

    pub fn stream_sink<T: IntoDart>(&self) -> StreamSink<T> {
        StreamSink::new(self.rust2dart)
    }
}

#[derive(Clone)]
pub struct StreamSink<T: IntoDart> {
    rust2dart: Rust2Dart,
    _phantom_data: PhantomData<T>,
}

impl<T: IntoDart> StreamSink<T> {
    pub fn new(rust2dart: Rust2Dart) -> Self {
        Self {
            rust2dart,
            _phantom_data: PhantomData,
        }
    }

    pub fn add(&self, value: T) -> bool {
        self.rust2dart.success(value)
    }
}
