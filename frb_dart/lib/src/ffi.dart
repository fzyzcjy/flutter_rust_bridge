import 'dart:async';
export 'ffi/ffi_io.dart' if (dart.library.html) 'ffi/ffi_web.dart';

/// A JS function that returns a Promise to a WASM module.
/// See [this file](https://github.com/fzyzcjy/flutter_rust_bridge/blob/ffc9c2f530daa72ebd2f77e45e67b4fa7a65c172/frb_example/pure_dart/dart/lib/ffi.web.dart)
/// for an example of how to obtain and initialize this type.
///
/// ## Enabling cross-origin isolation
/// Rust WASM modules do not work without cross-origin isolation.
/// Please refer to [Setting up the web server](http://cjycode.com/flutter_rust_bridge/build_wasm.html#setting-up-the-web-server)
/// for an example of a Dart web server that accomplishes this task.
abstract class WasmModule {
  Object call([String? moduleName]);

  /// Create a new WASM module initializer that is bound to the specified binary.
  WasmModule bind(dynamic thisArg, String moduleName);

  static Future<T> cast<T extends WasmModule>(FutureOr<WasmModule> module) {
    return Future.value(module).then((module) => module as T);
  }
}
