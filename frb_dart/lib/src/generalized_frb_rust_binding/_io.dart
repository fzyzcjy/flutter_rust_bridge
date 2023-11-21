import 'package:flutter_rust_bridge/src/ffigen_generated/frb_rust.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class GeneralizedFrbRustBinding {
  final FrbRustCBinding _binding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(this._binding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject(DartPostCObject ptr) {
    TODO;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object getDartObject(int ptr) {
    TODO;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dropDartObject(int ptr) {
    TODO;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int newDartOpaque(Object obj) {
    TODO;
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(WireSyncReturn val) {
    TODO;
  }
}

// TODO rm
/// {@macro flutter_rust_bridge.only_for_generated_code}
extension StoreDartPostCObjectExt on BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}
