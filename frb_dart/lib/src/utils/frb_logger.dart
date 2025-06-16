// ignore_for_file: avoid_print

import 'dart:io';
// ignore: depend_on_referenced_packages
import 'package:logging/logging.dart';

/// Call Log functions from the Dart side
/// to be instantiated with FRBLogger.initLogger()
/// NOT initAndGetSingleton directly!
class FRBDartLogger<MirLogRecord> {
  /// stream to receive log records from rust code
  final Stream<MirLogRecord> streamSink;

  /// default log function
  final Function({required MirLogRecord record}) logFn;

  /// function to convert intermediate Log records to Dart LogRecords
  final MirLogRecord Function(LogRecord record) fromDartLogRecord;

  // Private constructor to enforce singleton pattern and ensure initialization
  const FRBDartLogger._({
    required this.streamSink,
    required this.logFn,
    required this.fromDartLogRecord,
  });

  // This static field is typed as FRBDartLogger<dynamic> because `frb_logger.dart` cannot
  // know the concrete MirLogRecord type from the generated code (in 'frb_rust/misc/logging.rs') at compile time.
  static FRBDartLogger<dynamic>? _singleton;
  static late String _currentLoggerName;

  /// Initializes the logging system, including the Rust logger.
  ///
  /// This method is designed to be called **once** from the generated dart file.
  /// It takes the concrete `MirLogRecord` type's parameters as `dynamic` arguments,
  /// then creates and stores the strongly-typed singleton.
  static FRBDartLogger<M> initAndGetSingleton<M>(
      {required Stream<M> streamSink,
      required void Function({required dynamic record}) logFn,
      required M Function(LogRecord record) fromDartLogRecord,
      String name = 'FRBLogger',
      String maxLogLevel = 'INFO',
      void Function({required dynamic record})? customLogFunction}) {
    if (_singleton != null) {
      throw Exception('Called FRBLogger initialisation twice');
    }

    // Assign the singleton directly using a private constructor.
    // The type `dynamic` acts as a placeholder here, but the actual functions
    // passed in `streamSink`, `logFn`, `fromDartLogRecord` will be correctly typed
    // from the calling side (generated dart code).
    _singleton = FRBDartLogger<M>._(
      streamSink: streamSink,
      logFn: logFn,
      fromDartLogRecord: fromDartLogRecord,
    );

    Logger(name);
    _currentLoggerName = name; // Accessing static field directly

    String? envLogLevel = Platform.environment['LOG_LEVEL'];
    if (envLogLevel != null) {
      print(
          'Taking log level from env: $envLogLevel instead of the one given by code: $maxLogLevel');
      maxLogLevel = envLogLevel;
    }
    Logger.root.level = logLevelFromStr(maxLogLevel);

    final Function({required dynamic record}) usedlogFn =
        customLogFunction ?? _singleton!.logFn;

    // logs from Rust
    _singleton!.streamSink.listen((record) {
      usedlogFn(record: record);
    });

    // logs from Dart
    Logger.root.onRecord.listen((record) {
      usedlogFn(record: _singleton!.fromDartLogRecord(record));
    });

    return _singleton! as FRBDartLogger<M>; // Return the static _singleton
  }

  /// Gets the initialized singleton logger.
  /// It returns `FRBDartLogger<dynamic>` as `frb_logger.dart` doesn't know the concrete type.
  static FRBDartLogger<dynamic> getLogger([String? name]) {
    if (_singleton != null) {
      throw Exception("You have to call FRBLogger.initLogger() first!");
    }
    var loggerName = name ?? _currentLoggerName;
    Logger(loggerName);
    _currentLoggerName = loggerName;
    return _singleton!;
  }

  // These logging methods use the MirLogRecord type from the instance they are called on.
  /// trace level logging output
  trace(String message) {
    Logger(_currentLoggerName).log(Level.FINE, message);
  }

  /// debug level logging output
  debug(String message) {
    Logger(_currentLoggerName).log(Level.CONFIG, message);
  }

  /// info level logging output
  info(String message) {
    Logger(_currentLoggerName).log(Level.INFO, message);
  }

  /// warn level logging output
  warn(String message) {
    Logger(_currentLoggerName).log(Level.WARNING, message);
  }

  /// error level logging output
  error(String message) {
    Logger(_currentLoggerName).log(Level.SHOUT, message);
  }

  /// convert to Log Level from a String
  static Level logLevelFromStr(String levelStr) {
    switch (levelStr.toUpperCase()) {
      case 'ALL':
        return Level.ALL;
      case 'FINEST':
        return Level.FINEST;
      case 'FINER':
        return Level.FINER;
      case 'FINE':
        return Level.FINE;
      case 'CONFIG':
        return Level.CONFIG;
      case 'INFO':
        return Level.INFO;
      case 'WARNING':
        return Level.WARNING;
      case 'SEVERE':
        return Level.SEVERE;
      case 'SHOUT':
        return Level.SHOUT;
      case 'OFF':
        return Level.OFF;
      default:
        print(
            'unknown LOG_LEVEL: $levelStr. For potential values see https://pub.dev/documentation/logging/latest/logging/Level-class.html');
        exit(1);
    }
  }

  /// convert from log level number to Dart package logging->Level
  static Level logLevelFromNumber(int level) {
    switch (level) {
      case < 300:
        return Level.ALL;
      case < 400:
        return Level.FINEST;
      case < 500:
        return Level.FINER;
      case < 700:
        return Level.FINE;
      case < 800:
        return Level.CONFIG;
      case < 900:
        return Level.INFO;
      case < 1000:
        return Level.WARNING;
      case < 1200:
        return Level.SEVERE;
      case < 2000:
        return Level.SHOUT;
      case >= 2000:
        return Level.OFF;
      default:
        return Level.ALL;
    }
  }

  @override
  int get hashCode => streamSink.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FRBDartLogger &&
          runtimeType == other.runtimeType &&
          streamSink == other.streamSink;
}
