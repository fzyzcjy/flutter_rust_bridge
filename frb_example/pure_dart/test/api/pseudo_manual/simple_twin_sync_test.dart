// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `simple_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/simple_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call simpleAdder', () async {
    expect(await simpleAdderTwinSync(a: 42, b: 100), 142);
  });
}
