import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

// TODO rename the class (should not have `FlutterRustBridge` prefix when possible)
/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class BaseWire {
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

/// {@macro flutter_rust_bridge.only_for_generated_code}
extension StoreDartPostCObjectExt on BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}
