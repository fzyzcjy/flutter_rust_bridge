import 'package:frb_example_dart_minimal/src/rust/api/log_2_dart.dart';
import 'package:logging/logging.dart';

//TODO add to Runtime
void init_logger() {
  Logger.root.level = Level.ALL;
  var stream = initLog2Dart();
  // logs from Rust
  stream.listen((record) {
    log(record);
  });

  // logs frim Dart
  Logger.root.onRecord.listen((record) {
    log(record.message);
  });
}

void log(String msg) {
  print(
      // 'log in Dart: ${record.level.name}: ${record.time}: ${record.message}');
      'log in Dart: ${msg}');
}
