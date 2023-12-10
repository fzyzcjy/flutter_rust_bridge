import 'dart:ffi' as ffi;
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

export 'package:flutter_rust_bridge/src/ffigen_generated/multi_package.dart'
    show WireSyncReturnDco, WireSyncReturnSse;

/// Abstraction over a Dart SendPort and a JS MessagePort.
///
/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef NativePortType = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformPointer = ffi.Pointer<ffi.Void>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef DartPostCObject = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>;

/// {@macro flutter_rust_bridge.internal}
Uint8List wireSyncReturnSseAsUint8ListView(WireSyncReturnSse raw) =>
    raw.ptr.asTypedList(raw.len);

/// {@macro flutter_rust_bridge.only_for_generated_code}
class ExternalLibrary extends BaseExternalLibrary {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final ffi.DynamicLibrary ffiDynamicLibrary;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const ExternalLibrary(
      {required this.ffiDynamicLibrary, required super.debugInfo});

  /// {@macro flutter_rust_bridge.internal}
  factory ExternalLibrary.open(String path, {String debugInfo = ''}) =>
      ExternalLibrary(
        ffiDynamicLibrary: ffi.DynamicLibrary.open(path),
        debugInfo: 'by open($path)$debugInfo',
      );

  /// {@macro flutter_rust_bridge.internal}
  factory ExternalLibrary.process({String debugInfo = ''}) => ExternalLibrary(
        ffiDynamicLibrary: ffi.DynamicLibrary.process(),
        debugInfo: 'by process()$debugInfo',
      );
}

/// {@macro flutter_rust_bridge.internal}
class PlatformPointerUtil {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer ptrFromInt(int ptr) => ffi.Pointer.fromAddress(ptr);

  /// {@macro flutter_rust_bridge.internal}
  static int ptrToInt(PlatformPointer ptr) => ptr.address;

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);

  /// {@macro flutter_rust_bridge.internal}
  static bool isNullPtr(PlatformPointer ptr) => ptr.address == 0;
}
