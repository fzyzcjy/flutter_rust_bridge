// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `ownership_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/ownership_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(borrowStringTwinRustAsyncSse, ['a']);
  addTestsIdentityFunctionCall(borrowStrTwinRustAsyncSse, ['a']);
  addTestsIdentityFunctionCall(borrowI32TwinRustAsyncSse, [100]);
  addTestsIdentityFunctionCall(borrowSliceU8TwinRustAsyncSse, [
    [10, 20, 30]
  ]);
  addTestsIdentityFunctionCall(borrowSliceStringTwinRustAsyncSse, [
    ['a', 'b']
  ]);
  addTestsIdentityFunctionCall(borrowStructTwinRustAsyncSse,
      [SimpleStructForBorrowTwinRustAsyncSse(one: 'a')]);
}
