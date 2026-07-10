// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/namespaced_type.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('returns a model from its namespaced module', () async {
    final models = await duplicateNamedModelsTwinNormal();
    expect(models.single.value, 'second');
  });
}
