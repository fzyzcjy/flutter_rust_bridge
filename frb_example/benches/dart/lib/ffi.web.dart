import 'interceptor.dart';

export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const root = 'pkg/flutter_rust_bridge_example_benchmark_suite';

FlutterRustBridgeExampleBenchmarkSuiteImplBench initializeExternalLibrary(
        void _,
        {bool? useJSON}) =>
    FlutterRustBridgeExampleBenchmarkSuiteImplBench.wasm(
        WasmModule.initialize(kind: const Modules.noModules(root: root)),
        useJSON: useJSON ?? false);
