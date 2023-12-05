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
  Object getDartObject(int ptr) => _getDartObject(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dropDartObject(int ptr) => _dropDartObject(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int newDartOpaque(Object obj, NativePortType port) =>
      throw UnimplementedError();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(WireSyncReturn raw) {}
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.get_dart_object")
external Object _getDartObject(int ptr);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.drop_dart_object")
external void _dropDartObject(int ptr);
