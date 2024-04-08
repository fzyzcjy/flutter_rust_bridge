// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/external_impl.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('SimpleTranslatableExternalStructWithMethod', () async {
    expect(
        await SimpleTranslatableExternalStructWithMethod(a: 'hello')
            .simpleExternalMethod(),
        'hello');
  });

  test('SimpleTranslatableExternalStructWithMethod', () async {
    expect(
        await SimpleTranslatableExternalStructWithMethod(a: 'hello')
            .simpleExternalMethod(),
        'hello');
  });
}
