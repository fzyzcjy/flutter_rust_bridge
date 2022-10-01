import 'bridge_generated.web.dart';
export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const root = 'pkg/flutter_rust_bridge_example';

FlutterRustBridgeExampleBenchmarkSuiteImpl initializeExternalLibrary(void _) =>
    FlutterRustBridgeExampleBenchmarkSuiteImpl.wasm(
      WasmModule.initialize(kind: const Modules.noModules(root: root)),
    );
