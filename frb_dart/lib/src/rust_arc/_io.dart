import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = ffi.NativeFinalizer;

/// {@macro flutter_rust_bridge.internal}
class RustArcBase implements ffi.Finalizable {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer ptrFromInt(int ptr) => ffi.Pointer.fromAddress(ptr);

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);

  /// {@macro flutter_rust_bridge.internal}
  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;

  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(RustArcBase opaque, PlatformPointer ptr, int size,
          ArcTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
