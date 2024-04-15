import 'dart:ffi' as ffi;

import 'package:flutter_rust_bridge/src/ffigen_generated/multi_package.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/generalized_uint8list.dart';
import 'package:flutter_rust_bridge/src/platform_types/_io.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class GeneralizedFrbRustBinding {
  final MultiPackageCBinding _binding;
  final String _externalLibraryDebugInfo;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  GeneralizedFrbRustBinding(ExternalLibrary externalLibrary)
      : _binding = MultiPackageCBinding(externalLibrary.ffiDynamicLibrary),
        _externalLibraryDebugInfo = externalLibrary.debugInfo;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void storeDartPostCObject() {
    try {
      _binding.store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
    } on ArgumentError catch (e, s) {
      _userFriendlyDynamicLibraryErrorReporting(e, s);
      rethrow;
    }
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void initFrbDartApiDl() =>
      _binding.init_frb_dart_api_dl(ffi.NativeApi.initializeApiDLData);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void pdeFfiDispatcherPrimary({
    required int funcId,
    required NativePortType port,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    _binding.frb_pde_ffi_dispatcher_primary(
        funcId, port, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  WireSyncRust2DartSse pdeFfiDispatcherSync({
    required int funcId,
    required PlatformGeneralizedUint8ListPtr ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    return _binding.frb_pde_ffi_dispatcher_sync(
        funcId, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartFnDeliverOutput({
    required int callId,
    required ffi.Pointer<ffi.Uint8> ptr,
    required int rustVecLen,
    required int dataLen,
  }) {
    return _binding.dart_fn_deliver_output(callId, ptr, rustVecLen, dataLen);
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  int getRustContentHash() {
    try {
      return _binding.frb_get_rust_content_hash();
    } on ArgumentError catch (e, s) {
      _userFriendlyDynamicLibraryErrorReporting(e, s);
      rethrow;
    }
  }

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  PlatformPointer dartOpaqueDart2RustEncode(
          Object object, NativePortType dartHandlerPort) =>
      _binding.dart_opaque_dart2rust_encode(object, dartHandlerPort);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  Object dartOpaqueRust2DartDecode(int ptr) =>
      _binding.dart_opaque_rust2dart_decode(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void dartOpaqueDropThreadBoxPersistentHandle(int ptr) =>
      _binding.dart_opaque_drop_thread_box_persistent_handle(ptr);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartDco(WireSyncRust2DartDco val) =>
      _binding.free_wire_sync_rust2dart_dco(val);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void freeWireSyncRust2DartSse(WireSyncRust2DartSse val) =>
      _binding.free_wire_sync_rust2dart_sse(val);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  ffi.Pointer<ffi.Uint8> rustVecU8New(int len) => _binding.rust_vec_u8_new(len);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  ffi.Pointer<ffi.Uint8> rustVecU8Resize(
          ffi.Pointer<ffi.Uint8> ptr, int oldLen, int newLen) =>
      _binding.rust_vec_u8_resize(ptr, oldLen, newLen);

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  void rustVecU8Free(ffi.Pointer<ffi.Uint8> ptr, int len) =>
      _binding.rust_vec_u8_free(ptr, len);

  void _userFriendlyDynamicLibraryErrorReporting(
      ArgumentError e, StackTrace s) {
    final message = e.message;
    if (message is String && message.contains('Failed to lookup symbol')) {
      throw ArgumentError(
        '$e\n'
        'This is often because the Rust library is not loaded correctly.\n'
        'Debug information of the external library: $_externalLibraryDebugInfo\n'
        'Original stack trace: $s',
      );
    }
  }
}
