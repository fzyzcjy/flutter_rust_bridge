import 'dart:async';

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

  // ----------------------------------------------------------------

  setUp(ltGetAndResetLogsTwinNormal);

  group('when to dispose and the dispose order', () {
    test('dispose ownedStruct', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      expect(ltGetAndResetLogsTwinNormal(), <String>[]);

      ownedStruct.dispose();
      expect(ltGetAndResetLogsTwinNormal(),
          <String>['LtOwnedStructTwinNormal.drop']);
    });

    test('dispose ownedStruct - dispose subStruct', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal();

      ownedStruct.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        // Do *not* really dispose ownedStruct
      ]);

      typeWithLifetime.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        // NOTE order: Firstly the borrowed type, secondly the owned type
        'LtTypeWithLifetimeTwinNormal.drop',
        'LtOwnedStructTwinNormal.drop',
      ]);
    });

    test('dispose subStruct - dispose ownedStruct', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal();

      typeWithLifetime.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        'LtTypeWithLifetimeTwinNormal.drop',
      ]);

      ownedStruct.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        'LtOwnedStructTwinNormal.drop',
      ]);
    });
  });

  group('features', () {
    test('computeTypeWithLifetimeTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal();

      expect(await typeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
      expect(await typeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');

      ownedStruct.dispose();

      // can still call
      expect(await typeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
      expect(await typeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');
    });

    test('computeSubStructTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final subStruct = await ownedStruct.computeSubStructTwinNormal();

      expect(await subStruct.greetBorrowSelfTwinNormal(), 'a');
      expect(await subStruct.greetBorrowMutSelfTwinNormal(), 'a');

      ownedStruct.dispose();

      // can still call
      expect(await subStruct.greetBorrowSelfTwinNormal(), 'a');
      expect(await subStruct.greetBorrowMutSelfTwinNormal(), 'a');
    });
  });
}
