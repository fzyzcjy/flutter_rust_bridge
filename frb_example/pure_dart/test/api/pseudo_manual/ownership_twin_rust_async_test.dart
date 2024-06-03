// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/ownership_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(borrowStringTwinRustAsync, ['a']);
  addTestsIdentityFunctionCall(borrowStrTwinRustAsync, ['a']);
  addTestsIdentityFunctionCall(borrowI32TwinRustAsync, [100]);
  addTestsIdentityFunctionCall(borrowSliceU8TwinRustAsync, [
    [10, 20, 30]
  ]);
  addTestsIdentityFunctionCall(borrowSliceStringTwinRustAsync, [
    ['a', 'b']
  ]);
  addTestsIdentityFunctionCall(borrowStructTwinRustAsync,
      [SimpleStructForBorrowTwinRustAsync(one: 'a')]);
}
