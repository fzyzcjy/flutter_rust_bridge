// ignore_for_file: avoid_print

import 'dart:async';

import 'package:logging/logging.dart';

/// A normalized Rust log record received through flutter_rust_bridge.
class FrbLogRecordData {
  /// The Rust `log` level name, for example `INFO` or `WARN`.
  final String level;

  /// The formatted log message.
  final String message;

  /// The Rust log target.
  final String target;

  /// The Rust module path, when available.
  final String? modulePath;

  /// The Rust source file, when available.
  final String? file;

  /// The Rust source line, when available.
  final int? line;

  /// Creates a normalized log record for Dart-side processing.
  const FrbLogRecordData({
    required this.level,
    required this.message,
    required this.target,
    required this.modulePath,
    required this.file,
    required this.line,
  });
}

/// The Dart side of the Rust `log` to Dart `logging` bridge.
final kFrbDartLogging = FrbDartLogging._();

class _FrbDartLoggingState {
  final StreamSubscription<Object?> subscription;
  final StreamSubscription<LogRecord>? defaultOutputSubscription;
  final void Function()? disposeRustLogger;

  _FrbDartLoggingState({
    required this.subscription,
    required this.defaultOutputSubscription,
    required this.disposeRustLogger,
  });
}

/// Installs the Dart side of the Rust `log` to Dart `logging` bridge.
class FrbDartLogging {
  FrbDartLogging._();

  _FrbDartLoggingState? _state;

  /// Connects a generated Rust log stream to the Dart `logging` package.
  void init<T>({
    required Stream<T> rustLogStream,
    required FrbLogRecordData Function(T record) mapRecord,
    bool setupDefaultOutput = true,
    void Function()? disposeRustLogger,
  }) {
    final previousState = _state;

    var defaultOutputSubscription = previousState?.defaultOutputSubscription;
    if (setupDefaultOutput && defaultOutputSubscription == null) {
      defaultOutputSubscription = _createDefaultOutputSubscription();
    }

    final subscription = rustLogStream.listen(
      (record) {
        final mapped = mapRecord(record);
        Logger(mapped.target).log(_toDartLevel(mapped.level), mapped.message);
      },
      onError: (Object error, StackTrace stackTrace) {
        Logger(
          'flutter_rust_bridge.logging',
        ).severe('Error in Rust log stream', error, stackTrace);
      },
    );

    _state = _FrbDartLoggingState(
      subscription: subscription,
      defaultOutputSubscription: defaultOutputSubscription,
      disposeRustLogger: disposeRustLogger,
    );
    if (previousState != null) {
      unawaited(previousState.subscription.cancel());
    }
  }

  /// Disconnects the Rust log stream listener.
  void dispose() {
    final state = _state;
    _state = null;
    if (state == null) {
      return;
    }

    state.disposeRustLogger?.call();
    unawaited(state.subscription.cancel());
    if (state.defaultOutputSubscription != null) {
      unawaited(state.defaultOutputSubscription!.cancel());
    }
  }

  StreamSubscription<LogRecord>
  _createDefaultOutputSubscription() => Logger.root.onRecord.listen((record) {
    final loggerName = record.loggerName.isEmpty ? 'root' : record.loggerName;
    print(
      '${record.level.name}: ${record.time}: $loggerName: ${record.message}',
    );
  });

  Level _toDartLevel(String level) {
    switch (level.toUpperCase()) {
      case 'TRACE':
        return Level.FINER;
      case 'DEBUG':
        return Level.FINE;
      case 'INFO':
        return Level.INFO;
      case 'WARN':
      case 'WARNING':
        return Level.WARNING;
      case 'ERROR':
        return Level.SEVERE;
      default:
        return Level.INFO;
    }
  }
}
