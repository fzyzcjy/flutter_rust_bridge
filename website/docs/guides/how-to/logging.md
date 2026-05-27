# Logging

Since I have seen some questions asking how logging can be implemented with a Flutter + Rust application, here are some examples.

## Approach 1: Use the default one

If using the template by `flutter_rust_bridge_codegen create/integrate`, the "print logs to console" is configured by default,
via the auto-generated call to `flutter_rust_bridge::setup_default_user_utils()`.

Thus, you do not need to do anything :)

## Example 2: Print logs to console

```rust
fn setup_the_logger() {
    #[cfg(target_os = "android")]
    android_logger::init_once(android_logger::Config::default().with_max_level(LevelFilter::Trace));

    #[cfg(target_os = "ios")]
    oslog::OsLogger::new("com.example.test").level_filter(LevelFilter::Trace).init().unwrap();
}
```

In other words, use the corresponding platform logger
(https://crates.io/crates/android_logger and https://crates.io/crates/oslog).

## Example 3: My logger in production

In my own app in production, I use the following strategy for Rust logging: Use normal Rust logging methods, such as `info!` and `debug!` macros. The logs are consumed in two places: They are printed via platform-specific methods (like android Logcat and iOS NSLog), and also use a Stream to send them to the Dart side such that my Dart code and further process are using the same pipeline as normal Dart logs (e.g. save to a file, send to server, etc).

The *full* code related to logging in my app can be seen here: [#486](https://github.com/fzyzcjy/flutter_rust_bridge/issues/486).

## Example 4: Send Rust logs to Dart

@MnlPhlp encapsulates the step-by-step example below into a small Rust package,
such that you can setup Rust-logging-to-Dart in several lines.
Please refer to https://github.com/mnlphlp/flutter_logger for details.

## Example 5: A step-by-step guide to send Rust logs to Dart

Let us implement a simple logging system (adapted from the logging system I use with `flutter_rust_bridge` in my app in production), where Rust code can send logs to Dart code.

The Rust `api.rs`:

```rust
pub struct LogEntry {
    pub time_millis: i64,
    pub level: i32,
    pub tag: String,
    pub msg: String,
}

// Simplified just for demonstration.
// To compile, you need a OnceCell, or Mutex, or RwLock
// Also see https://github.com/fzyzcjy/flutter_rust_bridge/issues/398
lazy_static! { static ref log_stream_sink: StreamSink<LogEntry>; }

pub fn create_log_stream(s: StreamSink<LogEntry>) {
    stream_sink = s;
}
```

Now Rust will probably complain at you because `IntoDart` is not implemented for `LogEntry`. This is expected, because `flutter_rust_bridge` will generate this trait implementation for you.
To fix this error you should just rerun `flutter_rust_bridge_codegen`.


Generated Dart code:

```Dart
Stream<LogEntry> createLogStream();
```

Now let us use it in Dart:

```dart
Future<void> setup() async {
    createLogStream().listen((event) {
      print('log from rust: ${event.level} ${event.tag} ${event.msg} ${event.timeMillis}');
    });
}
```

And now we can happily log anything in Rust:

```rust
log_stream_sink.add(LogEntry { msg: "hello I am a log from Rust", ... })
```

Of course, you can implement a logger following the Rust's `log` crate wrapping this raw stream sink, then you can use standard Rust logging mechanisms like `info!`. I did exactly that in my project.
