// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `dart_code_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/dart_code_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('TranslatableStructWithDartCodeTwinSync', () async {
    final one = TranslatableStructWithDartCodeTwinSync(a: 100);
    final two = TranslatableStructWithDartCodeTwinSync(a: 100);
    expect(one.hashCode, two.hashCode);
    expect(one == two, true);
    expect(one.dartExtraMethod(), 200);
    expect(await one.normalMethod(), 200);
  });

  test('OpaqueStructWithDartCodeTwinSync', () async {
    expect(OpaqueStructWithDartCodeTwinSync.dartCodeGetter, 123);
  });
}
