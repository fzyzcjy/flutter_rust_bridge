import 'package:frb_example_pure_dart/src/rust/api/structure.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main() async {
  await RustLib.init();

  addTestsIdentityFunctionCall(funcStructWithZeroFieldTwinNormal, [StructWithZeroField()]);
  addTestsIdentityFunctionCall(funcStructWithOneFieldTwinNormal, [StructWithOneField(a: 42)]);
  addTestsIdentityFunctionCall(funcStructWithTwoFieldTwinNormal, [StructWithTwoField(a: 10, b: 20)]);
  addTestsIdentityFunctionCall(funcTupleStructWithOneFieldTwinNormal, [TupleStructWithOneField(field0: 42)]);
  addTestsIdentityFunctionCall(
      funcTupleStructWithTwoFieldTwinNormal, [TupleStructWithTwoField(field0: 10, field1: 20)]);
}
