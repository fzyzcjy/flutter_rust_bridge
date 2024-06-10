import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:js/js.dart';

/// {@macro flutter_rust_bridge.internal}
Future<void> initializeWasmModule({required String root}) async {
  _ensureCrossOriginIsolated();
  final script = ScriptElement()..src = '$root.js';
  document.head!.append(script);

  await script.onLoad.first;

  jsEval('window.wasm_bindgen = wasm_bindgen');

  await promiseToFuture(_noModules('${root}_bg.wasm'));
}

@JS('wasm_bindgen')
external dynamic get _noModules;

void _ensureCrossOriginIsolated() {
  switch (crossOriginIsolated) {
    case false:
      throw const MissingHeaderException();
    case true:
      return;
    case null:
      jsConsoleWarn(
          'Warning: crossOriginIsolated is null, browser might not support buffer sharing.');
      return;
  }
}
