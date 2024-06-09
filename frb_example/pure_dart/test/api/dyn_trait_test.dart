// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/dyn_trait.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('support &dyn TraitType', () async {
    final one =
        await StructOneWithTraitForDynTwinNormal.createTwinNormal(one: 10);
    final two =
        await StructTwoWithTraitForDynTwinNormal.createTwinNormal(two: 100);
    expect(await funcArgDynTraitTwinNormal(arg: one), 10);
    expect(await funcArgDynTraitTwinNormal(arg: two), 200);
  });
}
