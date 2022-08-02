// ignore_for_file: avoid_web_libraries_in_flutter

import 'dart:html';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'bridge_generated.dart';

@JS()
external WasmModule get wasmBindgen;

Future<WasmModule> _initModule() {
  if (crossOriginIsolated != true) return Future.error(MissingHeaderException());

  final script = ScriptElement()..src = 'pkg/flutter_rust_bridge_example.js';
  document.head!.append(script);
  return script.onLoad.first.then((_) {
    eval("window.wasmBindgen = wasm_bindgen");
    return ([String? module]) => wasmBindgen(module ?? 'pkg/flutter_rust_bridge_example_bg.wasm');
  });
}

final api = FlutterRustBridgeExampleImpl(_initModule() as ExternalLibrary);
