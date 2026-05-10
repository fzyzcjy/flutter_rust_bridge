// ignore_for_file: avoid_print

import 'dart:async';

import 'package:logging/logging.dart';

/// A normalized Rust log record received through flutter_rust_bridge.
class FrbLogRecordData {
  final String level;
  final String message;
  final String target;
  final String? modulePath;
  final String? file;
  final int? line;

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

  static void init<T>({
    required Stream<T> rustLogStream,
    required FrbLogRecordData Function(T record) mapRecord,
    bool setupDefaultOutput = true,
  }) {
    if (_subscription != null) {
      throw StateError('FRB logging should not be initialized twice');
    }

    _subscription = rustLogStream.listen((record) {
      final mapped = mapRecord(record);
      Logger(mapped.target).log(
        _toDartLevel(mapped.level),
        mapped.message,
      );
    });

    if (setupDefaultOutput) {
      _setupDefaultOutput();
    }
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
        return Level.FINE;
      case 'DEBUG':
        return Level.CONFIG;
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
