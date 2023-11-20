/// An opaque pointer to a native C (valid for io only) or Rust (valid for both io and web) type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
class FrbOpaqueBase implements Finalizable {
  static PlatformPointer initPtr(int ptr) => ffi.Pointer.fromAddress(ptr);

  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);

  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;

  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr, int size, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
