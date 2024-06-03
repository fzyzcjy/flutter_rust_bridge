import 'package:frb_example_pure_dart/src/rust/api/impl_trait.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('SimpleTraitTwinNormal', () async {
    expect((await SimpleTraitTwinNormal.simpleTraitFnTwinNormal()).value, 42);
    expect(await SimpleTraitTwinNormal.simpleTraitFnWithDefaultImplTwinNormal(),
        42);
  });
}
