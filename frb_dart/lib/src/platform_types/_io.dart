import 'dart:ffi' as ffi;

import '../ffigen_generated/dart_native_api.dart';

/// Abstraction over a Dart SendPort and a JS MessagePort.
typedef NativePortType = int;

typedef WireSyncReturn = ffi.Pointer<Dart_CObject>;

typedef PlatformPointer = ffi.Pointer<ffi.Void>;
