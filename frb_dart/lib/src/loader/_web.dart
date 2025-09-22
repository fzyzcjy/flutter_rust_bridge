import 'dart:async';

import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// See `loadExternalLibrary` in the counterpart `_io.dart` for doc
FutureOr<ExternalLibrary> loadExternalLibrary(
    ExternalLibraryLoaderConfig config) async {
  return loadExternalLibraryRaw(
    moduleRoot: '${config.webPrefix}${config.stem}',
    wasmBindGenName: config.wasmBindGenName,
  );
}

/// Please see `loadExternalLibrary` for details
Future<ExternalLibrary> loadExternalLibraryRaw({
  required String moduleRoot,
  required String wasmBindGenName,
}) async {
  await initializeWasmModule(
    root: moduleRoot,
    wasmBindGenName: wasmBindGenName,
  );
  return ExternalLibrary(
    debugInfo: 'moduleRoot=$moduleRoot',
    wasmBindGenName: wasmBindGenName,
  );
}
