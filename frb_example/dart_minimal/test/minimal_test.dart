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

    test('dispose ownedStruct - dispose typeWithLifetime', () async {
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

    test('dispose typeWithLifetime - dispose ownedStruct', () async {
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

    test('dispose ownedStruct - dispose typeWithLifetime', () async {
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

    test(
        'dispose ownedStruct - dispose typeWithLifetime - dispose nestedTypeWithLifetime',
        () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal();
      final nestedTypeWithLifetime = await typeWithLifetime
          .computeNestedTypeWithLifetimeTwinNormal(arg: ownedStruct);

      ownedStruct.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        // Do *not* really dispose
      ]);

      typeWithLifetime.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        // Do *not* really dispose
      ]);

      nestedTypeWithLifetime.dispose();
      expect(ltGetAndResetLogsTwinNormal(), <String>[
        // NOTE the order
        'LtNestedTypeWithLifetimeTwinNormal.drop',
        'LtTypeWithLifetimeTwinNormal.drop',
        'LtOwnedStructTwinNormal.drop',
      ]);
    });
  });

  group('features', () {
    test('computeTypeWithLifetimeTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal();
      await _testTypeWithLifetime(ownedStruct, typeWithLifetime);
    });

    test('computeWithUnrelatedBorrowedArgTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeWithUnrelatedBorrowedArgTwinNormal(
        unrelatedBorrowed: await LtOwnedStructTwinNormal.create(value: 'hi'),
        unrelatedOwned: await LtOwnedStructTwinNormal.create(value: 'hi'),
      );
      await _testTypeWithLifetime(ownedStruct, typeWithLifetime);
    });

    test('ltComputeWithLifetimeFunctionTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ltComputeWithLifetimeFunctionTwinNormal(arg: ownedStruct);
      await _testTypeWithLifetime(ownedStruct, typeWithLifetime);
    });

    test('computeNestedTypeWithLifetimeTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal();
      final nestedTypeWithLifetime =
          await typeWithLifetime.computeNestedTypeWithLifetimeTwinNormal();
      await _testNestedTypeWithLifetime(
          ownedStruct, typeWithLifetime, nestedTypeWithLifetime);
    });

    test('computeArgGenericLifetimeTwinNormal', () async {
      final ownedStruct = await LtOwnedStructTwinNormal.create(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinNormal(arg: ownedStruct);
      final nestedTypeWithLifetime = await LtTypeWithLifetimeTwinNormal
          .computeArgGenericLifetimeTwinNormal(arg: typeWithLifetime);
      await _testNestedTypeWithLifetime(
          ownedStruct, typeWithLifetime, nestedTypeWithLifetime);
    });
  });
}

Future<void> _testTypeWithLifetime(
  LtOwnedStructTwinNormal ownedStruct,
  LtTypeWithLifetimeTwinNormal typeWithLifetime,
) async {
  expect(await typeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
  expect(await typeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');
  ownedStruct.dispose();
  expect(await typeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
  expect(await typeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');
}

Future<void> _testNestedTypeWithLifetime(
  LtOwnedStructTwinNormal ownedStruct,
  LtTypeWithLifetimeTwinNormal typeWithLifetime,
  LtNestedTypeWithLifetimeTwinNormal nestedTypeWithLifetime,
) async {
  expect(await typeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
  expect(await typeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');

  expect(await nestedTypeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
  expect(await nestedTypeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');

  ownedStruct.dispose();
  typeWithLifetime.dispose();

  // can still call
  expect(await nestedTypeWithLifetime.greetBorrowSelfTwinNormal(), 'a');
  expect(await nestedTypeWithLifetime.greetBorrowMutSelfTwinNormal(), 'a');
}
