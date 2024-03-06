// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/inside_macro.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  addTestsIdentityFunctionCall(
      funcMacroStructTwinNormal, [const MacroStruct(data: 42)]);

  test("macro struct", () async {
    var x = await anotherMacroStructTwinNormal();
    expect(x.data, 123);
    x.nonFinalData = 321;
    expect(x.nonFinalData, 321);
  });
}
