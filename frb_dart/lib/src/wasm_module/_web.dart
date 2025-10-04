import 'dart:async';
import 'dart:js_interop';
import 'dart:js_interop_unsafe';

import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
Future<void> initializeWasmModule({
  required String root,
  String wasmBindgenName = 'wasm_bindgen',
}) async {
  _ensureCrossOriginIsolated();

  final script = web.HTMLScriptElement()..src = '$root.js';
  web.document.head!.append(script);

  await script.onLoad.first;

  jsEval('window.$wasmBindgenName = $wasmBindgenName');

  final jsObject = web.window.getProperty(wasmBindgenName.toJS) as JSObject;
  final wasmBindgen = _JSWasmBindgen(jsObject);
  await wasmBindgen({"module_or_path": '${root}_bg.wasm'}.jsify()).toDart;
}

void _ensureCrossOriginIsolated() {
  switch (crossOriginIsolated) {
    case false:
      web.console.warn(
        'Warning: Buffers cannot be shared due to missing cross-origin headers. Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/manual/miscellaneous/web-cross-origin for details.'
            .toJS,
      );
      return;
    case true:
      return;
    case null:
      web.console.warn(
        'Warning: crossOriginIsolated is null, browser might not support buffer sharing.'
            .toJS,
      );
      return;
  }
}

extension type _JSWasmBindgen(JSObject _) implements JSObject {
  @JS()
  external JSPromise call(JSAny? arg);
}
