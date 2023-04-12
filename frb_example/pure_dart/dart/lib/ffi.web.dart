import 'bridge_definitions.dart';
import 'bridge_generated.web.dart';
export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const root = 'pkg/flutter_rust_bridge_example_pure_dart';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(void _) =>
    FlutterRustBridgeExampleSingleBlockTestImpl.wasm(
      WasmModule.initialize(kind: const Modules.noModules(root: root))
          as ExternalLibrary,
    );

FlutterRustBridgeExampleSingleBlockTest? _api;
FlutterRustBridgeExampleSingleBlockTest get api => _api ?? (throw Exception('API not yet initialized'));
