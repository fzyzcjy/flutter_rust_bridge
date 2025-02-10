import 'dart:async';
import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:web/web.dart' as web;

import 'dart:ui_web';
import 'dart:convert';
import 'dart:html' as html;

/// {@macro flutter_rust_bridge.internal}
Future<void> initializeWasmModule({required String root}) async {
  _ensureCrossOriginIsolated();
  final packageInfo = await _getPackageInfo();
  final versionQuery = _assembleVersionQuery(packageInfo);
  
  final script = web.HTMLScriptElement()..src = '$root.js$versionQuery';
  web.document.head!.append(script);

  await script.onLoad.first;

  jsEval('window.wasm_bindgen = wasm_bindgen');

  await _jsWasmBindgen('${root}_bg.wasm$versionQuery').toDart;
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

/// Get the version number of the web app.
/// 
/// These functions follows the approach used in `package_info_plus` to fetch version information.
/// Since frb needs to be pure dart, so we cannot directly depend on `package_info_plus`.

String _assembleVersionQuery(Map<String, dynamic>? packageInfo) =>
    packageInfo != null && 
    packageInfo['version'] != null &&
    packageInfo['build_number'] != null ?
    '?version=${packageInfo['version']}.${packageInfo['build_number']}' :
    '';


Future<Map<String, dynamic>?> _getPackageInfo() async {
  String baseUrl = assetManager.getAssetUrl('').replaceAll('${assetManager.assetsDir}/', '');
  final resJson = 
    await _request(baseUrl) ??
    await _request(web.window.document.baseURI);
  if (resJson == null) return null;
  try {
    return jsonDecode(resJson);
  } catch (e) {
    return null;
  }
}

Future<String?> _request(String baseUrl) async {
    final req = await html.HttpRequest.request('$baseUrl/version.json?t=${DateTime.now().millisecondsSinceEpoch}');
    if (req.status == 200) {
      return req.responseText;
    }
    return null;
}