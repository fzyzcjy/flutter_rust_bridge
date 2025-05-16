// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_rust_async_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgOwnTwinRustAsyncMoi(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgOwnTwinRustAsyncMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () =>
                rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
                arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
                arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
                arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);

        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100));

        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
                arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await Future.wait([
          futurizeVoidTwinRustAsyncMoi(
              rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100)),
          futurizeVoidTwinRustAsyncMoi(
              rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncMoi(
      //           rustAutoOpaqueArgOwnTwinRustAsyncMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsyncMoi(
      //           rustAutoOpaqueArgOwnTwinRustAsyncMoi(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncMoi(rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinRustAsyncMoi(rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsyncMoi(
      //           rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsyncMoi(rustAutoOpaqueArgMutBorrowTwinRustAsyncMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsyncMoi(arg: a);

      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueArgOwnTwinRustAsyncMoi(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinRustAsyncMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 20);

      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueTwoArgsTwinRustAsyncMoi(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 42);

      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueNormalAndOpaqueArgTwinRustAsyncMoi(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinRustAsyncMoi();
      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaquePlusSignArgTwinRustAsyncMoi(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinRustAsyncMoi();
      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueCallableArgTwinRustAsyncMoi(arg: obj));
    });
  });

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinRustAsyncMoi obj, String expect) async {
  //     await futurizeVoidTwinRustAsyncMoi(rustAutoOpaqueTraitObjectArgBorrowTwinRustAsyncMoi(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinRustAsyncMoi(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsyncMoi(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinRustAsyncMoi(
  //         rustAutoOpaqueTraitObjectArgOwnTwinRustAsyncMoi(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsyncMoi(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsyncMoi(), 'B'));
  // });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinRustAsyncMoi
        .staticMethodReturnOwnTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        NonCloneSimpleTwinRustAsyncMoi.staticMethodArgBorrowTwinRustAsyncMoi(
            arg: obj));
    await futurizeVoidTwinRustAsyncMoi(
        NonCloneSimpleTwinRustAsyncMoi.staticMethodArgMutBorrowTwinRustAsyncMoi(
            arg: obj));
    await futurizeVoidTwinRustAsyncMoi(
        NonCloneSimpleTwinRustAsyncMoi.staticMethodArgOwnTwinRustAsyncMoi(
            arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinRustAsyncMoi.newTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        obj.instanceMethodArgBorrowTwinRustAsyncMoi());
    await futurizeVoidTwinRustAsyncMoi(
        obj.instanceMethodArgMutBorrowTwinRustAsyncMoi());
    await futurizeVoidTwinRustAsyncMoi(
        obj.instanceMethodReturnOwnTwinRustAsyncMoi());
    await futurizeVoidTwinRustAsyncMoi(
        obj.instanceMethodArgOwnTwinRustAsyncMoi());
  });
  test('instance newWithResult', () async {
    final obj =
        await NonCloneSimpleTwinRustAsyncMoi.newWithResultTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        obj.instanceMethodArgBorrowTwinRustAsyncMoi());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinRustAsyncMoi.newTwinRustAsyncMoi();
    expect(await obj.instanceMethodGetterTwinRustAsyncMoi, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsyncMoi();
    expect(obj.good, 'hello');
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsyncMoi(
            arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinRustAsyncMoi());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinRustAsyncMoi());

    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncMoi(arg: good));
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncMoi(arg: opaque));

    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsyncMoi(
            arg: EnumWithGoodAndOpaqueTwinRustAsyncMoi.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinRustAsyncMoi();
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueEnumArgBorrowTwinRustAsyncMoi(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinRustAsyncMoi();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinRustAsyncMoi();

    expect(vec.length, 2);
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: vec[0], expect: 10));
    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueArgBorrowTwinRustAsyncMoi(arg: vec[1], expect: 20));

    await futurizeVoidTwinRustAsyncMoi(
        rustAutoOpaqueArgVecOwnTwinRustAsyncMoi(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj =
          await rustAutoOpaqueExplicitReturnTwinRustAsyncMoi(initial: 100);
      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueExplicitArgTwinRustAsyncMoi(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj =
          await rustAutoOpaqueExplicitReturnTwinRustAsyncMoi(initial: 100);
      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueExplicitStructTwinRustAsyncMoi(
              arg: StructWithExplicitAutoOpaqueFieldTwinRustAsyncMoi(
                  autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinRustAsyncMoi();
      await futurizeVoidTwinRustAsyncMoi(
          rustAutoOpaqueExplicitStructTwinRustAsyncMoi(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj =
            await rustAutoOpaqueExplicitReturnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueArgOwnTwinRustAsyncMoi(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
        await futurizeVoidTwinRustAsyncMoi(
            rustAutoOpaqueExplicitArgTwinRustAsyncMoi(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncMoi(
            borrow: obj, mutBorrow: obj),
        'TwinRustAsyncMoi',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinRustAsyncMoi(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinRustAsyncMoi(a: obj, b: obj),
          200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinRustAsyncMoi(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 200);
      expect(
          await rustAutoOpaqueSleepTwinRustAsyncMoi(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinRustAsyncMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinRustAsyncMoi(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsyncMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinRustAsyncMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinRustAsyncMoi(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
