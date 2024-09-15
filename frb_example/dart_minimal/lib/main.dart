import 'package:frb_example_dart_minimal/Logger.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:logging/logging.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`

final LOGGER = Logger('frb_logger');
Future<void> main() async {
  print('\n\n Starting Dart\n');
  await RustLib.init();
  init_logger();
  LOGGER.info('\n\n Starting Dart - after logger init\n');
  LOGGER.info(
      'from Dart: Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
