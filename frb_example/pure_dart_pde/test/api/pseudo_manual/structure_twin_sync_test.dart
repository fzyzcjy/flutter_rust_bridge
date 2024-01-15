// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `structure_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/structure_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcStructWithZeroFieldTwinSync, [StructWithZeroFieldTwinSync()]);
  addTestsIdentityFunctionCall(
      funcStructWithOneFieldTwinSync, [StructWithOneFieldTwinSync(a: 42)]);
  addTestsIdentityFunctionCall(funcStructWithTwoFieldTwinSync,
      [StructWithTwoFieldTwinSync(a: 10, b: 20)]);
  addTestsIdentityFunctionCall(funcTupleStructWithOneFieldTwinSync,
      [TupleStructWithOneFieldTwinSync(field0: 42)]);
  addTestsIdentityFunctionCall(funcTupleStructWithTwoFieldTwinSync,
      [TupleStructWithTwoFieldTwinSync(field0: 10, field1: 20)]);
}
