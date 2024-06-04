// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/impl_trait.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('StructWithTraitTwinNormal', () async {
    expect(
        (await StructWithTraitTwinNormal.simpleTraitFnTwinNormal()).value, 42);
    expect(
        await StructWithTraitTwinNormal
            .simpleTraitFnWithDefaultImplTwinNormal(),
        42);
  });
}
