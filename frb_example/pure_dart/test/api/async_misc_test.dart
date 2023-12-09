// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "syncSse", "rustAsyncSse"]}

import 'package:frb_example_pure_dart/src/rust/api/async_misc.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcAsyncVoid', () async {
    await funcAsyncVoid();
  });

  test('dart call funcAsyncSimpleAdd', () async {
    expect(await funcAsyncSimpleAdd(a: 10, b: 20), 30);
  });
}
