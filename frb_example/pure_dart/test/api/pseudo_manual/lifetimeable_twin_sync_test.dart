// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `lifetimeable_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "sse", "sync sse", "rustAsync sse"], "replaceCode": {"misc_no_twin_example_a": "misc_no_twin_example_a"}}

import 'dart:async';

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/lifetimeable_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/api/misc_no_twin_example_a.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('when to dispose and the dispose order', () {
    test('dispose ownedStruct', () async {
      final simpleLogger = SimpleLogger();
      final ownedStruct = await LtOwnedStructTwinSync.createWithLoggerTwinSync(
          value: 'a', logger: simpleLogger);
      expect(simpleLogger.getAndReset(), <String>[]);

      ownedStruct.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        'LtOwnedStructTwinSync.drop',
        'LtOwnedSubStructTwinSync.drop',
      ]);
    });

    test('dispose ownedStruct - dispose typeWithLifetime', () async {
      final simpleLogger = SimpleLogger();
      final ownedStruct = await LtOwnedStructTwinSync.createWithLoggerTwinSync(
          value: 'a', logger: simpleLogger);
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();

      ownedStruct.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // Do *not* really dispose ownedStruct
      ]);

      typeWithLifetime.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // NOTE order: Firstly the borrowed type, secondly the owned type
        'LtTypeWithLifetimeTwinSync.drop',
        'LtOwnedStructTwinSync.drop',
        'LtOwnedSubStructTwinSync.drop',
      ]);
    });

    test('dispose typeWithLifetime - dispose ownedStruct', () async {
      final simpleLogger = SimpleLogger();
      final ownedStruct = await LtOwnedStructTwinSync.createWithLoggerTwinSync(
          value: 'a', logger: simpleLogger);
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();

      typeWithLifetime.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        'LtTypeWithLifetimeTwinSync.drop',
      ]);

      ownedStruct.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        'LtOwnedStructTwinSync.drop',
        'LtOwnedSubStructTwinSync.drop',
      ]);
    });

    test('dispose ownedStruct - dispose typeWithLifetime', () async {
      final simpleLogger = SimpleLogger();
      final ownedStruct = await LtOwnedStructTwinSync.createWithLoggerTwinSync(
          value: 'a', logger: simpleLogger);
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();

      ownedStruct.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // Do *not* really dispose ownedStruct
      ]);

      typeWithLifetime.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // NOTE order: Firstly the borrowed type, secondly the owned type
        'LtTypeWithLifetimeTwinSync.drop',
        'LtOwnedStructTwinSync.drop',
        'LtOwnedSubStructTwinSync.drop',
      ]);
    });

    test(
        'dispose ownedStruct - dispose typeWithLifetime - dispose nestedTypeWithLifetime',
        () async {
      final simpleLogger = SimpleLogger();
      final ownedStruct = await LtOwnedStructTwinSync.createWithLoggerTwinSync(
          value: 'a', logger: simpleLogger);
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();
      final nestedTypeWithLifetime =
          await typeWithLifetime.computeNestedTypeWithLifetimeTwinSync();

      ownedStruct.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // Do *not* really dispose
      ]);

      typeWithLifetime.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // Do *not* really dispose
      ]);

      nestedTypeWithLifetime.dispose();
      expect(simpleLogger.getAndReset(), <String>[
        // NOTE the order
        'LtNestedTypeWithLifetimeTwinSync.drop',
        'LtTypeWithLifetimeTwinSync.drop',
        'LtOwnedStructTwinSync.drop',
        'LtOwnedSubStructTwinSync.drop',
      ]);
    });
  });

  group('features', () {
    test('computeTypeWithLifetimeTwinSync', () async {
      final ownedStruct =
          await LtOwnedStructTwinSync.createTwinSync(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();
      await _testTypeWithLifetime(ownedStruct, typeWithLifetime);
    });

    test('computeWithUnrelatedBorrowedArgTwinSync', () async {
      final ownedStruct =
          await LtOwnedStructTwinSync.createTwinSync(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeWithUnrelatedBorrowedArgTwinSync(
        unrelatedBorrowed:
            await LtOwnedStructTwinSync.createTwinSync(value: 'hi'),
        unrelatedOwned: await LtOwnedStructTwinSync.createTwinSync(value: 'hi'),
      );
      await _testTypeWithLifetime(ownedStruct, typeWithLifetime);
    });

    test('ltComputeWithLifetimeFunctionTwinSync', () async {
      final ownedStruct =
          await LtOwnedStructTwinSync.createTwinSync(value: 'a');
      final typeWithLifetime =
          await ltComputeWithLifetimeFunctionTwinSync(arg: ownedStruct);
      await _testTypeWithLifetime(ownedStruct, typeWithLifetime);
    });

    test('computeNestedTypeWithLifetimeTwinSync', () async {
      final ownedStruct =
          await LtOwnedStructTwinSync.createTwinSync(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();
      final nestedTypeWithLifetime =
          await typeWithLifetime.computeNestedTypeWithLifetimeTwinSync();
      await _testNestedTypeWithLifetime(
          ownedStruct, typeWithLifetime, nestedTypeWithLifetime);
    });

    test('computeArgGenericLifetimeTwinSync', () async {
      final ownedStruct =
          await LtOwnedStructTwinSync.createTwinSync(value: 'a');
      final typeWithLifetime =
          await ownedStruct.computeTypeWithLifetimeTwinSync();
      final anotherTypeWithLifetime =
          await LtTypeWithLifetimeTwinSync.computeArgGenericLifetimeTwinSync(
              arg: typeWithLifetime);

      expect(await anotherTypeWithLifetime.greetBorrowSelfTwinSync(), 'a');
      expect(await anotherTypeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');

      ownedStruct.dispose();
      typeWithLifetime.dispose();

      expect(await anotherTypeWithLifetime.greetBorrowSelfTwinSync(), 'a');
      expect(await anotherTypeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');
    });

    test('computeWithMultiArgHavingLifetimeTwinSync', () async {
      final typeWithMultiDep = await LtTypeWithMultiDepTwinSync
          .computeWithMultiArgHavingLifetimeTwinSync(
        a: await LtOwnedStructTwinSync.createTwinSync(value: 'a'),
        b: await LtOwnedStructTwinSync.createTwinSync(value: 'b'),
        unrelatedBorrowed:
            await LtOwnedStructTwinSync.createTwinSync(value: 'hi'),
        unrelatedOwned: await LtOwnedStructTwinSync.createTwinSync(value: 'hi'),
      );
      expect(await typeWithMultiDep.greetBorrowSelfTwinSync(), ['a', 'b']);
      expect(await typeWithMultiDep.greetBorrowMutSelfTwinSync(), ['a', 'b']);
    });
  });
}

Future<void> _testTypeWithLifetime(
  LtOwnedStructTwinSync ownedStruct,
  LtTypeWithLifetimeTwinSync typeWithLifetime,
) async {
  expect(await typeWithLifetime.greetBorrowSelfTwinSync(), 'a');
  expect(await typeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');
  ownedStruct.dispose();
  expect(await typeWithLifetime.greetBorrowSelfTwinSync(), 'a');
  expect(await typeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');
}

Future<void> _testNestedTypeWithLifetime(
  LtOwnedStructTwinSync ownedStruct,
  LtTypeWithLifetimeTwinSync typeWithLifetime,
  LtNestedTypeWithLifetimeTwinSync nestedTypeWithLifetime,
) async {
  expect(await typeWithLifetime.greetBorrowSelfTwinSync(), 'a');
  // cannot mut borrow, since nestedTypeWithLifetime has borrowed typeWithLifetime
  // expect(await typeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');

  expect(await nestedTypeWithLifetime.greetBorrowSelfTwinSync(), 'a');
  expect(await nestedTypeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');

  ownedStruct.dispose();
  typeWithLifetime.dispose();

  // can still call
  expect(await nestedTypeWithLifetime.greetBorrowSelfTwinSync(), 'a');
  expect(await nestedTypeWithLifetime.greetBorrowMutSelfTwinSync(), 'a');
}
