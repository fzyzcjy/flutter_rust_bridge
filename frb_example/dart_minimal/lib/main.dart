import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';

final LOGGER = FRBLogger.initLogger(maxLogLevel: LogLevel.trace
    // ,
    // customLogFunction: ({required record}) {
    //   final message =
    //       "${LogLevel.fromNumber(record.levelNumber)} ${record.rustLog ? "Rust: " : "Dart: "} LoggerName: ${record.loggerName} ModulePath: ${record.modulePath} FileName: ${record.fileName} line: ${record.lineNumber}\n${record.message}";
    //   print(
    //       "record.timestamp: ${record.timestamp}\n record.fileName: ${record.fileName}\n record.levelNumber: ${record.levelNumber}\n record.levelName: ${record.levelName}\n record.lineNumber: ${record.lineNumber}\n record.loggerName: ${record.loggerName}\n record.modulePath: ${record.modulePath}\n \n$message");
    // },
    );

// If you are developing a binary program, you may want to put it in `bin/something.dart`
Future<void> main() async {
  await RustLib.init();
  LOGGER.trace(
      'Call Rust and get: 100+200 = ${await minimalAdder(a: 100, b: 200)}');
}
