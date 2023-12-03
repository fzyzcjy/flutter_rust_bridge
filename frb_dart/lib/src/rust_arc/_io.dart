import 'dart:ffi' as ffi;

// TODO rename to CrossPlatformFinalizer
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = ffi.NativeFinalizer;

// TODO rename to CrossPlatformFinalizerArg
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizerArg = ffi.Pointer<ffi.NativeFinalizerFunction>;

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
