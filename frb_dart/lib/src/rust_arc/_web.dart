import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = Finalizer<PlatformPointer>;

/// {@macro flutter_rust_bridge.internal}
class RustArcBase {
  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(RustArcBase opaque, PlatformPointer ptr, int _,
          ArcTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque);
}
