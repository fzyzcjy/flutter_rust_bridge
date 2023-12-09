// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `inside_macro_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/inside_macro_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcMacroStructTwinSse, [const MacroStruct(data: 42)]);

  test("macro struct", () async {
    var x = await anotherMacroStructTwinSse();
    expect(x.data, 123);
    x.nonFinalData = 321;
    expect(x.nonFinalData, 321);
  });
}
