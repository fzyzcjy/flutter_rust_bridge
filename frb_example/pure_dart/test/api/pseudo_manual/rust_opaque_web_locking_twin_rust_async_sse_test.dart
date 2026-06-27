// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque_web_locking_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_opaque_web_locking_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('Rust opaque Web locking', () {
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
}
