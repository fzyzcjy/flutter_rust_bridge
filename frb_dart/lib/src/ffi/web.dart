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


@JS("wasm_bindgen.drop_opaque_box")
external void dropOpaqueBox(int ptr, int dropPtr, int lendPtr);
@JS("wasm_bindgen.lend_opaque_box")
external int lendOpaqueBox(int ptr, int lendPtr);


/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at some point during runtime.
class FrbOpaque {
  int _ptr = 0;
  late int _drop;
  late int _lend;
  static int counter = 0;
  static final Finalizer<List<int>> _finalizer =
      Finalizer((obj) {
        dropOpaqueBox(obj[0], obj[1], obj[2]);});


  /// This constructor should never be called manually.
  FrbOpaque.unsafe(int? ptr, int drop, int lend)
  {
    if (ptr != null) {
    _ptr = ptr;
    _finalizer.attach(this, [_ptr, drop, lend], detach: this);
    }
    _drop = drop;
    _lend = lend;
  }

  /// The native finalizer runs [_drop] on [_ptr]
  /// if the object is garbage collected.
  // late final NativeFinalizer _finalizer;

  /// Call Rust destructors on the backing memory of this pointer.
  /// This function should be run at least once during the lifetime of the program, 
  /// and can be run many times.
  ///
  /// When passed into a Rust function, 
  /// Rust enacts *shared ownership* and inhibits disposal of this pointer's contents, 
  /// even if [dispose] is immediately run.
  /// Furthermore, if that same function reuses the allocation 
  /// (usually by returning the same opaque pointer) 
  /// ownership of this pointer will be moved into that new opaque pointer.
  void dispose() {
    if (!isStale()) {
      _finalizer.detach(_ptr);
      dropOpaqueBox(_ptr, _drop, _lend);
      _ptr = 0;
    }
  }

  static dynamic lend(FrbOpaque ptr) {
    if (!ptr.isStale()) {
      return lendOpaqueBox(ptr._ptr, ptr._lend);
    } else {
      return 0;
    }
  }

  /// Checks whether [dispose] has been called at any point during the lifetime
  /// of this pointer. This does not guarantee that the backing memory has actually
  /// been reclaimed.
  // not nullptr, this is an internal bookkeeping method
  bool isStale() => _ptr == 0;
}
