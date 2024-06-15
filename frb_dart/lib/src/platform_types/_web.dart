import 'dart:js_interop';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef NativePortType = web.EventTarget;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef WireSyncRust2DartDco = List<dynamic>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef WireSyncRust2DartSse = JSUint8Array;

/// {@macro flutter_rust_bridge.internal}
Uint8List wireSyncRust2DartSseAsUint8ListView(WireSyncRust2DartSse raw) =>
    raw.toDart;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformPointer = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformInt64 = BigInt;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef DartPostCObject = void;

/// {@macro flutter_rust_bridge.only_for_generated_code}
class ExternalLibrary extends BaseExternalLibrary {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const ExternalLibrary({required super.debugInfo});
}

/// {@macro flutter_rust_bridge.internal}
class PlatformPointerUtil {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer ptrFromInt(int ptr) => ptr;

  /// {@macro flutter_rust_bridge.internal}
  static int ptrToInt(int ptr) => ptr;

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => 0;

  /// {@macro flutter_rust_bridge.internal}
  static bool isNullPtr(PlatformPointer ptr) => ptr == 0;
}

/// {@macro flutter_rust_bridge.internal}
class PlatformInt64Util {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformInt64 from(int value) => PlatformInt64.from(value);
}
