import 'dart:ffi' as ffi;

// TODO rename to RustArc.*
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizer = ffi.NativeFinalizer;

// TODO rename to RustArc.*
/// {@macro flutter_rust_bridge.internal}
typedef ArcTypeFinalizerArg = ffi.Pointer<ffi.NativeFinalizerFunction>;

/// {@macro flutter_rust_bridge.internal}
class RustArcBase implements ffi.Finalizable {}

/// {@macro flutter_rust_bridge.internal}
extension ExtFinalizer on ffi.NativeFinalizer {
  /// {@macro flutter_rust_bridge.internal}
  void attachCrossPlatform(ffi.Finalizable value, ffi.Pointer<ffi.Void> token,
          {Object? detach, int? externalSize}) =>
      attach(value, token, detach: detach, externalSize: externalSize);
}
