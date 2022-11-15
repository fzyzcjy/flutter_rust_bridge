import 'dart:async';
import 'dart:html';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:meta/meta.dart';
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

abstract class FlutterRustBridgeWireBase {
  void storeDartPostCObject() {}
  // ignore: non_constant_identifier_names
  void free_WireSyncReturnStruct(WireSyncReturnStruct raw) {}
}

typedef WireSyncReturnStruct = List<dynamic>;

extension WireSyncReturnStructExt on WireSyncReturnStruct {
  Uint8List get buffer => this[0];
  bool get isSuccess => this[1];
}

class FlutterRustBridgeWasmWireBase<T extends WasmModule>
    extends FlutterRustBridgeWireBase {
  final Future<T> init;

  FlutterRustBridgeWasmWireBase(FutureOr<T> module)
      : init = Future.value(module).then((module) => promiseToFuture(module()));
}

/// An opaque pointer to a Rust type.
///
/// Recipients of this type should call [dispose] at some point during runtime.
abstract class FrbOpaque {
  /// Pointer to this opaque Rust type.
  int _ptr;

  /// A native finalizer rust opaque type.
  /// Is static for each frb api class instance.
  Finalizer get staticFinalizer;

  /// Rust type specific drop function.
  ///
  /// This function should never be called manually.
  void Function(int) get dropFn;

  /// Rust type specific share function.
  ///
  /// This function should never be called manually.
  int Function(int) get shareFn;

  /// This constructor should never be called manually.
  @internal
  // ignore: no_leading_underscores_for_local_identifiers
  FrbOpaque.unsafe(this._ptr, int _size) {
    assert(_ptr > 0);
    staticFinalizer.attach(this, _ptr);
  }

  /// Call Rust destructors on the backing memory of this pointer.
  ///
  /// This function should be run at least once during the lifetime of the
  /// program, and can be run many times.
  ///
  /// When passed into a Rust function, Rust enacts *shared ownership*,
  /// if this pointer is shared with Rust when [dispose] is called,
  /// ownership is fully transferred to Rust else this pointer is cleared.
  void dispose() {
    if (!isStale()) {
      var ptr = _ptr;
      _ptr = 0;

      staticFinalizer.detach(this);
      dropFn(ptr);
    }
  }

  /// Increments inner reference counter and returns pointer to the underlying
  /// Rust object.
  ///
  /// Throws a [StateError] if called after [dispose].
  @internal
  int share() {
    if (!isStale()) {
      return shareFn(_ptr);
    } else {
      throw StateError('Use after dispose.');
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has
  /// actually been reclaimed.
  bool isStale() => _ptr == 0;
}
