import 'package:frb_example_pure_dart/src/rust/api/structure.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcStructWithZeroFieldTwinNormal, [StructWithZeroFieldTwinNormal()]);
  addTestsIdentityFunctionCall(
      funcStructWithOneFieldTwinNormal, [StructWithOneFieldTwinNormal(a: 42)]);
  addTestsIdentityFunctionCall(funcStructWithTwoFieldTwinNormal,
      [StructWithTwoFieldTwinNormal(a: 10, b: 20)]);
  addTestsIdentityFunctionCall(funcTupleStructWithOneFieldTwinNormal,
      [TupleStructWithOneFieldTwinNormal(field0: 42)]);
  addTestsIdentityFunctionCall(funcTupleStructWithTwoFieldTwinNormal,
      [TupleStructWithTwoFieldTwinNormal(field0: 10, field1: 20)]);
}
