import 'dart:ffi' as ffi;
import 'dart:ffi';
export 'dart:ffi' show NativePort, DynamicLibrary;

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

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

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  /// Not to be used by normal users, but has to be public for generated code
  external ffi.Pointer<ffi.Uint8> ptr;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Int32()
  external int len;

  /// Not to be used by normal users, but has to be public for generated code
  @ffi.Uint8()
  external int success;

  Uint8List? get buffer {
    if (ptr.address == 0) {
      return null;
    }
    return Uint8List.fromList(ptr.asTypedList(len));
  }

  bool get isSuccess => success > 0;
}

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
