import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// See `loadExternalLibrary` in the counterpart `_io.dart` for doc
ExternalLibrary loadExternalLibrary(ExternalLibraryLoaderConfig config) {
  return loadExternalLibraryRaw(moduleRoot: '${config.webPrefix}${config.stem}');
}

/// Please see `loadExternalLibrary` for details
ExternalLibrary loadExternalLibraryRaw({required String moduleRoot}) {
  return ExternalLibrary(wasmModule: WasmModule.initialize(kind: Modules.noModules(root: moduleRoot)));
}
