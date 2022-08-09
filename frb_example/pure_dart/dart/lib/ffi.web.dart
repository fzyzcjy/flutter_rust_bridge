import 'dart:async';
import 'dart:html';
import 'bridge_generated.web.dart';
export 'bridge_generated.web.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

const root = 'pkg/flutter_rust_bridge_example_single_block_test';

FlutterRustBridgeExampleSingleBlockTestImpl initializeExternalLibrary(void _) {
  if (crossOriginIsolated != true) {
    return FlutterRustBridgeExampleSingleBlockTestImpl.wasm(
        Future.error(MissingHeaderException()));
  }

  final script = ScriptElement()..src = '$root.js';
  document.head!.append(script);
  final module = script.onLoad.first.then((_) {
    eval('window.wasm_bindgen = wasm_bindgen');
    return wasmModule.bind(null, '${root}_bg.wasm');
  });
  return FlutterRustBridgeExampleSingleBlockTestImpl.wasm(module);
}
