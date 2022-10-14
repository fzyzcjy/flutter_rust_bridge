import 'bridge_generated.web.dart';
export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const root = '/media/human/DE2466F72466D1D7/Work/Github/Test/flutter_rust_bridge/frb_example/pure_dart/rust/frb_example/pure_dart/rust/lib/flutter_rust_bridge_example';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(void _) =>
    FlutterRustBridgeExampleSingleBlockTestImpl.wasm(
      WasmModule.initialize(kind: const Modules.noModules(root: root)),
    );
