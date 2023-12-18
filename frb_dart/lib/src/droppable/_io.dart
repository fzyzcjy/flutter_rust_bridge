import 'dart:ffi' as ffi;

/// {@macro flutter_rust_bridge.internal}
class DroppableBase implements ffi.Finalizable {}

/// {@macro flutter_rust_bridge.internal}
typedef CrossPlatformFinalizer = ffi.NativeFinalizer;

/// {@macro flutter_rust_bridge.internal}
typedef CrossPlatformFinalizerArg = ffi.Pointer<ffi.NativeFinalizerFunction>;

/// {@macro flutter_rust_bridge.internal}
extension ExtFinalizer on ffi.NativeFinalizer {
  /// {@macro flutter_rust_bridge.internal}
  void attachCrossPlatform(
    ffi.Finalizable value,
    ffi.Pointer<ffi.Void> token, {
    Object? detach,
    int? externalSizeOnNative,
  }) =>
      attach(value, token, detach: detach, externalSize: externalSizeOnNative);
}
