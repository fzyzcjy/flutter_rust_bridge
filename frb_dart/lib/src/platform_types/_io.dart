import 'dart:ffi' as ffi;
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

export 'package:flutter_rust_bridge/src/ffigen_generated/multi_package.dart'
    show WireSyncRust2DartDco, WireSyncRust2DartSse;

/// Abstraction over a Dart SendPort and a JS MessagePort.
///
/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef NativePortType = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformPointer = ffi.Pointer<ffi.Void>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformInt64 = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef DartPostCObject =
    ffi.Pointer<
      ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>
    >;

/// {@macro flutter_rust_bridge.internal}
Uint8List wireSyncRust2DartSseAsUint8ListView(WireSyncRust2DartSse raw) =>
    raw.ptr.asTypedList(raw.len);

/// {@macro flutter_rust_bridge.only_for_generated_code}
class ExternalLibrary extends BaseExternalLibrary {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final ffi.DynamicLibrary ffiDynamicLibrary;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // Do *not* make this constructor public, otherwise the protection in
  // `.process()` constructor will not be in effect.
  const ExternalLibrary._({
    required this.ffiDynamicLibrary,
    required super.debugInfo,
  });

  /// {@macro flutter_rust_bridge.internal}
  factory ExternalLibrary.open(String path, {String debugInfo = ''}) =>
      ExternalLibrary._(
        ffiDynamicLibrary: ffi.DynamicLibrary.open(path),
        debugInfo: 'by open($path)$debugInfo',
      );

  /// Firstly, usually you do NOT need to use this function at all.
  /// Under all platforms, Flutter officially suggests `open()`
  /// (see https://github.com/flutter/flutter/blob/8b6277e63868c2029f1e2327879b7899be44fbe2/packages/flutter_tools/templates/plugin_ffi/lib/projectName.dart.tmpl#L47-L58),
  /// and flutter_rust_bridge's template also uses the `open()`.
  ///
  /// When linking multiple FRB packages into the process, generate each package
  /// with a unique `native_symbol_prefix` and share a compatible FRB runtime.
  /// Executables must also export those symbols for dynamic lookup.
  ///
  /// After reading the remarks above, set [iKnowHowToUseIt] to true to use it :)
  factory ExternalLibrary.process({
    required bool iKnowHowToUseIt,
    String debugInfo = '',
  }) {
    assert(iKnowHowToUseIt);
    return ExternalLibrary._(
      ffiDynamicLibrary: ffi.DynamicLibrary.process(),
      debugInfo: 'by process()$debugInfo',
    );
  }
}

/// {@macro flutter_rust_bridge.internal}
class PlatformPointerUtil {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer ptrFromInt(int ptr) => ffi.Pointer.fromAddress(ptr);

  /// {@macro flutter_rust_bridge.internal}
  static int ptrToInt(PlatformPointer ptr) => ptr.address;

  /// {@macro flutter_rust_bridge.internal}
  static PlatformInt64 ptrToPlatformInt64(PlatformPointer ptr) => ptrToInt(ptr);

  /// {@macro flutter_rust_bridge.internal}
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);

  /// {@macro flutter_rust_bridge.internal}
  static bool isNullPtr(PlatformPointer ptr) => ptr.address == 0;
}

/// {@macro flutter_rust_bridge.internal}
class PlatformInt64Util {
  /// {@macro flutter_rust_bridge.internal}
  static PlatformInt64 from(int value) => value;
}
