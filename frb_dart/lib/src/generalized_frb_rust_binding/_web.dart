// ignore_for_file: non_constant_identifier_names

import 'dart:js_interop';
import 'dart:js_interop_unsafe';

import 'package:flutter_rust_bridge/src/generalized_uint8list/generalized_uint8list.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:web/web.dart' as web;

/// {@macro flutter_rust_bridge.only_for_generated_code}
class GeneralizedFrbRustBinding {
  final String _wasmBindgenName;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(ExternalLibrary externalLibrary)
      : _wasmBindgenName = externalLibrary.wasmBindgenName;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initFrbDartApiDl() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initShutdownWatcher() {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void pdeFfiDispatcherPrimary({
    required int funcId,
    required NativePortType port,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    _wasmBindgen.frb_pde_ffi_dispatcher_primary(
        funcId, port, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  WireSyncRust2DartSse pdeFfiDispatcherSync({
    required int funcId,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    return _wasmBindgen.frb_pde_ffi_dispatcher_sync(
        funcId, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartFnDeliverOutput({
    required int callId,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    return _wasmBindgen.frb_dart_fn_deliver_output(
        callId, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int getRustContentHash() => _wasmBindgen.frb_get_rust_content_hash();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  PlatformPointer dartOpaqueDart2RustEncode(
          Object object, NativePortType dartHandlerPort) =>
      _wasmBindgen.frb_dart_opaque_dart2rust_encode(
          object.jsify()!, dartHandlerPort);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object dartOpaqueRust2DartDecode(int ptr) =>
      _wasmBindgen.frb_dart_opaque_rust2dart_decode(ptr).dartify()!;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartOpaqueDropThreadBoxPersistentHandle(int ptr) =>
      _wasmBindgen.frb_dart_opaque_drop_thread_box_persistent_handle(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartDco(WireSyncRust2DartDco raw) {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartSse(WireSyncRust2DartSse raw) {}

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  _JSWasmBindgen get _wasmBindgen {
    final jsObject = web.window.getProperty(_wasmBindgenName.toJS) as JSObject;
    return _JSWasmBindgen(jsObject);
  }
}

/// {@macro flutter_rust_bridge.only_for_generated_code}
extension type _JSWasmBindgen(JSObject _) implements JSObject {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_pde_ffi_dispatcher_primary")
  external void frb_pde_ffi_dispatcher_primary(
    int funcId,
    NativePortType port,
    PlatformGeneralizedUint8ListPtr ptr,
    int rustVecLen,
    int dataLen,
  );

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_pde_ffi_dispatcher_sync")
  external WireSyncRust2DartSse frb_pde_ffi_dispatcher_sync(
    int funcId,
    PlatformGeneralizedUint8ListPtr ptr,
    int rustVecLen,
    int dataLen,
  );

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_dart_fn_deliver_output")
  external void frb_dart_fn_deliver_output(
    int callId,
    PlatformGeneralizedUint8ListPtr ptr,
    int rustVecLen,
    int dataLen,
  );

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_get_rust_content_hash")
  external int frb_get_rust_content_hash();

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_dart_opaque_dart2rust_encode")
  external int frb_dart_opaque_dart2rust_encode(
      JSAny object, NativePortType dartHandlerPort);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_dart_opaque_rust2dart_decode")
  external JSAny frb_dart_opaque_rust2dart_decode(int ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @JS("frb_dart_opaque_drop_thread_box_persistent_handle")
  external void frb_dart_opaque_drop_thread_box_persistent_handle(int ptr);
}
