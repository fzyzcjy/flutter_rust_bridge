// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'rust_opaque.dart';

String syncRunOpaque({required NonSendHideData opaque, dynamic hint}) =>
    RustLib.instance.api.syncRunOpaque(opaque: opaque, hint: hint);

/// Structure for testing the sync-mode RustOpaque code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
Future<FrbOpaqueSyncReturn> frbSyncGeneratorTest({dynamic hint}) =>
    RustLib.instance.api.frbSyncGeneratorTest(hint: hint);

@sealed
class FrbOpaqueSyncReturn extends FrbOpaque {
  FrbOpaqueSyncReturn.fromRaw(int ptr, int size) : super.unsafe(ptr, size);

  @override
  OpaqueDropFnType get dropFn =>
      RustLib.instance.api.dropOpaqueFrbOpaqueSyncReturn;

  @override
  OpaqueShareFnType get shareFn =>
      RustLib.instance.api.shareOpaqueFrbOpaqueSyncReturn;

  @override
  OpaqueTypeFinalizer get staticFinalizer =>
      RustLib.instance.api.frbOpaqueSyncReturnFinalizer;
}
