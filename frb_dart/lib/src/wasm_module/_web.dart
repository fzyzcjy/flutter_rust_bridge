import 'dart:async';
import 'dart:js_interop';
import 'dart:js_interop_unsafe';

import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/wasm_bindgen_registration.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
Future<void> initializeWasmModule({
  required String root,
  String wasmBindgenName = 'wasm_bindgen',
}) async {
  _ensureCrossOriginIsolated();

  final script = web.HTMLScriptElement()..src = '$root.js';
  web.document.head!.append(script);

  final wasmBindgenCompleter = Completer<JSObject>();

  void listener(JSObject event) {
    final hasBindGen = event.has(wasmBindgenName);
    if (event is web.Event && hasBindGen) {
      event.stopPropagation();
      final wasmBindgen = event.getProperty(wasmBindgenName.toJS);
      wasmBindgenCompleter.complete(wasmBindgen as JSObject);
    }
  }

  web.window.addEventListener(wasmBindgenRegisteredEventName, listener.toJS);

  await script.onLoad.first;

  final jsObject = await wasmBindgenCompleter.future;

  web.window.setProperty(wasmBindgenName.toJS, jsObject);

  final wasmBindgen = _JSWasmBindgen(jsObject);
  await wasmBindgen('${root}_bg.wasm'.toJS).toDart;
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
