// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/async_misc.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcAsyncVoid', () async {
    await funcAsyncVoidTwinNormal();
  });

  test('dart call funcAsyncSimpleAdd', () async {
    expect(await funcAsyncSimpleAddTwinNormal(a: 10, b: 20), 30);
  });
}
