// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse"]}

import 'package:frb_example_pure_dart_pde/src/rust/api/rust_opaque_web_locking.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('Rust opaque Web locking', () {
    group('RustOpaque', () {
      test(
        'browser event-loop sync call fails fast instead of deadlocking',
        () async {
          final obj = await rustOpaqueWebLockingCreate(initial: 100);

          final _ = rustOpaqueWebLockingHoldMutBorrowForever(arg: obj);
          await Future<void>.delayed(const Duration(milliseconds: 100));

          expect(
            () => rustOpaqueWebLockingSyncAdd(arg: obj, adder: 1),
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
        final obj = await rustOpaqueWebLockingCreate(initial: 100);

        await Future.wait([
          rustOpaqueWebLockingWorkerAdd(
            arg: obj,
            adder: 1,
            delayMillis: 100,
          ),
          rustOpaqueWebLockingWorkerAdd(
            arg: obj,
            adder: 10,
            delayMillis: 0,
          ),
        ]);

        expect(await rustOpaqueWebLockingGet(arg: obj), 111);
      });

      test('async calls can wait for the same object lock', () async {
        final obj = await rustOpaqueWebLockingCreate(initial: 100);

        await Future.wait([
          rustOpaqueWebLockingAsyncAdd(arg: obj, adder: 1),
          rustOpaqueWebLockingAsyncAdd(arg: obj, adder: 10),
        ]);

        expect(await rustOpaqueWebLockingGet(arg: obj), 111);
      });
    });

    group('RustAutoOpaque', () {
      test(
        'browser event-loop sync call fails fast instead of deadlocking',
        () async {
          final obj = await rustAutoOpaqueWebLockingCreate(initial: 100);

          final _ = rustAutoOpaqueWebLockingHoldMutBorrowForever(arg: obj);
          await Future<void>.delayed(const Duration(milliseconds: 100));

          expect(
            () => rustAutoOpaqueWebLockingSyncAdd(arg: obj, adder: 1),
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
        final obj = await rustAutoOpaqueWebLockingCreate(initial: 100);

        await Future.wait([
          rustAutoOpaqueWebLockingWorkerAdd(
            arg: obj,
            adder: 1,
            delayMillis: 100,
          ),
          rustAutoOpaqueWebLockingWorkerAdd(
            arg: obj,
            adder: 10,
            delayMillis: 0,
          ),
        ]);

        expect(await rustAutoOpaqueWebLockingGet(arg: obj), 111);
      });

      test('async calls can wait for the same object lock', () async {
        final obj = await rustAutoOpaqueWebLockingCreate(initial: 100);

        await Future.wait([
          rustAutoOpaqueWebLockingAsyncAdd(arg: obj, adder: 1),
          rustAutoOpaqueWebLockingAsyncAdd(arg: obj, adder: 10),
        ]);

        expect(await rustAutoOpaqueWebLockingGet(arg: obj), 111);
      });
    });
  });
}
