import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
ExternalLibrary loadExternalLibrary({
  String? defaultExternalLibraryRelativeDirectory,
  String? stem,
}) {
  print('loadExternalLibrary return dummy thing');
  return WasmModule.initialize(kind: const Modules.noModules(root: 'this-is-dummy-root'));
}
