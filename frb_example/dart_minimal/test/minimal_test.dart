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

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);
      expect(await rustAutoOpaqueSleepTwinNormal(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinNormal(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinNormal(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinNormal(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinNormal(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
