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

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinRustAsyncSse obj, String expect) async {
  //     await futurizeVoidTwinRustAsyncSse(rustAutoOpaqueTraitObjectArgBorrowTwinRustAsyncSse(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinRustAsyncSse(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsyncSse(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinRustAsyncSse(
  //         rustAutoOpaqueTraitObjectArgOwnTwinRustAsyncSse(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsyncSse(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsyncSse(), 'B'));
  // });

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

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsyncSse();
    expect(obj.good, 'hello');
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsyncSse(
            arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinRustAsyncSse());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinRustAsyncSse());

    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncSse(arg: good));
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncSse(arg: opaque));

    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncSse(
            arg: EnumWithGoodAndOpaqueTwinRustAsyncSse.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinRustAsyncSse();
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueEnumArgBorrowTwinRustAsyncSse(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinRustAsyncSse();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinRustAsyncSse();

    expect(vec.length, 2);
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: vec[0], expect: 10));
    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueArgBorrowTwinRustAsyncSse(arg: vec[1], expect: 20));

    await futurizeVoidTwinRustAsyncSse(
        rustAutoOpaqueArgVecOwnTwinRustAsyncSse(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj =
          await rustAutoOpaqueExplicitReturnTwinRustAsyncSse(initial: 100);
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueExplicitArgTwinRustAsyncSse(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj =
          await rustAutoOpaqueExplicitReturnTwinRustAsyncSse(initial: 100);
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueExplicitStructTwinRustAsyncSse(
              arg: StructWithExplicitAutoOpaqueFieldTwinRustAsyncSse(
                  autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinRustAsyncSse();
      await futurizeVoidTwinRustAsyncSse(
          rustAutoOpaqueExplicitStructTwinRustAsyncSse(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj =
            await rustAutoOpaqueExplicitReturnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueArgOwnTwinRustAsyncSse(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
        await futurizeVoidTwinRustAsyncSse(
            rustAutoOpaqueExplicitArgTwinRustAsyncSse(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncSse(
            borrow: obj, mutBorrow: obj),
        'TwinRustAsyncSse',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncSse(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinRustAsyncSse(a: obj, b: obj),
          200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinRustAsyncSse(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 200);
      expect(
          await rustAutoOpaqueSleepTwinRustAsyncSse(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinRustAsyncSse(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinRustAsyncSse(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncSse(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinRustAsyncSse(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinRustAsyncSse(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
