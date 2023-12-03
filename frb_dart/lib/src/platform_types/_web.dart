import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef NativePortType = dynamic;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef WireSyncReturn = List<dynamic>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformPointer = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef DartPostCObject = void;

/// {@macro flutter_rust_bridge.only_for_generated_code}
class ExternalLibrary extends BaseExternalLibrary {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final Object wasmModule;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const ExternalLibrary({required this.wasmModule, required super.debugInfo});
}

/// {@macro flutter_rust_bridge.internal}
class PlatformPointerUtil {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer ptrFromInt(int ptr) => ptr;

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => 0;

  /// {@macro flutter_rust_bridge.internal}
  static bool isNullPtr(PlatformPointer ptr) => ptr == 0;
}
