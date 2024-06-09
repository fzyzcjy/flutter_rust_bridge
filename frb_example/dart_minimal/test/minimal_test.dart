import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  print('Action: Configure tests (end)');

  test('support &dyn TraitType', () async {
    final one =
        await StructOneWithTraitForDynTwinNormal.createTwinNormal(one: 10);
    final two =
        await StructTwoWithTraitForDynTwinNormal.createTwinNormal(two: 100);
    expect(await funcArgDynTraitTwinNormal(arg: one), 10);
    expect(await funcArgDynTraitTwinNormal(arg: two), 200);
  });
}
