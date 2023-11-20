import 'dart:ffi' as ffi;
import 'dart:ffi';
export 'dart:ffi' show NativePort, DynamicLibrary;
import 'package:flutter_rust_bridge/src/ffi/dart_cobject.dart';

import 'stub.dart' show FlutterRustBridgeWireBase;
export 'stub.dart'
    show castInt, castNativeBigInt, FlutterRustBridgeWireBase, WasmModule;

/// Abstraction over a Dart SendPort and a JS MessagePort.
typedef NativePortType = int;
typedef ExternalLibrary = ffi.DynamicLibrary;
typedef DartPostCObject = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>;

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

typedef WireSyncReturn = ffi.Pointer<Dart_CObject>;

List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) =>
    syncReturn.ref.intoDart();

typedef PlatformPointer = ffi.Pointer<ffi.Void>;
typedef OpaqueTypeFinalizer = NativeFinalizer;

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
class FrbOpaqueBase implements Finalizable {
  static PlatformPointer initPtr(int ptr) => ffi.Pointer.fromAddress(ptr);
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);
  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;
  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr,
          int size, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
