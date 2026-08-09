// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_web_locking_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_web_locking_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('Rust opaque Web locking', () {
    group('RustOpaque', () {
      test(
        'browser event-loop sync call fails fast instead of deadlocking',
        () async {
          final obj = await rustOpaqueWebLockingCreateTwinSse(initial: 100);

          final _ = rustOpaqueWebLockingHoldMutBorrowForeverTwinSse(
            arg: obj,
          );
          await Future<void>.delayed(const Duration(milliseconds: 100));

          expect(
            () => rustOpaqueWebLockingSyncAddTwinSse(arg: obj, adder: 1),
            throwsA(
              predicate(
                (Object error) => error
                    .toString()
                    .contains('cannot synchronously write rust opaque objects'),
              ),
            ),
          );
        },
        skip: !kIsWeb ? 'Web-only regression coverage' : null,
      );

      test('worker-pool calls can wait for the same object lock', () async {
        final obj = await rustOpaqueWebLockingCreateTwinSse(initial: 100);

        await Future.wait([
          rustOpaqueWebLockingWorkerAddTwinSse(
            arg: obj,
            adder: 1,
            delayMillis: 100,
          ),
          rustOpaqueWebLockingWorkerAddTwinSse(
            arg: obj,
            adder: 10,
            delayMillis: 0,
          ),
        ]);

        expect(await rustOpaqueWebLockingGetTwinSse(arg: obj), 111);
      });

      test('async calls can wait for the same object lock', () async {
        final obj = await rustOpaqueWebLockingCreateTwinSse(initial: 100);

        await Future.wait([
          rustOpaqueWebLockingAsyncAddTwinSse(arg: obj, adder: 1),
          rustOpaqueWebLockingAsyncAddTwinSse(arg: obj, adder: 10),
        ]);

        expect(await rustOpaqueWebLockingGetTwinSse(arg: obj), 111);
      });
    });

    group('RustAutoOpaque', () {
      test(
        'browser event-loop sync call fails fast instead of deadlocking',
        () async {
          final obj = await rustAutoOpaqueWebLockingCreateTwinSse(
            initial: 100,
          );

          final _ = rustAutoOpaqueWebLockingHoldMutBorrowForeverTwinSse(
            arg: obj,
          );
          await Future<void>.delayed(const Duration(milliseconds: 100));

          expect(
            () => rustAutoOpaqueWebLockingSyncAddTwinSse(arg: obj, adder: 1),
            throwsA(
              predicate(
                (Object error) => error
                    .toString()
                    .contains('cannot synchronously write rust opaque objects'),
              ),
            ),
          );
        },
        skip: !kIsWeb ? 'Web-only regression coverage' : null,
      );

      test('worker-pool calls can wait for the same object lock', () async {
        final obj = await rustAutoOpaqueWebLockingCreateTwinSse(
          initial: 100,
        );

        await Future.wait([
          rustAutoOpaqueWebLockingWorkerAddTwinSse(
            arg: obj,
            adder: 1,
            delayMillis: 100,
          ),
          rustAutoOpaqueWebLockingWorkerAddTwinSse(
            arg: obj,
            adder: 10,
            delayMillis: 0,
          ),
        ]);

        expect(await rustAutoOpaqueWebLockingGetTwinSse(arg: obj), 111);
      });

      test('async calls can wait for the same object lock', () async {
        final obj = await rustAutoOpaqueWebLockingCreateTwinSse(
          initial: 100,
        );

        await Future.wait([
          rustAutoOpaqueWebLockingAsyncAddTwinSse(arg: obj, adder: 1),
          rustAutoOpaqueWebLockingAsyncAddTwinSse(arg: obj, adder: 10),
        ]);

        expect(await rustAutoOpaqueWebLockingGetTwinSse(arg: obj), 111);
      });
    });
  });
}
