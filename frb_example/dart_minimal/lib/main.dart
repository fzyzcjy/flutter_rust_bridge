import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';

final LOGGER = FRBLogger.initLogger(maxLogLevel: LogLevel.trace);

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  LOGGER.trace(
      'Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
