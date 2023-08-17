import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
export 'package:js/js.dart';
export 'package:js/js_util.dart' show promiseToFuture, getProperty;

abstract class WasmModule {
  Object call([String? moduleName]);

  /// Create a new WASM module initializer that is bound to the specified binary.
  WasmModule bind(dynamic thisArg, String moduleName);

  static Future<T> cast<T extends WasmModule>(FutureOr<WasmModule> module) {
    return Future.value(module).then((module) => module as T);
  }

  static FutureOr<WasmModule> initialize(
          {required Modules kind, WasmModule Function()? module}) =>
      kind.initializeModule(module);
}

abstract class Modules {
  const Modules();

  const factory Modules.noModules({required String root}) =
      _WasmBindgenNoModules;

  FutureOr<WasmModule> initializeModule(WasmModule Function()? module);

  void _ensureCrossOriginIsolated() {
    switch (crossOriginIsolated) {
      case false:
        throw const MissingHeaderException();
      case true:
      case null:
        warn(
            'Warning: crossOriginIsolated is null, browser might not support buffer sharing.');
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
      eval('window.wasm_bindgen = wasm_bindgen');
      final module_ = module?.call() ?? _noModules!;
      return module_.bind(null, '${root}_bg.wasm');
    });
  }
}

typedef ExternalLibrary = FutureOr<WasmModule>;
typedef DartPostCObject = void;

@JS()
external bool? get crossOriginIsolated;

@JS('console.warn')
external void warn([a, b, c, d, e, f, g, h, i]);

@JS('Number')
external int castInt(Object? value);

@JS('BigInt')
external Object castNativeBigInt(Object? value);

@JS('Function')
class _Function {
  external dynamic call();
  external factory _Function(String script);
}

@JS('wasm_bindgen')
external WasmModule? get _noModules;

dynamic eval(String script) => _Function(script)();

abstract class DartApiDl {}

@JS("wasm_bindgen.get_dart_object")
// ignore: non_constant_identifier_names
external Object getDartObject(int ptr);
@JS("wasm_bindgen.drop_dart_object")
// ignore: non_constant_identifier_names
external void dropDartObject(int ptr);

abstract class FlutterRustBridgeWireBase {
  void storeDartPostCObject() {}
  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn raw) {}

  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    return getDartObject(ptr);
  }

  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    dropDartObject(ptr);
  }

  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj, NativePortType port) {
    throw UnimplementedError();
  }
}

typedef WireSyncReturn = List<dynamic>;

List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => syncReturn;

class FlutterRustBridgeWasmWireBase<T extends WasmModule>
    extends FlutterRustBridgeWireBase {
  final Future<T> init;

  FlutterRustBridgeWasmWireBase(FutureOr<T> module)
      : init = Future.value(module).then((module) => promiseToFuture(module()));
}

typedef PlatformPointer = int;
typedef OpaqueTypeFinalizer = Finalizer<PlatformPointer>;

/// An opaque pointer to a Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
class FrbOpaqueBase {
  static PlatformPointer initPtr(int ptr) => ptr;
  static PlatformPointer nullPtr() => 0;
  static bool isStalePtr(PlatformPointer ptr) => ptr == 0;
  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr, int _,
          OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque);
}
