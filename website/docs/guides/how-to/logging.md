# Logging

flutter_rust_bridge provides two logging paths:

- `setup_default_user_utils()` configures platform-specific Rust console logging on supported platforms.
- `enable_frb_logging!()` bridges Rust `log` records into Dart's `logging` package.

Use the bridge when you want Rust logs to follow the same Dart-side pipeline as the rest of your Flutter app, for example when saving logs to a file, uploading them to a service, or using one Dart logging listener for both Dart and Rust code.

## Bridge Rust logs to Dart

First, ensure your Rust crate depends on the standard Rust `log` crate:

```toml
[dependencies]
flutter_rust_bridge = "..."
log = "0.4"
```

Then enable the bridge in a Rust file covered by `rust_input`:

```rust
flutter_rust_bridge::enable_frb_logging!();

pub fn compute() {
    log::info!("start compute");
    log::warn!("using fallback path");
}
```

After code generation, `RustLib.init()` automatically initializes the generated log stream and connects it to Dart logging. By default, FRB also installs a simple Dart-side output listener, so logs are visible in `flutter run`.

## Customize Dart output

The default output is meant for quick start. For an app-specific logging setup, disable it:

```rust
flutter_rust_bridge::enable_frb_logging!(
    setup_dart_logging_output = false
);
```

Then configure Dart logging yourself before or after `RustLib.init()`:

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

## Change the Rust max level

The bridge defaults to `log::LevelFilter::Info`. You can choose another Rust max level:

```rust
flutter_rust_bridge::enable_frb_logging!(
    max_level = log::LevelFilter::Debug
);
```

Both options can be combined:

```rust
flutter_rust_bridge::enable_frb_logging!(
    max_level = log::LevelFilter::Trace,
    setup_dart_logging_output = false
);
```

## Platform console logging

The older platform console setup is still available through `setup_default_user_utils()` and `setup_log_to_console(...)`. It sends Rust logs to platform tools such as Android Logcat, Apple unified logging, or the browser console on wasm.

That is useful when platform-native logs are exactly what you want. The Dart bridge is useful when you want Rust logs to be handled by Dart code.

## Fully manual streams

For unusual logging designs, you can still expose your own `StreamSink<T>` and send custom records manually:

```rust
use crate::frb_generated::StreamSink;

pub struct LogEntry {
    pub level: String,
    pub message: String,
}

pub fn create_log_stream(sink: StreamSink<LogEntry>) {
    sink.add(LogEntry {
        level: "INFO".to_owned(),
        message: "hello from Rust".to_owned(),
    }).ok();
}
```

The built-in bridge is just the common version of this pattern for Rust's standard `log` crate and Dart's standard `logging` package.
