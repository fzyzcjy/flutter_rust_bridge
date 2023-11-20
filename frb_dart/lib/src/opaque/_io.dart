import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

class FrbOpaqueBase implements ffi.Finalizable {
  static PlatformPointer initPtr(int ptr) => ffi.Pointer.fromAddress(ptr);

  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);

  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;

  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr, int size, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
