// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["rustAsync", "rustAsyncSse"]}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgOwnTwinSse(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgOwnTwinSse(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSse(
            rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgMutBorrowTwinSse(arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgMutBorrowTwinSse(arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSse(rustAutoOpaqueArgMutBorrowTwinSse(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSse(
            rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);

        await futurizeVoidTwinSse(
            rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100));

        await futurizeVoidTwinSse(
            rustAutoOpaqueArgMutBorrowTwinSse(arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await Future.wait([
          futurizeVoidTwinSse(
              rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100)),
          futurizeVoidTwinSse(
              rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSse(
      //           rustAutoOpaqueArgOwnTwinSse(arg: obj, expect: 100)),
      //       futurizeVoidTwinSse(
      //           rustAutoOpaqueArgOwnTwinSse(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSse(rustAutoOpaqueArgMutBorrowTwinSse(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinSse(rustAutoOpaqueArgMutBorrowTwinSse(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSse(
      //           rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 100)),
      //       futurizeVoidTwinSse(rustAutoOpaqueArgMutBorrowTwinSse(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinSse(arg: a);

      await futurizeVoidTwinSse(
          rustAutoOpaqueArgOwnTwinSse(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinSse(initial: 20);

      await futurizeVoidTwinSse(rustAutoOpaqueTwoArgsTwinSse(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 42);

      await futurizeVoidTwinSse(
          rustAutoOpaqueNormalAndOpaqueArgTwinSse(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinSse();
      await futurizeVoidTwinSse(rustAutoOpaquePlusSignArgTwinSse(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinSse();
      await futurizeVoidTwinSse(rustAutoOpaqueCallableArgTwinSse(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(BoxHelloTraitTwinSse obj, String expect) async {
      await futurizeVoidTwinSse(
          rustAutoOpaqueTraitObjectArgBorrowTwinSse(arg: obj, expect: expect));
      await futurizeVoidTwinSse(rustAutoOpaqueTraitObjectArgMutBorrowTwinSse(
          arg: obj, expect: expect));
      await futurizeVoidTwinSse(
          rustAutoOpaqueTraitObjectArgOwnTwinSse(arg: obj, expect: expect));
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinSse(), 'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSse(), 'B'));
  });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinSse.staticMethodReturnOwnTwinSse();
    await futurizeVoidTwinSse(
        NonCloneSimpleTwinSse.staticMethodArgBorrowTwinSse(arg: obj));
    await futurizeVoidTwinSse(
        NonCloneSimpleTwinSse.staticMethodArgMutBorrowTwinSse(arg: obj));
    await futurizeVoidTwinSse(
        NonCloneSimpleTwinSse.staticMethodArgOwnTwinSse(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinSse.newTwinSse();
    await futurizeVoidTwinSse(obj.instanceMethodArgBorrowTwinSse());
    await futurizeVoidTwinSse(obj.instanceMethodArgMutBorrowTwinSse());
    await futurizeVoidTwinSse(obj.instanceMethodReturnOwnTwinSse());
    await futurizeVoidTwinSse(obj.instanceMethodArgOwnTwinSse());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinSse.newWithResultTwinSse();
    await futurizeVoidTwinSse(obj.instanceMethodArgBorrowTwinSse());
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSse();
    await futurizeVoidTwinSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinSse(arg: obj));
    await futurizeVoidTwinSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinSse(
            arg: obj));
    await futurizeVoidTwinSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSse(arg: obj));
  });
}
