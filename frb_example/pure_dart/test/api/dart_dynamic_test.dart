// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sse", "sync sse", "rustAsync sse"], "skipPde": true}

import 'package:frb_example_pure_dart/src/rust/api/dart_dynamic.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call return_dart_dynamic', () async {
    final data = await returnDartDynamicTwinNormal();
    expect(data, ['foo']);
  });
}
