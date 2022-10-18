export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:flutter_rust_bridge_benchmark/bridge_generated.dart';

const root = 'pkg/flutter_rust_bridge_example_benchmark_suite';

/// initialize WASM built from Rust dynamic library (on web platform)
FlutterRustBridgeExampleBenchmarkSuiteImpl initializeBenchExternalLibrary(void _) =>
    FlutterRustBridgeExampleBenchmarkSuiteImpl.wasm(
      WasmModule.initialize(kind: const Modules.noModules(root: root)),
    );
