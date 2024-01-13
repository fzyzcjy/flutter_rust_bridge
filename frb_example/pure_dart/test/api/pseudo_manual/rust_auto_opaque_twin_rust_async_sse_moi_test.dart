// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_rust_async_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj =
          await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgOwnTwinRustAsyncSseMoi(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgOwnTwinRustAsyncSseMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(
                arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
                arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
                arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
                arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);

        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(arg: obj, expect: 100));

        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
                arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinRustAsyncSseMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(
                arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj =
            await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
        await Future.wait([
          futurizeVoidTwinRustAsyncSseMoi(
              rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(
                  arg: obj, expect: 100)),
          futurizeVoidTwinRustAsyncSseMoi(
              rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(
                  arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncSseMoi(
      //           rustAutoOpaqueArgOwnTwinRustAsyncSseMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsyncSseMoi(
      //           rustAutoOpaqueArgOwnTwinRustAsyncSseMoi(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncSseMoi(rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinRustAsyncSseMoi(rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncSseMoi(
      //           rustAutoOpaqueArgBorrowTwinRustAsyncSseMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsyncSseMoi(rustAutoOpaqueArgMutBorrowTwinRustAsyncSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 42);

      final b =
          await rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncSseMoi(arg: a);

      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueArgOwnTwinRustAsyncSseMoi(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinRustAsyncSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 20);

      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueTwoArgsTwinRustAsyncSseMoi(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 42);

      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncSseMoi(
              a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinRustAsyncSseMoi();
      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaquePlusSignArgTwinRustAsyncSseMoi(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinRustAsyncSseMoi();
      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueCallableArgTwinRustAsyncSseMoi(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(
        BoxHelloTraitTwinRustAsyncSseMoi obj, String expect) async {
      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueTraitObjectArgBorrowTwinRustAsyncSseMoi(
              arg: obj, expect: expect));
      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsyncSseMoi(
              arg: obj, expect: expect));
      await futurizeVoidTwinRustAsyncSseMoi(
          rustAutoOpaqueTraitObjectArgOwnTwinRustAsyncSseMoi(
              arg: obj, expect: expect));
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsyncSseMoi(),
            'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsyncSseMoi(),
            'B'));
  });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinRustAsyncSseMoi
        .staticMethodReturnOwnTwinRustAsyncSseMoi();
    await futurizeVoidTwinRustAsyncSseMoi(NonCloneSimpleTwinRustAsyncSseMoi
        .staticMethodArgBorrowTwinRustAsyncSseMoi(arg: obj));
    await futurizeVoidTwinRustAsyncSseMoi(NonCloneSimpleTwinRustAsyncSseMoi
        .staticMethodArgMutBorrowTwinRustAsyncSseMoi(arg: obj));
    await futurizeVoidTwinRustAsyncSseMoi(
        NonCloneSimpleTwinRustAsyncSseMoi.staticMethodArgOwnTwinRustAsyncSseMoi(
            arg: obj));
  });

  test('instance method', () async {
    final obj =
        await NonCloneSimpleTwinRustAsyncSseMoi.newTwinRustAsyncSseMoi();
    await futurizeVoidTwinRustAsyncSseMoi(
        obj.instanceMethodArgBorrowTwinRustAsyncSseMoi());
    await futurizeVoidTwinRustAsyncSseMoi(
        obj.instanceMethodArgMutBorrowTwinRustAsyncSseMoi());
    await futurizeVoidTwinRustAsyncSseMoi(
        obj.instanceMethodReturnOwnTwinRustAsyncSseMoi());
    await futurizeVoidTwinRustAsyncSseMoi(
        obj.instanceMethodArgOwnTwinRustAsyncSseMoi());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinRustAsyncSseMoi
        .newWithResultTwinRustAsyncSseMoi();
    await futurizeVoidTwinRustAsyncSseMoi(
        obj.instanceMethodArgBorrowTwinRustAsyncSseMoi());
  });

  test('getter', () async {
    final obj =
        await NonCloneSimpleTwinRustAsyncSseMoi.newTwinRustAsyncSseMoi();
    expect(await obj.instanceMethodGetterTwinRustAsyncSseMoi, 42);
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsyncSseMoi();
    await futurizeVoidTwinRustAsyncSseMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinRustAsyncSseMoi(
            arg: obj));
    await futurizeVoidTwinRustAsyncSseMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinRustAsyncSseMoi(
            arg: obj));
    await futurizeVoidTwinRustAsyncSseMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsyncSseMoi(
            arg: obj));
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj =
          await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncSseMoi(
            borrow: obj, mutBorrow: obj),
        'TwinRustAsyncSseMoi',
        messageMatcherOnNative: matches(RegExp('Fail to.*borrow object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncSseMoi(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj =
          await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinRustAsyncSseMoi(
              a: obj, b: obj),
          200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSseMoi(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinRustAsyncSseMoi(a: a, b: b),
          300);
    });
  });
}
