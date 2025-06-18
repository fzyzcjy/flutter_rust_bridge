// ignore_for_file: avoid_print

import 'dart:io';

import 'package:logging/logging.dart';

/// FRB normalized LogLevels
/// Keeps the more fine granular log levels of the logging.dart package while overwriting
/// the unusual ones with the log level names from the rust log crate.
///
/// This enum also handles the conversion between
/// its own levels and integer numbers to `logging.dart` package's `Level` enum.
enum LogLevel {
  /// Maps to `Level.ALL` and integer levels below 300.
  all(
    levelNumberThreshold: 300,
  ),

  /// Maps to `Level.FINEST` and integer levels below 400.
  finest(
    levelNumberThreshold: 400,
  ),

  /// Maps to `Level.FINER` and integer levels below 500.
  finer(
    levelNumberThreshold: 500,
  ),

  /// Maps to `Level.FINE` and integer levels below 700. This is typically used for Trace.
  trace(
    levelNumberThreshold: 700,
  ),

  /// Maps to `Level.CONFIG` and integer levels below 800. This is typically used for Debug.
  debug(
    levelNumberThreshold: 800,
  ),

  /// Maps to `Level.INFO` and integer levels below 900.
  info(
    levelNumberThreshold: 900,
  ),

  /// Maps to `Level.WARNING` and integer levels below 1000. This is typically used for Warn.
  warn(
    levelNumberThreshold: 1000,
  ),

  /// Maps to `Level.SEVERE` and integer levels below 1200. This is typically used for Error.
  error(
    levelNumberThreshold: 1200,
  ),

  /// Maps to `Level.SHOUT` and integer levels below 2000. This is typically used for Fatal/Shout.
  fatal(
    levelNumberThreshold: 2000,
  ),

  /// Maps to `Level.OFF` and integer levels equal to or above 2000.
  off(
    levelNumberThreshold: 2000, // Or any value that signifies 'Off'
  );

  /// threshold up to which number mapps to which level
  final int levelNumberThreshold;

  const LogLevel({
    required this.levelNumberThreshold,
  });

  /// Converts an integer level number to a [LogLevel] enum.
  /// The conversion is based on predefined thresholds and the enum's declaration order.
  static LogLevel fromNumber(int levelNumber) {
    // The first level whose threshold is greater than the given levelNumber is the correct one.
    // This assumes the enum values are declared in ascending order of their thresholds.
    for (final level in LogLevel.values) {
      // For 'Off', it's usually `>=` the threshold, while others are `<`.
      // Handle the 'Off' case explicitly if its threshold logic is different.
      if (level == LogLevel.off) {
        if (levelNumber >= level.levelNumberThreshold) {
          return LogLevel.off;
        }
      } else if (levelNumber < level.levelNumberThreshold) {
        return level;
      }
    }
    // If no specific level is matched (e.g., if levelNumber is very high but doesn't explicitly hit Off's threshold),
    // default to Off. This can happen if the last regular level's threshold is passed.
    return LogLevel.off;
  }

  /// Maps a string log level name to a `logging.Level`.
  /// This is used for parsing initial configuration from strings (e.g., environment variables).
  static LogLevel fromString(String levelStr) {
    switch (levelStr.toUpperCase()) {
      case 'ALL':
        return LogLevel.all;
      case 'FINEST':
        return LogLevel.finest;
      case 'FINER':
        return LogLevel.finer;
      case 'TRACE':
        return LogLevel.trace;
      case 'DEBUG':
        return LogLevel.debug;
      case 'INFO':
        return LogLevel.info;
      case 'WARN':
        return LogLevel.warn;
      case 'ERROR':
        return LogLevel.error;
      case 'FATAL':
        return LogLevel.fatal;
      case 'OFF':
        return LogLevel.off;
      default:
        print(
            'Unknown LOG_LEVEL: "$levelStr". For potential values, refer to LogLevel enum definition.');
        exit(1); // Exit, as this is a critical configuration error
    }
  }

  /// converts a LogLevel to a logging package's level
  Level toLoggingLevel() {
    switch (this) {
      case LogLevel.all:
        return Level.ALL;
      case LogLevel.finest:
        return Level.FINEST;
      case LogLevel.finer:
        return Level.FINER;
      case LogLevel.trace:
        return Level.FINE;
      case LogLevel.debug:
        return Level.CONFIG;
      case LogLevel.info:
        return Level.INFO;
      case LogLevel.warn:
        return Level.WARNING;
      case LogLevel.error:
        return Level.SEVERE;
      case LogLevel.fatal:
        return Level.SHOUT;
      case LogLevel.off:
        return Level.OFF;
    }
  }

  /// converts a LogLevel to a logging package's level
  static LogLevel fromLoggingLevel(Level level) {
    switch (level) {
      case Level.ALL:
        return LogLevel.all;
      case Level.FINEST:
        return LogLevel.finest;
      case Level.FINER:
        return LogLevel.finer;
      case Level.FINE:
        return LogLevel.trace;
      case Level.CONFIG:
        return LogLevel.debug;
      case Level.INFO:
        return LogLevel.info;
      case Level.WARNING:
        return LogLevel.warn;
      case Level.SEVERE:
        return LogLevel.error;
      case Level.SHOUT:
        return LogLevel.fatal;
      case Level.OFF:
        return LogLevel.off;
      default:
        throw Exception("Unknown Dart logging level: $level");
    }
  }
}

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
  static FRBDartLogger<MirLogRecordType> initAndGetSingleton<MirLogRecordType>({
    required Stream<MirLogRecordType> streamSink,
    required void Function({required dynamic record}) logFn,
    required MirLogRecordType Function(LogRecord record) fromDartLogRecord,
    String name = 'FRBLogger',
    LogLevel maxLogLevel = LogLevel.info,
    void Function({required dynamic record})? customLogFunction,
  }) {
    if (_singleton != null) {
      throw Exception('Called FRBLogger.initLogger() twice!');
    }

    // Assign the singleton directly using a private constructor.
    // The type `dynamic` acts as a placeholder here, but the actual functions
    // passed in `streamSink`, `logFn`, `fromDartLogRecord` will be correctly typed
    // from the calling side (generated dart code).
    _singleton = FRBDartLogger<MirLogRecordType>._(
      streamSink: streamSink,
      logFn: logFn,
      fromDartLogRecord: fromDartLogRecord,
    );

    Logger(name);
    _currentLoggerName = name;

    String? envLogLevel = Platform.environment['LOG_LEVEL'];
    if (envLogLevel != null) {
      print(
          'Taking log level from env: $envLogLevel instead of the one given by code: $maxLogLevel');
      maxLogLevel = LogLevel.fromString(envLogLevel);
    }
    Logger.root.level = maxLogLevel.toLoggingLevel();

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

    return _singleton! as FRBDartLogger<MirLogRecordType>;
  }

  /// Gets the initialized singleton logger.
  /// It returns `FRBDartLogger<dynamic>` as `frb_logger.dart` doesn't know the concrete type.
  static FRBDartLogger<dynamic> getLogger([String? name]) {
    if (_singleton == null) {
      throw Exception("You have to call FRBLogger.initLogger() first!");
    }
    var loggerName = name ?? _currentLoggerName;
    Logger(loggerName);
    _currentLoggerName = loggerName;
    return _singleton!;
  }

  // These logging methods use the MirLogRecord type from the instance they are called on.
  /// finest level logging output
  finest(String message) {
    Logger(_currentLoggerName).log(LogLevel.finest.toLoggingLevel(), message);
  }

  /// finer level logging output
  finer(String message) {
    Logger(_currentLoggerName).log(LogLevel.finer.toLoggingLevel(), message);
  }

  /// trace level logging output
  trace(String message) {
    Logger(_currentLoggerName).log(LogLevel.trace.toLoggingLevel(), message);
  }

  /// debug level logging output
  debug(String message) {
    Logger(_currentLoggerName).log(LogLevel.debug.toLoggingLevel(), message);
  }

  /// info level logging output
  info(String message) {
    Logger(_currentLoggerName).log(LogLevel.info.toLoggingLevel(), message);
  }

  /// warn level logging output
  warn(String message) {
    Logger(_currentLoggerName).log(LogLevel.warn.toLoggingLevel(), message);
  }

  /// error level logging output
  error(String message) {
    Logger(_currentLoggerName).log(LogLevel.error.toLoggingLevel(), message);
  }

  /// fatal level logging output
  fatal(String message) {
    Logger(_currentLoggerName).log(LogLevel.fatal.toLoggingLevel(), message);
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
