import 'dart:async';
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
Future<void> initializeWasmModule({required String root}) async {
  _ensureCrossOriginIsolated();

  final script = web.HTMLScriptElement()..src = '$root.js';
  web.document.head!.append(script);

  await script.onLoad.first;

  jsEval('window.wasm_bindgen = wasm_bindgen');

  await _jsWasmBindgen('${root}_bg.wasm').toDart;
}

@JS('wasm_bindgen')
external JSPromise _jsWasmBindgen(String path);

void _ensureCrossOriginIsolated() {
  switch (crossOriginIsolated) {
    case false:
      throw const MissingHeaderException();
    case true:
      return;
    case null:
      web.console.warn(
          'Warning: crossOriginIsolated is null, browser might not support buffer sharing.'
              .toJS);
      return;
  }
}
