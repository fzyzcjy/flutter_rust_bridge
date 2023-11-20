import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

// TODO rename the class (should not have `FlutterRustBridge` prefix when possible)
/// This class, together with its subclasses, are only for internal usage.
/// Usually it should not be used by normal users.
abstract class FlutterRustBridgeWireBase {
  // ignore: non_constant_identifier_names
  void store_dart_post_cobject(DartPostCObject ptr) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  Object get_dart_object(int ptr) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  void drop_dart_object(int ptr) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  int new_dart_opaque(Object obj) {
    throw UnimplementedError();
  }

  // ignore: non_constant_identifier_names
  void free_WireSyncReturn(WireSyncReturn val) {
    throw UnimplementedError();
  }
}

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}
