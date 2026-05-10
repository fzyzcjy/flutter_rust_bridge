# Logging

## Overview

flutter_rust_bridge provides three kinds of logging support:

- Builtin Rust console logging via `setup_default_user_utils()`.
- Builtin Rust-to-Dart logging via `enable_frb_rust_to_dart_logging!()`.
- Alternative examples for full control over platform loggers or streams.

## Case 1: Builtin Rust console logging

### Approach 1: Use the default one

If using the template by `flutter_rust_bridge_codegen create/integrate`, the "print logs to console" is configured by default,
via the auto-generated call to `flutter_rust_bridge::setup_default_user_utils()`.

Thus, you do not need to do anything :)

This path sends Rust logs to platform-native logging tools such as Android Logcat, Apple unified logging, or the browser console on wasm.

## Case 2: Builtin Rust-to-Dart logging

Use the Rust-to-Dart bridge when you want Rust logs to follow the same Dart-side pipeline as the rest of your Flutter app, for example when saving logs to a file, uploading them to a service, or using one Dart logging listener for both Dart and Rust code.

### Enable Rust-to-Dart logging

First, ensure your Rust crate depends on the standard Rust `log` crate:

```toml
[dependencies]
log = "0.4"
```

Then enable the bridge with one macro call in a Rust file covered by `rust_input`:

```rust
flutter_rust_bridge::enable_frb_rust_to_dart_logging!();
```

After code generation, `RustLib.init()` automatically initializes the generated log stream and connects it to Dart logging. By default, FRB also installs a simple Dart-side output listener, so items logged via the standard Rust calls like `log::info!` are visible in `flutter run`.

### Customize Dart output

The default output is meant for quick start. For an app-specific logging setup, disable it:

```rust
flutter_rust_bridge::enable_frb_rust_to_dart_logging!(
    setup_dart_logging_output = false
);
```

Then configure the standard Dart logging yourself before or after `RustLib.init()`:

```dart
import 'package:logging/logging.dart';

void setupLogging() {
  Logger.root.level = Level.ALL;
  Logger.root.onRecord.listen((record) {
    myLoggingService.write(
      level: record.level.name,
      loggerName: record.loggerName,
      message: record.message,
      time: record.time,
    );
  });
}
```

Rust `log::info!`, `log::warn!`, and similar calls will enter this Dart listener through FRB.

### Change the Rust max level

The bridge defaults to `log::LevelFilter::Info`. You can choose another Rust max level:

```rust
flutter_rust_bridge::enable_frb_rust_to_dart_logging!(
    max_level = log::LevelFilter::Debug
);
```

```rust
flutter_rust_bridge::enable_frb_rust_to_dart_logging!(
    max_level = log::LevelFilter::Trace,
    setup_dart_logging_output = false
);
```

## Case 3: Alternative examples

### Example 2: Print logs to console

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

### Example 3: My logger in production

In my own app in production, I use the following strategy for Rust logging: Use normal Rust logging methods, such as `info!` and `debug!` macros. The logs are consumed in two places: They are printed via platform-specific methods (like android Logcat and iOS NSLog), and also use a Stream to send them to the Dart side such that my Dart code and further process are using the same pipeline as normal Dart logs (e.g. save to a file, send to server, etc).

The *full* code related to logging in my app can be seen here: [#486](https://github.com/fzyzcjy/flutter_rust_bridge/issues/486).

### Example 4: Send Rust logs to Dart

@MnlPhlp encapsulates the step-by-step example below into a small Rust package,
such that you can setup Rust-logging-to-Dart in several lines.
Please refer to https://github.com/mnlphlp/flutter_logger for details.

### Example 5: A step-by-step guide to send Rust logs to Dart

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
