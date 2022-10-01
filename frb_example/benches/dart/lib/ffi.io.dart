import 'dart:ffi';
import 'dart:io';

import 'interceptor.dart';

// ignore: unused_import
import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

FlutterRustBridgeExampleBenchmarkSuiteImplBench initializeExternalLibrary(
    String path,
    {bool? useJSON}) {
  return FlutterRustBridgeExampleBenchmarkSuiteImplBench(
      (Platform.isMacOS || Platform.isIOS) &&
              !const bool.fromEnvironment('SILICON', defaultValue: false)
          ? DynamicLibrary.executable()
          : DynamicLibrary.open(path),
      useJSON: useJSON ?? false);
}
