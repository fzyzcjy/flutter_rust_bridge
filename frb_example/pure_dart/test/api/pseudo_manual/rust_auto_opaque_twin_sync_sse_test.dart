// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsyncSse"]}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgOwnTwinSyncSse(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgOwnTwinSyncSse(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);

        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100));

        await futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinSyncSse(
            rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
        await Future.wait([
          futurizeVoidTwinSyncSse(
              rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100)),
          futurizeVoidTwinSyncSse(
              rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncSse(
      //           rustAutoOpaqueArgOwnTwinSyncSse(arg: obj, expect: 100)),
      //       futurizeVoidTwinSyncSse(
      //           rustAutoOpaqueArgOwnTwinSyncSse(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncSse(
      //           rustAutoOpaqueArgBorrowTwinSyncSse(arg: obj, expect: 100)),
      //       futurizeVoidTwinSyncSse(rustAutoOpaqueArgMutBorrowTwinSyncSse(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinSyncSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinSyncSse(arg: a);

      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueArgOwnTwinSyncSse(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinSyncSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 20);

      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueTwoArgsTwinSyncSse(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinSyncSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSse(initial: 42);

      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueNormalAndOpaqueArgTwinSyncSse(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinSyncSse();
      await futurizeVoidTwinSyncSse(
          rustAutoOpaquePlusSignArgTwinSyncSse(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinSyncSse();
      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueCallableArgTwinSyncSse(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(BoxHelloTraitTwinSyncSse obj, String expect) async {
      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueTraitObjectArgBorrowTwinSyncSse(
              arg: obj, expect: expect));
      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueTraitObjectArgMutBorrowTwinSyncSse(
              arg: obj, expect: expect));
      await futurizeVoidTwinSyncSse(
          rustAutoOpaqueTraitObjectArgOwnTwinSyncSse(arg: obj, expect: expect));
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinSyncSse(), 'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSyncSse(), 'B'));
  });

  test('static method', () async {
    final obj =
        await NonCloneSimpleTwinSyncSse.staticMethodReturnOwnTwinSyncSse();
    await futurizeVoidTwinSyncSse(
        NonCloneSimpleTwinSyncSse.staticMethodArgBorrowTwinSyncSse(arg: obj));
    await futurizeVoidTwinSyncSse(
        NonCloneSimpleTwinSyncSse.staticMethodArgMutBorrowTwinSyncSse(
            arg: obj));
    await futurizeVoidTwinSyncSse(
        NonCloneSimpleTwinSyncSse.staticMethodArgOwnTwinSyncSse(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinSyncSse.newTwinSyncSse();
    await futurizeVoidTwinSyncSse(obj.instanceMethodArgBorrowTwinSyncSse());
    await futurizeVoidTwinSyncSse(obj.instanceMethodArgMutBorrowTwinSyncSse());
    await futurizeVoidTwinSyncSse(obj.instanceMethodReturnOwnTwinSyncSse());
    await futurizeVoidTwinSyncSse(obj.instanceMethodArgOwnTwinSyncSse());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinSyncSse.newWithResultTwinSyncSse();
    await futurizeVoidTwinSyncSse(obj.instanceMethodArgBorrowTwinSyncSse());
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSyncSse();
    await futurizeVoidTwinSyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinSyncSse(
            arg: obj));
    await futurizeVoidTwinSyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinSyncSse(
            arg: obj));
    await futurizeVoidTwinSyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSyncSse(arg: obj));
  });
}
