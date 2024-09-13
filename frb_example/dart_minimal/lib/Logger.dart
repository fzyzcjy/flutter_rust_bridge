import 'package:logging/logging.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';

void init_logger() {
  Logger.root.level = Level.ALL;
  Logger.root.onRecord.listen((record) {
    log_callback(record);
  });
}

void log_callback(LogRecord record) {
  // print('${record.level.name}: ${record.time}: ${record.message}');
}
