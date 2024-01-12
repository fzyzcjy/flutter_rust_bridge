// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_rust_async_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgOwnTwinRustAsyncSse(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgOwnTwinRustAsyncSse(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () =>
                rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
                arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
                arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
                arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);

        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100));

        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
                arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await Future.wait([
          futurizeVoidTwinRustAsyncSse(
              rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100)),
          futurizeVoidTwinRustAsyncSse(
              rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncSse(
      //           rustAutoOpaqueArgOwnTwinRustAsyncSse(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsyncSse(
      //           rustAutoOpaqueArgOwnTwinRustAsyncSse(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncSse(rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinRustAsyncSse(rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncSse(
      //           rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsyncSse(rustAutoOpaqueArgMutBorrowTwinRustAsyncSse(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncSse(arg: a);

      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueArgOwnTwinRustAsyncSse(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinRustAsyncSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 20);

      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueTwoArgsTwinRustAsyncSse(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncSse', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 42);

      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncSse(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinRustAsyncSse();
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaquePlusSignArgTwinRustAsyncSse(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinRustAsyncSse();
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueCallableArgTwinRustAsyncSse(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(BoxHelloTraitTwinRustAsyncSse obj, String expect) async {
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueTraitObjectArgBorrowTwinRustAsyncSse(
              arg: obj, expect: expect));
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsyncSse(
              arg: obj, expect: expect));
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueTraitObjectArgOwnTwinRustAsyncSse(
              arg: obj, expect: expect));
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsyncSse(),
            'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsyncSse(),
            'B'));
  });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinRustAsyncSse
        .staticMethodReturnOwnTwinRustAsyncSse();
    await futurizeVoidTwinRustAsyncSse(
        NonCloneSimpleTwinRustAsyncSse.staticMethodArgBorrowTwinRustAsyncSse(
            arg: obj));
    await futurizeVoidTwinRustAsyncSse(
        NonCloneSimpleTwinRustAsyncSse.staticMethodArgMutBorrowTwinRustAsyncSse(
            arg: obj));
    await futurizeVoidTwinRustAsyncSse(
        NonCloneSimpleTwinRustAsyncSse.staticMethodArgOwnTwinRustAsyncSse(
            arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinRustAsyncSse.newTwinRustAsyncSse();
    await futurizeVoidTwinRustAsyncSse(
        obj.instanceMethodArgBorrowTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        obj.instanceMethodArgMutBorrowTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        obj.instanceMethodReturnOwnTwinRustAsyncSse());
    await futurizeVoidTwinRustAsyncSse(
        obj.instanceMethodArgOwnTwinRustAsyncSse());
  });
  test('instance newWithResult', () async {
    final obj =
        await NonCloneSimpleTwinRustAsyncSse.newWithResultTwinRustAsyncSse();
    await futurizeVoidTwinRustAsyncSse(
        obj.instanceMethodArgBorrowTwinRustAsyncSse());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinRustAsyncSse.newTwinRustAsyncSse();
    expect(await obj.instanceMethodGetterTwinRustAsyncSse, 42);
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsyncSse();
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinRustAsyncSse(
            arg: obj));
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinRustAsyncSse(
            arg: obj));
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsyncSse(
            arg: obj));
  });
}
