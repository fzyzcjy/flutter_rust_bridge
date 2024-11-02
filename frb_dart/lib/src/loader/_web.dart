import 'dart:async';

import 'package:flutter_rust_bridge/src/loader/_common.dart';
import 'package:flutter_rust_bridge/src/main_components/entrypoint.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// See `loadExternalLibrary` in the counterpart `_io.dart` for doc
FutureOr<ExternalLibrary> loadExternalLibrary(
    BaseEntrypoint entrypoint, ExternalLibraryLoaderConfig config) async {
  var hex = entrypoint.rustContentHash.toRadixString(16);
  return loadExternalLibraryRaw(
      moduleRoot: '${config.webPrefix}${config.stem}${hex}');
}

/// Please see `loadExternalLibrary` for details
Future<ExternalLibrary> loadExternalLibraryRaw(
    {required String moduleRoot}) async {
  await initializeWasmModule(root: moduleRoot);
  return ExternalLibrary(debugInfo: 'moduleRoot=$moduleRoot');
}
