import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/ffigen_generated/allo_isolate.dart';
import 'package:flutter_rust_bridge/src/ffigen_generated/frb_rust.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
abstract class GeneralizedFrbRustBinding {
  final FrbRustCBinding _frbRustCBinding;
  final AlloIsolateCBinding _alloIsolateCBinding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding({
    required FrbRustCBinding frbRustCBinding,
    required AlloIsolateCBinding alloIsolateCBinding,
  })  : _frbRustCBinding = frbRustCBinding,
        _alloIsolateCBinding = alloIsolateCBinding;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject(DartPostCObject ptr) =>
      _alloIsolateCBinding.store_dart_post_cobject(ffi.NativeApi.postCObject.cast());

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object getDartObject(int ptr) => _frbRustCBinding.get_dart_object(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dropDartObject(int ptr) => _frbRustCBinding.drop_dart_object(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int newDartOpaque(Object obj) => _frbRustCBinding.new_dart_opaque(obj);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncReturn(WireSyncReturn val) => _frbRustCBinding.free_wire_sync_return(val);
}
