// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

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

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinSse obj, String expect) async {
  //     await futurizeVoidTwinSse(rustAutoOpaqueTraitObjectArgBorrowTwinSse(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinSse(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinSse(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinSse(
  //         rustAutoOpaqueTraitObjectArgOwnTwinSse(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinSse(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSse(), 'B'));
  // });

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

  test('getter', () async {
    final obj = await NonCloneSimpleTwinSse.newTwinSse();
    expect(await obj.instanceMethodGetterTwinSse, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSse();
    expect(obj.good, 'hello');
    await futurizeVoidTwinSse(
        rustAutoOpaqueArgBorrowTwinSse(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinSse(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSse(arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSse());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSse());

    await futurizeVoidTwinSse(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSse(arg: good));
    await futurizeVoidTwinSse(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSse(arg: opaque));

    await futurizeVoidTwinSse(rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSse(
        arg: EnumWithGoodAndOpaqueTwinSse.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinSse();
    await futurizeVoidTwinSse(rustAutoOpaqueEnumArgBorrowTwinSse(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinSse();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinSse(
        rustAutoOpaqueArgBorrowTwinSse(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinSse();

    expect(vec.length, 2);
    await futurizeVoidTwinSse(
        rustAutoOpaqueArgBorrowTwinSse(arg: vec[0], expect: 10));
    await futurizeVoidTwinSse(
        rustAutoOpaqueArgBorrowTwinSse(arg: vec[1], expect: 20));

    await futurizeVoidTwinSse(
        rustAutoOpaqueArgVecOwnTwinSse(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSse(initial: 100);
      await futurizeVoidTwinSse(
          rustAutoOpaqueExplicitArgTwinSse(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSse(initial: 100);
      await futurizeVoidTwinSse(rustAutoOpaqueExplicitStructTwinSse(
          arg: StructWithExplicitAutoOpaqueFieldTwinSse(
              autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinSse();
      await futurizeVoidTwinSse(rustAutoOpaqueExplicitStructTwinSse(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj = await rustAutoOpaqueExplicitReturnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueArgOwnTwinSse(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
        await futurizeVoidTwinSse(
            rustAutoOpaqueExplicitArgTwinSse(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinSse(
            borrow: obj, mutBorrow: obj),
        'TwinSse',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSse(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinSse(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSse(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSse(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSse(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSse(initial: 200);
      expect(await rustAutoOpaqueSleepTwinSse(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSse(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSse(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSse(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSse(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSse(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSse(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSse(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
