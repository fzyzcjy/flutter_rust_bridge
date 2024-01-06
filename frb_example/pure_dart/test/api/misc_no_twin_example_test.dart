// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "syncSse", "rustAsyncSse"]}

import 'package:frb_example_pure_dart/src/rust/api/misc_no_twin_example.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('StructWithArcField.func_async', () async {
    final obj = await StructWithArcField.newStructWithArcField();
    await obj.funcAsync();
  });
}
