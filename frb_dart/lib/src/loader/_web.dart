import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
ExternalLibrary loadExternalLibrary({
  String? defaultExternalLibraryRelativeDirectory,
  required String stem,
}) {
  // TODO what about `pkg`
  final root = 'pkg/$stem';
  return WasmModule.initialize(kind: Modules.noModules(root: root));
}
