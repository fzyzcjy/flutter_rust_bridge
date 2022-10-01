import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'interceptor.io.dart' if (dart.library.html) 'interceptor.web.dart';
import 'bridge_generated.io.dart'
    if (dart.library.html) 'bridge_generated.web.dart';

class FlutterRustBridgeExampleBenchmarkSuiteImplBench
    extends FlutterRustBridgeExampleBenchmarkSuiteImpl {
  // ignore: unused_field
  final FlutterRustBridgeInterceptor _interceptor =
      FlutterRustBridgeInterceptor();
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench(
          ExternalLibrary dylib) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(
          FlutterRustBridgeExampleBenchmarkSuitePlatform(
        dylib,
      ));

  /// Only valid on web/WASM platforms.
  factory FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(
          FutureOr<WasmModule> module) =>
      FlutterRustBridgeExampleBenchmarkSuiteImplBench(
          module as ExternalLibrary);
  @override
  FlutterRustBridgeExampleBenchmarkSuiteImplBench.raw(
      FlutterRustBridgeExampleBenchmarkSuitePlatform platform)
      : super.raw(platform);

  FlutterRustBridgeInterceptor get interceptor {
    return _interceptor;
  }
}
