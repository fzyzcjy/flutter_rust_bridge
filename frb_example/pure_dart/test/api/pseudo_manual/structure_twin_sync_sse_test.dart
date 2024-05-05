// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/structure_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcStructWithZeroFieldTwinSyncSse, [StructWithZeroFieldTwinSyncSse()]);
  addTestsIdentityFunctionCall(funcStructWithOneFieldTwinSyncSse,
      [StructWithOneFieldTwinSyncSse(a: 42)]);
  addTestsIdentityFunctionCall(funcStructWithTwoFieldTwinSyncSse,
      [StructWithTwoFieldTwinSyncSse(a: 10, b: 20)]);
  addTestsIdentityFunctionCall(funcTupleStructWithOneFieldTwinSyncSse,
      [TupleStructWithOneFieldTwinSyncSse(field0: 42)]);
  addTestsIdentityFunctionCall(funcTupleStructWithTwoFieldTwinSyncSse,
      [TupleStructWithTwoFieldTwinSyncSse(field0: 10, field1: 20)]);
}
