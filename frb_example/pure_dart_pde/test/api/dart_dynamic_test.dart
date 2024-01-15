// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/dart_dynamic.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call return_dart_dynamic', () async {
    final data = await returnDartDynamicTwinNormal();
    expect(data, ['foo']);
  });
}
