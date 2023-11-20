import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

typedef OpaqueTypeFinalizer = Finalizer<PlatformPointer>;

class FrbOpaqueBase {
  static PlatformPointer initPtr(int ptr) => ptr;

  static PlatformPointer nullPtr() => 0;

  static bool isStalePtr(PlatformPointer ptr) => ptr == 0;

  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr, int _, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque);
}
