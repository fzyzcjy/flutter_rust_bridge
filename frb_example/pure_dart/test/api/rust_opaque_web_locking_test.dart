// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart/src/rust/api/rust_opaque_web_locking.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('Rust opaque Web locking', () {
    group('RustOpaque', () {
      test(
        'browser event-loop sync call fails fast instead of deadlocking',
        () async {
          final obj = await rustOpaqueWebLockingCreateTwinNormal(initial: 100);

          final _ = rustOpaqueWebLockingHoldMutBorrowForeverTwinNormal(
            arg: obj,
          );
          await Future<void>.delayed(const Duration(milliseconds: 100));

          expect(
            () => rustOpaqueWebLockingSyncAddTwinNormal(arg: obj, adder: 1),
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
        final obj = await rustOpaqueWebLockingCreateTwinNormal(initial: 100);

        await Future.wait([
          rustOpaqueWebLockingWorkerAddTwinNormal(
            arg: obj,
            adder: 1,
            delayMillis: 100,
          ),
          rustOpaqueWebLockingWorkerAddTwinNormal(
            arg: obj,
            adder: 10,
            delayMillis: 0,
          ),
        ]);

        expect(await rustOpaqueWebLockingGetTwinNormal(arg: obj), 111);
      });

      test('async calls can wait for the same object lock', () async {
        final obj = await rustOpaqueWebLockingCreateTwinNormal(initial: 100);

        await Future.wait([
          rustOpaqueWebLockingAsyncAddTwinNormal(arg: obj, adder: 1),
          rustOpaqueWebLockingAsyncAddTwinNormal(arg: obj, adder: 10),
        ]);

        expect(await rustOpaqueWebLockingGetTwinNormal(arg: obj), 111);
      });
    });

    group('RustAutoOpaque', () {
      test(
        'browser event-loop sync call fails fast instead of deadlocking',
        () async {
          final obj = await rustAutoOpaqueWebLockingCreateTwinNormal(
            initial: 100,
          );

          final _ = rustAutoOpaqueWebLockingHoldMutBorrowForeverTwinNormal(
            arg: obj,
          );
          await Future<void>.delayed(const Duration(milliseconds: 100));

          expect(
            () => rustAutoOpaqueWebLockingSyncAddTwinNormal(arg: obj, adder: 1),
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
        final obj = await rustAutoOpaqueWebLockingCreateTwinNormal(
          initial: 100,
        );

        await Future.wait([
          rustAutoOpaqueWebLockingWorkerAddTwinNormal(
            arg: obj,
            adder: 1,
            delayMillis: 100,
          ),
          rustAutoOpaqueWebLockingWorkerAddTwinNormal(
            arg: obj,
            adder: 10,
            delayMillis: 0,
          ),
        ]);

        expect(await rustAutoOpaqueWebLockingGetTwinNormal(arg: obj), 111);
      });

      test('async calls can wait for the same object lock', () async {
        final obj = await rustAutoOpaqueWebLockingCreateTwinNormal(
          initial: 100,
        );

        await Future.wait([
          rustAutoOpaqueWebLockingAsyncAddTwinNormal(arg: obj, adder: 1),
          rustAutoOpaqueWebLockingAsyncAddTwinNormal(arg: obj, adder: 10),
        ]);

        expect(await rustAutoOpaqueWebLockingGetTwinNormal(arg: obj), 111);
      });
    });
  });
}
