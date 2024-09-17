//todo add this to the Runtime/frb_generated
import 'package:frb_example_dart_logging/src/rust/api/logging_example.dart';
import 'package:frb_example_dart_logging/src/rust/frb_generated.dart';
import 'package:frb_example_dart_logging/Logger.dart';

import 'package:logging/logging.dart';

// If you are developing a binary program, you may want to put it in `bin/something.dart`

// Initialize the logger as usual with the dart package `logging`
final LOGGER = Logger('frb_logger');
Future<void> main() async {
  await RustLib.init();
  init_logger();

  LOGGER.info('The default log level is info!');
  // change the log level like this:
  // note that this is done on the package class `Logger`, not the instance `LOGGER`
  Logger.root.level = Level.WARNING;
  LOGGER.info("this is not logged");
  LOGGER.warning("this is logged");

  // set you ow function to output logs
  LOGGER.setLogFunction((LogRecord record) => {
        print(
            "My own logging function - used for rust logs as well! ${record.message}")
      });
  LOGGER.shout("Logged with custom log funtion from above!");
  LOGGER.warning(
      'from Dart: Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
