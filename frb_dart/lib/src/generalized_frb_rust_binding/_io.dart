import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/ffigen_generated/multi_package.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class GeneralizedFrbRustBinding {
  final MultiPackageCBinding _binding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(this._binding);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject(DartPostCObject ptr) => _binding.store_dart_post_cobject(ffi.NativeApi.postCObject.cast());

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object getDartObject(int ptr) => _binding.get_dart_object(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dropDartObject(int ptr) => _binding.drop_dart_object(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int newDartOpaque(Object obj) => _binding.new_dart_opaque(obj);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(WireSyncReturn val) => _binding.free_wire_sync_return(val);
}
