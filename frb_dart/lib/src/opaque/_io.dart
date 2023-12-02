import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.internal}
typedef OpaqueTypeFinalizer = ffi.NativeFinalizer;

/// {@macro flutter_rust_bridge.internal}
class RustOpaqueBase implements ffi.Finalizable {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer initPtr(int ptr) => ffi.Pointer.fromAddress(ptr);

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);

  /// {@macro flutter_rust_bridge.internal}
  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;

  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(RustOpaqueBase opaque, PlatformPointer ptr,
          int size, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
