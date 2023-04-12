import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.web.dart';

const root = 'pkg/flutter_rust_bridge_example';

final api = FlutterRustBridgeExampleImpl(
  WasmModule.initialize(
    kind: const Modules.noModules(root: root),
  ) as ExternalLibrary,
);
