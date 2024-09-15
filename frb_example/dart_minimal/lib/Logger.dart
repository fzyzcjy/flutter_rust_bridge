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

  // logs from Dart
  //TODO use this as the default, but let the user overwrite it with his own logging function
  Logger.root.onRecord.listen((record) {
    log(record.message);
  });

  Logger.root.onLevelChanged.listen((level) {
    // TODO propergate the level change to rust
    print('The new log level is $level');
  });
}

void log(String msg) {
  print(
      //TODO log the record, not only msg
      '${msg}');
}
