import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';
import 'package:js/js.dart';

/// A JS function that returns a Promise to a WASM module.
/// See [this file](https://github.com/fzyzcjy/flutter_rust_bridge/blob/ffc9c2f530daa72ebd2f77e45e67b4fa7a65c172/frb_example/pure_dart/dart/lib/ffi.web.dart)
/// for an example of how to obtain and initialize this type.
///
/// ## Enabling cross-origin isolation
/// Rust WASM modules do not work without cross-origin isolation.
/// Please refer to [Setting up the web server](http://cjycode.com/flutter_rust_bridge/build_wasm.html#setting-up-the-web-server)
/// for an example of a Dart web server that accomplishes this task.
abstract class WasmModule {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object call([String? moduleName]);

  /// Create a new WASM module initializer that is bound to the specified binary.
  WasmModule bind(dynamic thisArg, String moduleName);

  /// Cast the module into type `T`
  static Future<T> cast<T extends WasmModule>(FutureOr<WasmModule> module) {
    return Future.value(module).then((module) => module as T);
  }

  /// Initialize a [WasmModule] with the specified kind of [Modules].
  static FutureOr<WasmModule> initialize({required Modules kind, WasmModule Function()? module}) =>
      kind.initializeModule(module);
}

/// Currently supported modes of module initialization.
///
/// Advanced users may wish to inherit this class and override [initializeModule]
/// to provide their own initialization process.
abstract class Modules {
  /// Construct modules
  const Modules();

  /// Initialize a `wasm_bindgen` module built with the `-t no-modules` flag.
  ///
  /// The expected output is a file named `$root.js` and the accompanying
  /// WASM binary named `${root}_bg.wasm`.
  const factory Modules.noModules({required String root}) = _WasmBindgenNoModules;

  /// How a WASM module is brought into Dart's scope and initialized.
  ///
  /// Override this method to define custom initialization processes.
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module);

  void _ensureCrossOriginIsolated() {
    switch (crossOriginIsolated) {
      case false:
        throw const MissingHeaderException();
      case true:
      case null:
        jsConsoleWarn('Warning: crossOriginIsolated is null, browser might not support buffer sharing.');
        return;
    }
  }
}

class _WasmBindgenNoModules extends Modules {
  final String root;

  const _WasmBindgenNoModules({required this.root});

  @override
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module) {
    _ensureCrossOriginIsolated();
    final script = ScriptElement()..src = '$root.js';
    document.head!.append(script);
    return script.onLoad.first.then((_) {
      jsEval('window.wasm_bindgen = wasm_bindgen');
      final module_ = module?.call() ?? _noModules!;
      return module_.bind(null, '${root}_bg.wasm');
    });
  }
}

@JS('wasm_bindgen')
external WasmModule? get _noModules;
