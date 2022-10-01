import 'dart:ffi';
import 'dart:io';

import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

FlutterRustBridgeExampleBenchmarkSuiteImpl initializeExternalLibrary(
    String path) {
  return FlutterRustBridgeExampleBenchmarkSuiteImpl(
    (Platform.isMacOS || Platform.isIOS) &&
            const bool.fromEnvironment('SILICON', defaultValue: false)
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path),
  );
}
