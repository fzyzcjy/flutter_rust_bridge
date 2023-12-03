import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

// TODO rename to RustArc.*
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = ffi.NativeFinalizer;

/// {@macro flutter_rust_bridge.internal}
class RustArcBase implements ffi.Finalizable {
  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(RustArcBase opaque, PlatformPointer ptr, int size,
          ArcTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
