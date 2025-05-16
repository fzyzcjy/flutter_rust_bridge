// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/ownership_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(borrowStringTwinSyncSse, ['a']);
  addTestsIdentityFunctionCall(borrowStrTwinSyncSse, ['a']);
  addTestsIdentityFunctionCall(borrowI32TwinSyncSse, [100]);
  addTestsIdentityFunctionCall(borrowSliceU8TwinSyncSse, [
    [10, 20, 30]
  ]);
  addTestsIdentityFunctionCall(borrowSliceStringTwinSyncSse, [
    ['a', 'b']
  ]);
  addTestsIdentityFunctionCall(
      borrowStructTwinSyncSse, [SimpleStructForBorrowTwinSyncSse(one: 'a')]);
}
