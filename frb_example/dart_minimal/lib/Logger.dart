import 'package:frb_example_dart_minimal/src/rust/api/log_2_dart.dart';
import 'package:logging/logging.dart';

//TODO add to frb-Runtime

void init_logger(
    {Level maxLoglevel = Level.INFO, Function(LogRecord) log = default_log}) {
  Logger.root.level = maxLoglevel;
  var stream = initializeLog2Dart(maxLogLevel: maxLoglevel);
  // logs from Rust
  stream.listen((record) {
    log(record.toLogRecord());
  });

  // logs from Dart
  Logger.root.onRecord.listen((record) {
    log(record);
  });

  Logger.root.onLevelChanged.listen((newLoglevel) {
    changeLogLevel(newLogLevel: newLoglevel!);
  });
}

void default_log(LogRecord record) {
  print('${record.level}:${record.loggerName}: ${record.message}');
}

// convert from log crate's LevelFilter to Dart package logging->Level
Level fromLevelFilter(int level) {
  switch (level) {
    case <= 500:
      return Level.ALL;
    case <= 700:
      return Level.CONFIG;
    case <= 800:
      return Level.INFO;
    case <= 900:
      return Level.WARNING;
    case < 2000:
      return Level.SEVERE;
    case >= 2000:
      return Level.OFF;
    default:
      return Level.ALL;
  }
}

// convert from log crate's Record to Dart package logging->LogRecord
extension ToLogRecord on Log2DartLogRecord {
  LogRecord toLogRecord() {
    return LogRecord(
      this.level,
      this.message,
      this.loggerName,
    );
  }
}
