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

/// Installs the Dart side of the Rust `log` to Dart `logging` bridge.
class FrbDartLogging {
  static StreamSubscription<Object?>? _subscription;
  static StreamSubscription<LogRecord>? _defaultOutputSubscription;

  /// Connects a generated Rust log stream to the Dart `logging` package.
  static void init<T>({
    required Stream<T> rustLogStream,
    required FrbLogRecordData Function(T record) mapRecord,
    bool setupDefaultOutput = true,
  }) {
    final previousSubscription = _subscription;

    _subscription = rustLogStream.listen(
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

    if (previousSubscription != null) {
      unawaited(previousSubscription.cancel());
    }

    if (setupDefaultOutput) {
      _setupDefaultOutput();
    }
  }

  /// Disconnects the Rust log stream listener.
  static Future<void> dispose() async {
    await _subscription?.cancel();
    _subscription = null;
  }

  static void _setupDefaultOutput() {
    _defaultOutputSubscription ??= Logger.root.onRecord.listen((record) {
      final loggerName = record.loggerName.isEmpty ? 'root' : record.loggerName;
      print(
        '${record.level.name}: ${record.time}: $loggerName: ${record.message}',
      );
    });
  }

  static Level _toDartLevel(String level) {
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
