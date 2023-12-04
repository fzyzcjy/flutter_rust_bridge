// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync"]}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(
            rustAutoOpaqueArgOwnTwinSync(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(
            rustAutoOpaqueArgOwnTwinSync(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(
            rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(
            rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSync(
            rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSync(
            rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);

        await futurizeVoidTwinSync(
            rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100));

        await futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinSync(
            rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await Future.wait([
          futurizeVoidTwinSync(
              rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100)),
          futurizeVoidTwinSync(
              rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSync(
      //           rustAutoOpaqueArgOwnTwinSync(arg: obj, expect: 100)),
      //       futurizeVoidTwinSync(
      //           rustAutoOpaqueArgOwnTwinSync(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSync(
      //           rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 100)),
      //       futurizeVoidTwinSync(rustAutoOpaqueArgMutBorrowTwinSync(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinSync', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinSync(arg: a);

      await futurizeVoidTwinSync(
          rustAutoOpaqueArgOwnTwinSync(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinSync', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinSync(initial: 20);

      await futurizeVoidTwinSync(rustAutoOpaqueTwoArgsTwinSync(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinSync', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 42);

      await futurizeVoidTwinSync(
          rustAutoOpaqueNormalAndOpaqueArgTwinSync(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinSync();
      await futurizeVoidTwinSync(rustAutoOpaquePlusSignArgTwinSync(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinSync();
      await futurizeVoidTwinSync(rustAutoOpaqueCallableArgTwinSync(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(RwLockBoxHelloTraitTwinSync obj, String expect) async {
      await rustAutoOpaqueTraitObjectArgBorrowTwinSync(
          arg: obj, expect: expect);
      await rustAutoOpaqueTraitObjectArgMutBorrowTwinSync(
          arg: obj, expect: expect);
      await rustAutoOpaqueTraitObjectArgOwnTwinSync(arg: obj, expect: expect);
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinSync(), 'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSync(), 'B'));
  });

  test('static method', () async {
    final obj =
        await RwLockNonCloneSimpleTwinSync.staticMethodReturnOwnTwinSync();
    await futurizeVoidTwinSync(
        RwLockNonCloneSimpleTwinSync.staticMethodArgBorrowTwinSync(arg: obj));
    await futurizeVoidTwinSync(
        RwLockNonCloneSimpleTwinSync.staticMethodArgMutBorrowTwinSync(
            arg: obj));
    await futurizeVoidTwinSync(
        RwLockNonCloneSimpleTwinSync.staticMethodArgOwnTwinSync(arg: obj));
  });

  test('instance method', () async {
    final obj = await RwLockNonCloneSimpleTwinSync.newTwinSync();
    await futurizeVoidTwinSync(obj.instanceMethodArgBorrowTwinSync());
    await futurizeVoidTwinSync(obj.instanceMethodArgMutBorrowTwinSync());
    await futurizeVoidTwinSync(obj.instanceMethodReturnOwnTwinSync());
    await futurizeVoidTwinSync(obj.instanceMethodArgOwnTwinSync());
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSync();
    await futurizeVoidTwinSync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinSync(arg: obj));
    await futurizeVoidTwinSync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinSync(
            arg: obj));
    await futurizeVoidTwinSync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSync(arg: obj));
  });
}
