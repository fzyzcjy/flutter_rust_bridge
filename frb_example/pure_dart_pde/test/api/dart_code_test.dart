// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/dart_code.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('TranslatableStructWithDartCodeTwinNormal', () async {
    final one = TranslatableStructWithDartCodeTwinNormal(a: 100);
    final two = TranslatableStructWithDartCodeTwinNormal(a: 100);
    expect(one.hashCode, two.hashCode);
    expect(one == two, true);
    expect(one.dartExtraMethod(), 200);
    expect(await one.normalMethod(), 200);
  });

  test('OpaqueStructWithDartCodeTwinNormal', () async {
    expect(OpaqueStructWithDartCodeTwinNormal.dartCodeGetter, 123);
  });
}
