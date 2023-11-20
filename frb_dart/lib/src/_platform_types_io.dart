import 'dart:ffi' as ffi;

/// Abstraction over a Dart SendPort and a JS MessagePort.
typedef NativePortType = int;

typedef WireSyncReturn = ffi.Pointer<Dart_CObject>;
