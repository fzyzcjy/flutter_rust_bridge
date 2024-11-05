// ignore_for_file: non_constant_identifier_names

import 'dart:js_interop';

import 'package:flutter_rust_bridge/src/generalized_uint8list/generalized_uint8list.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class GeneralizedFrbRustBinding {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(ExternalLibrary externalLibrary);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initFrbDartApiDl() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void pdeFfiDispatcherPrimary({
    required int funcId,
    required NativePortType port,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    _frb_pde_ffi_dispatcher_primary(funcId, port, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  WireSyncRust2DartSse pdeFfiDispatcherSync({
    required int funcId,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    return _frb_pde_ffi_dispatcher_sync(funcId, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartFnDeliverOutput({
    required int callId,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    return _dart_fn_deliver_output(callId, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int getRustContentHash() => _frb_get_rust_content_hash();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  PlatformPointer dartOpaqueDart2RustEncode(
          Object object, NativePortType dartHandlerPort) =>
      _dart_opaque_dart2rust_encode(object.jsify()!, dartHandlerPort);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object dartOpaqueRust2DartDecode(int ptr) =>
      _dart_opaque_rust2dart_decode(ptr).dartify()!;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartOpaqueDropThreadBoxPersistentHandle(int ptr) =>
      _dart_opaque_drop_thread_box_persistent_handle(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartDco(WireSyncRust2DartDco raw) {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartSse(WireSyncRust2DartSse raw) {}
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.frb_pde_ffi_dispatcher_primary")
external void _frb_pde_ffi_dispatcher_primary(
  int funcId,
  NativePortType port,
  PlatformGeneralizedUint8ListPtr ptr,
  int rustVecLen,
  int dataLen,
);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.frb_pde_ffi_dispatcher_sync")
external WireSyncRust2DartSse _frb_pde_ffi_dispatcher_sync(
  int funcId,
  PlatformGeneralizedUint8ListPtr ptr,
  int rustVecLen,
  int dataLen,
);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_fn_deliver_output")
external void _dart_fn_deliver_output(
  int callId,
  PlatformGeneralizedUint8ListPtr ptr,
  int rustVecLen,
  int dataLen,
);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.frb_get_rust_content_hash")
external int _frb_get_rust_content_hash();

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_dart2rust_encode")
external int _dart_opaque_dart2rust_encode(
    JSAny object, NativePortType dartHandlerPort);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_rust2dart_decode")
external JSAny _dart_opaque_rust2dart_decode(int ptr);

/// {@macro flutter_rust_bridge.only_for_generated_code}
@JS("wasm_bindgen.dart_opaque_drop_thread_box_persistent_handle")
external void _dart_opaque_drop_thread_box_persistent_handle(int ptr);
