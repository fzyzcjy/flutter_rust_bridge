import 'dart:async';

import 'io.dart' if (dart.library.html) 'web.dart'
    show DartPostCObject, WireSyncReturnStruct, NativePortType;
export 'io.dart' if (dart.library.html) 'web.dart'
    show ExternalLibrary, WireSyncReturnStruct, NativePortType;
import '../isolate.dart' show SendPort;

/// This class, together with its subclasses, are only for internal usage.
/// Usually it should not be used by normal users.
abstract class FlutterRustBridgeWireBase {
  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(DartPostCObject ptr) {}

  /// Not to be used by normal users, but has to be public for generated code
  // ignore: non_constant_identifier_names
  void free_WireSyncReturnStruct(WireSyncReturnStruct val) {}
}

extension NativeType on SendPort {
  NativePortType get nativePort => throw UnimplementedError();
}

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() => throw UnimplementedError();
}

/// Whether the web platform has been isolated by COOP and COEP headers,
/// and is capable of sharing buffers between workers.
///
/// Note: not available on all browsers, in which case it will return null.
bool? get crossOriginIsolated => throw UnimplementedError();

int castInt(Object? value) => value as int;

/// Only used on the Web.
Object castNativeBigInt(int value) => throw UnimplementedError();

abstract class FlutterRustBridgeWasmWireBase<T extends WasmModule>
    extends FlutterRustBridgeWireBase {
  Future<T> get init => throw UnimplementedError();
  FlutterRustBridgeWasmWireBase([FutureOr<T>? module]);
}

class JS {
  const JS([String? name]);
}

class _Anonymous {
  const _Anonymous();
}

const anonymous = _Anonymous();

dynamic eval(String script) => throw UnimplementedError();

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

  /// Initialize a [WasmModule] with the specified kind of [Modules].
  static FutureOr<WasmModule> initialize(
          {required Modules kind, WasmModule Function()? module}) =>
      throw UnimplementedError();
}

/// Currently supported modes of module initialization.
///
/// Advanced users may wish to inherit this class and override [initializeModule]
/// to provide their own initialization process.
abstract class Modules {
  const Modules();

  /// Initialize a `wasm_bindgen` module built with the `-t no-modules` flag.
  ///
  /// The expected output is a file named `$root.js` and the accompanying
  /// WASM binary named `${root}_bg.wasm`.
  const factory Modules.noModules({required String root}) =
      _WasmBindgenNoModules;

  /// How a WASM module is brought into Dart's scope and initialized.
  ///
  /// Override this method to define custom initialization processes.
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module);
}

class _WasmBindgenNoModules extends Modules {
  final String root;
  const _WasmBindgenNoModules({required this.root});

  @override
  FutureOr<WasmModule> initializeModule(WasmModule Function()? module) =>
      throw UnimplementedError();
}
