import 'package:flutter_rust_bridge/src/ffigen_generated/frb_rust.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class GeneralizedFrbRustBinding {
  final FrbRustCBinding _binding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(this._binding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(DartPostCObject ptr) {
    throw UnimplementedError();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    throw UnimplementedError();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    throw UnimplementedError();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj) {
    throw UnimplementedError();
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn val) {
    throw UnimplementedError();
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
