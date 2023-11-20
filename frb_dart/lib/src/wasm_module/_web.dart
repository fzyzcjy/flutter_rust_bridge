import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:flutter_rust_bridge/src/utils/web_utils.dart';
import 'package:js/js.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class WasmModule {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object call([String? moduleName]);

  /// Create a new WASM module initializer that is bound to the specified binary.
  WasmModule bind(dynamic thisArg, String moduleName);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  static Future<T> cast<T extends WasmModule>(FutureOr<WasmModule> module) {
    return Future.value(module).then((module) => module as T);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  static FutureOr<WasmModule> initialize({required Modules kind, WasmModule Function()? module}) =>
      kind.initializeModule(module);
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class Modules {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const Modules();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const factory Modules.noModules({required String root}) = _WasmBindgenNoModules;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
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
