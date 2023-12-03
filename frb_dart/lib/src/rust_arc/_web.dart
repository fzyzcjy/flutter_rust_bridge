import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = Finalizer<PlatformPointer>;

/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizerArg = void Function(PlatformPointer);

/// {@macro flutter_rust_bridge.internal}
class RustArcBase {
  /// {@macro flutter_rust_bridge.internal}
  static void finalizerAttach(RustArcBase object, PlatformPointer ptr, int _,
          ArcTypeFinalizer finalizer) =>
      finalizer.attach(object, ptr, detach: object);
}
