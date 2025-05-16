// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/structure_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcStructWithZeroFieldTwinSse, [StructWithZeroFieldTwinSse()]);
  addTestsIdentityFunctionCall(
      funcStructWithOneFieldTwinSse, [StructWithOneFieldTwinSse(a: 42)]);
  addTestsIdentityFunctionCall(
      funcStructWithTwoFieldTwinSse, [StructWithTwoFieldTwinSse(a: 10, b: 20)]);
  addTestsIdentityFunctionCall(funcTupleStructWithOneFieldTwinSse,
      [TupleStructWithOneFieldTwinSse(field0: 42)]);
  addTestsIdentityFunctionCall(funcTupleStructWithTwoFieldTwinSse,
      [TupleStructWithTwoFieldTwinSse(field0: 10, field1: 20)]);
}
