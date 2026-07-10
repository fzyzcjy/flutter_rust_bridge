// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `namespaced_type_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/namespaced_type_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('returns a model from its namespaced module', () async {
    final models = await duplicateNamedModelsTwinRustAsync();
    expect(models.single.value, 'second');
  });
}
