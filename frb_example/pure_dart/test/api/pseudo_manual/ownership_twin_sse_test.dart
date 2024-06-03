// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/ownership_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(borrowStringTwinSse, ['a']);
  addTestsIdentityFunctionCall(borrowStrTwinSse, ['a']);
  addTestsIdentityFunctionCall(borrowI32TwinSse, [100]);
  addTestsIdentityFunctionCall(borrowSliceU8TwinSse, [
    [10, 20, 30]
  ]);
  addTestsIdentityFunctionCall(borrowSliceStringTwinSse, [
    ['a', 'b']
  ]);
  addTestsIdentityFunctionCall(
      borrowStructTwinSse, [SimpleStructForBorrowTwinSse(one: 'a')]);
}
