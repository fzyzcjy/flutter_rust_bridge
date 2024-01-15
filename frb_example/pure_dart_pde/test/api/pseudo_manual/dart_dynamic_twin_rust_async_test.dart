// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_dynamic_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/dart_dynamic_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call return_dart_dynamic', () async {
    final data = await returnDartDynamicTwinRustAsync();
    expect(data, ['foo']);
  });
}
