import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'interceptor.dart';

// ignore: unused_import
import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

/// initialize Rust dynamic library (on native platforms)
FlutterRustBridgeExampleBenchmarkSuiteImplBench initializeBenchExternalLibrary(String path, {bool? useJSON}) {
  return FlutterRustBridgeExampleBenchmarkSuiteImplBench(
      (Platform.isMacOS || Platform.isIOS) && !useOpenDylib ? DynamicLibrary.executable() : DynamicLibrary.open(path),
      useJSON: useJSON ?? false);
}
