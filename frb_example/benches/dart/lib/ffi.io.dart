import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

import 'interceptor.dart';

// ignore: unused_import
import 'bridge_generated.io.dart';
export 'bridge_generated.io.dart';

FlutterRustBridgeExampleBenchmarkSuiteImplBench initializeBenchExternalLibrary(String path, {bool? useJSON}) {
  return FlutterRustBridgeExampleBenchmarkSuiteImplBench(
      (Platform.isMacOS || Platform.isIOS) && !isSilicon ? DynamicLibrary.executable() : DynamicLibrary.open(path),
      useJSON: useJSON ?? false);
}
