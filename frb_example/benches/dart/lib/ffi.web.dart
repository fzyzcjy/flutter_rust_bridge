import 'interceptor.dart';

export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const root = 'pkg/flutter_rust_bridge_example_benchmark_suite';

/// initialize WASM built from Rust dynamic library (on web platform)
FlutterRustBridgeExampleBenchmarkSuiteImplBench initializeBenchExternalLibrary(void _, {bool? useJSON}) =>
    FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(
        WasmModule.initialize(kind: const Modules.noModules(root: root)),
        useJSON: useJSON ?? false);
