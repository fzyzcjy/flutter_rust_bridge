import 'dart:async';

import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef NativePortType = dynamic;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef WireSyncReturn = List<dynamic>;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef PlatformPointer = int;

/// {@macro flutter_rust_bridge.only_for_generated_code}
typedef DartPostCObject = void;

/// {@macro flutter_rust_bridge.only_for_generated_code}
class ExternalLibrary {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final FutureOr<WasmModule> wasmModule;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const ExternalLibrary({required this.wasmModule});
}
