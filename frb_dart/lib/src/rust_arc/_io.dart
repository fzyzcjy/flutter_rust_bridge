import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

// TODO rename to RustArc.*
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = ffi.NativeFinalizer;

// TODO rename to RustArc.*
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizerArg = ffi.Pointer<ffi.NativeFinalizerFunction>;

/// {@macro flutter_rust_bridge.internal}
class RustArcBase implements ffi.Finalizable {
  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(RustArcBase object, PlatformPointer ptr, int size,
          ArcTypeFinalizer finalizer) =>
      finalizer.attach(object, ptr, detach: object, externalSize: size);
}
