import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
class DroppableBase {}

/// {@macro flutter_rust_bridge.internal}
typedef CrossPlatformFinalizer = Finalizer<PlatformPointer>;

/// {@macro flutter_rust_bridge.internal}
typedef CrossPlatformFinalizerArg = void Function(PlatformPointer);

/// {@macro flutter_rust_bridge.internal}
extension ExtFinalizer on Finalizer<PlatformPointer> {
  /// {@macro flutter_rust_bridge.internal}
  void attachCrossPlatform(
    Object value,
    PlatformPointer finalizationToken, {
    Object? detach,
    // ignore: unused_element
    int? externalSizeOnNative,
  }) =>
      attach(value, finalizationToken, detach: detach);
}
