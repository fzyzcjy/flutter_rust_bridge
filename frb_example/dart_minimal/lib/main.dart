import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
//todo add this to the Runtime/frb_generated
import 'Logger.dart';
import 'package:logging/logging.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`

final LOGGER = Logger('frb_logger');
Future<void> main() async {
  await RustLib.init();
  init_logger();

  var log = (LogRecord record) => {
        {
          print(
              'My own log function, logging ${record.level}: ${record.message}')
        }
      };

  LOGGER.setLogFunction(log);

  LOGGER.warning("this is custom logged!");

  LOGGER.setLogFunction(
      (LogRecord record) => {print("nochmal geändert! ${record.message}")});
  LOGGER.info("nochmal this is custom logged!");

  LOGGER.info(
      'from Dart: Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
