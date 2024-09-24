import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';

// TODO hid these calls
// final hideme2 = FRBLogger.fromLevelFilter();

// final LOGGER = FRBLogger.init_logger();
final LOGGER = FRBLogger.init_logger(
    custom_log_function: (record) => print(record.message));
// final LOGGER = FRBLogger.init_logger(maxLoglevel: Level.SHOUT);

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  LOGGER.info(
      'Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
