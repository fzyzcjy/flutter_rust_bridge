import 'dart:ffi' as ffi;
import 'dart:ffi';

import 'stub.dart' show FlutterRustBridgeWireBase;

export 'dart:ffi' show NativePort, DynamicLibrary;

export 'stub.dart' show castInt, castNativeBigInt, FlutterRustBridgeWireBase, WasmModule;

typedef ExternalLibrary = ffi.DynamicLibrary;
typedef DartPostCObject = ffi.Pointer<ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>;

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}

class DartApiDl {
  static int? _initCode;
  final int Function(ffi.Pointer<ffi.Void>) _initFn;

  DartApiDl(this._initFn);

  void initApi() {
    _initCode ??= _initFn(ffi.NativeApi.initializeApiDLData);
    if (_initCode != 0) {
      throw Exception('Failed to initialize Dart API. Code: $_initCode');
    }
  }
}

typedef PlatformPointer = ffi.Pointer<ffi.Void>;
typedef OpaqueTypeFinalizer = NativeFinalizer;
