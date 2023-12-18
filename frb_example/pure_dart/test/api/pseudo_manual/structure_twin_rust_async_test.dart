// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/structure_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(funcStructWithZeroFieldTwinRustAsync,
      [StructWithZeroFieldTwinRustAsync()]);
  addTestsIdentityFunctionCall(funcStructWithOneFieldTwinRustAsync,
      [StructWithOneFieldTwinRustAsync(a: 42)]);
  addTestsIdentityFunctionCall(funcStructWithTwoFieldTwinRustAsync,
      [StructWithTwoFieldTwinRustAsync(a: 10, b: 20)]);
  addTestsIdentityFunctionCall(funcTupleStructWithOneFieldTwinRustAsync,
      [TupleStructWithOneFieldTwinRustAsync(field0: 42)]);
  addTestsIdentityFunctionCall(funcTupleStructWithTwoFieldTwinRustAsync,
      [TupleStructWithTwoFieldTwinRustAsync(field0: 10, field1: 20)]);
}
