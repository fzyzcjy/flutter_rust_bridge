import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/ffigen_generated/dart_native_api.dart';

/// Abstraction over a Dart SendPort and a JS MessagePort.
///
/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef NativePortType = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef WireSyncReturn = ffi.Pointer<Dart_CObject>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformPointer = ffi.Pointer<ffi.Void>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef ExternalLibrary = ffi.DynamicLibrary;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef DartPostCObject = ffi.Pointer<ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>;
