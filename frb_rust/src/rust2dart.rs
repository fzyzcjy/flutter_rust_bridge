pub use allo_isolate::ffi::DartCObject;
pub use allo_isolate::IntoDart;
use allo_isolate::Isolate;

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
