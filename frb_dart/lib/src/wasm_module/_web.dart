import 'dart:async';
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.internal}
Future<void> initializeWasmModule({required String root}) async {
  _ensureCrossOriginIsolated();
  final packageInfo = await PackageInfo.fromPlatform();
  final version = '${packageInfo.version}.${packageInfo.buildNumber}';

  final script = web.HTMLScriptElement()..src = '$root.js?version=$version';
  web.document.head!.append(script);

  await script.onLoad.first;

  jsEval('window.wasm_bindgen = wasm_bindgen');


  await _jsWasmBindgen('${root}_bg.wasm?version=$version').toDart;
}

@JS('wasm_bindgen')
external JSPromise _jsWasmBindgen(String path);

void _ensureCrossOriginIsolated() {
  switch (crossOriginIsolated) {
    case false:
      web.console.warn(
          'Warning: Buffers cannot be shared due to missing cross-origin headers. Please refer to https://fzyzcjy.github.io/flutter_rust_bridge/manual/miscellaneous/web-cross-origin for details.'
              .toJS);
      return;
    case true:
      return;
    case null:
      web.console.warn(
          'Warning: crossOriginIsolated is null, browser might not support buffer sharing.'
              .toJS);
      return;
  }
}
