// ignore_for_file: non_constant_identifier_names

import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:js/js.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class GeneralizedFrbRustBinding {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(ExternalLibrary externalLibrary);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initFrbDartApiDl() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object dartOpaqueRust2DartDecode(int ptr) =>
      _dart_opaque_rust2dart_decode(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartOpaqueDropThreadBoxPersistentHandle(int ptr) =>
      _dart_opaque_drop_thread_box_persistent_handle(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int newDartOpaque(Object obj, NativePortType port) =>
      throw UnimplementedError();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(WireSyncReturn raw) {}
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_rust2dart_decode")
external Object _dart_opaque_rust2dart_decode(int ptr);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_drop_thread_box_persistent_handle")
external void _dart_opaque_drop_thread_box_persistent_handle(int ptr);
