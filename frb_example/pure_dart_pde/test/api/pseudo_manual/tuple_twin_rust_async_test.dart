// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `tuple_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/tuple_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test("dart call tuples", () async {
    expect(await testTupleTwinRustAsync(), ('John', 0));
    expect(await testTupleTwinRustAsync(value: ('Bob', 42)), ('Hello Bob', 43));
  });
}
