import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

// ignore: unused_import
import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

/// initialize Rust dynamic library (on native platforms)
FlutterRustBridgeExampleBenchmarkSuiteImpl initializeBenchExternalLibrary(String path) {
  return FlutterRustBridgeExampleBenchmarkSuiteImpl(
    (Platform.isMacOS || Platform.isIOS) && !readBoolEnv('SILICON')
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(path),
  );
}
