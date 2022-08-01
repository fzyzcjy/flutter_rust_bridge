export 'ffi/ffi_io.dart' if (dart.library.html) 'ffi/ffi_web.dart';

/// A JS function that returns a Promise to a WASM module.
///
/// Example:
/// ```dart
/// // Assume that there exists a pkg/my-module.js in the web/ directory
/// // that was built by "wasm-pack -t no-modules"
/// import 'dart:html';
/// import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
///
/// @JS()
/// external WasmModule get wasm_bindgen;
///
/// Future<WasmModule> _initModule() {
///   // Some essential web features require cross-origin isolation.
///   if (crossOriginIsolated != true) return Future.error(MissingHeaderException());
///
///   final script = ScriptElement()..src = 'pkg/my-module.js';
///   document.head!.append(script);
///   return script.onLoad.first.then((_) {
///     // bring the wasm_bindgen variable into the global window scope
///     eval("window.wasm_bindgen = wasm_bindgen");
///     return wasm_bindgen;
///   });
/// }
///
/// final MyModule api = MyModule(_initModule());
/// ```
///
/// ## Enabling cross-origin isolation
/// Rust WASM modules do not work without cross-origin isolation. Here is a sample
/// web server implemented in Dart that supplies the correct response headers:
/// ```dart
/// // scripts/server.dart
/// import 'package:shelf/shelf.dart';
/// import 'package:shelf/shelf_io.dart' as io;
///
/// const host = 'localhost';
/// const port = 3000;
/// const addr = 'http://$host:$port';
///
/// void main() async {
///   final pipeline = const Pipeline().addMiddleware((handler) {
///     return (req) async {
///       final res = await handler(req);
///       return res.change(headers: {
///         'Cross-Origin-Opener-Policy': 'same-origin',
///         'Cross-Origin-Embedder-Policy': 'credentialless',
///       });
///     };
///   }).addHandler(createStaticHandler(
///     'build/web', // path to the output of "flutter build web"
///     defaultDocument: 'index.html',
///   ));
///   await io.serve(pipeline, host, port);
///   print('🎉 Serving at $addr');
/// }
/// ```
typedef WasmModule = Object Function([String? moduleName]);
