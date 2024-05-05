// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `tuple_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/tuple_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test("dart call tuples", () async {
    expect(await testTupleTwinSse(), ('John', 0));
    expect(await testTupleTwinSse(value: ('Bob', 42)), ('Hello Bob', 43));
  });
}
