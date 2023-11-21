import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:js/js.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class GeneralizedFrbRustBinding {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn raw) {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    return getDartObject(ptr);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    dropDartObject(ptr);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj, NativePortType port) {
    throw UnimplementedError();
  }
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.get_dart_object")
external Object getDartObject(int ptr);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.drop_dart_object")
external void dropDartObject(int ptr);
