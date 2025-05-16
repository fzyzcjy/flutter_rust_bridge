// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/rust_auto_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);

        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));

        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await Future.wait([
          futurizeVoidTwinNormal(
              rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
          futurizeVoidTwinNormal(
              rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinNormal(
      //           rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100)),
      //       futurizeVoidTwinNormal(
      //           rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinNormal(
      //           rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
      //       futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinNormal', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinNormal(arg: a);

      await futurizeVoidTwinNormal(
          rustAutoOpaqueArgOwnTwinNormal(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinNormal', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 20);

      await futurizeVoidTwinNormal(rustAutoOpaqueTwoArgsTwinNormal(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinNormal', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 42);

      await futurizeVoidTwinNormal(
          rustAutoOpaqueNormalAndOpaqueArgTwinNormal(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinNormal();
      await futurizeVoidTwinNormal(
          rustAutoOpaquePlusSignArgTwinNormal(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinNormal();
      await futurizeVoidTwinNormal(
          rustAutoOpaqueCallableArgTwinNormal(arg: obj));
    });
  });

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinNormal obj, String expect) async {
  //     await futurizeVoidTwinNormal(rustAutoOpaqueTraitObjectArgBorrowTwinNormal(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinNormal(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinNormal(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinNormal(
  //         rustAutoOpaqueTraitObjectArgOwnTwinNormal(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinNormal(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinNormal(), 'B'));
  // });

  test('static method', () async {
    final obj =
        await NonCloneSimpleTwinNormal.staticMethodReturnOwnTwinNormal();
    await futurizeVoidTwinNormal(
        NonCloneSimpleTwinNormal.staticMethodArgBorrowTwinNormal(arg: obj));
    await futurizeVoidTwinNormal(
        NonCloneSimpleTwinNormal.staticMethodArgMutBorrowTwinNormal(arg: obj));
    await futurizeVoidTwinNormal(
        NonCloneSimpleTwinNormal.staticMethodArgOwnTwinNormal(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinNormal.newTwinNormal();
    await futurizeVoidTwinNormal(obj.instanceMethodArgBorrowTwinNormal());
    await futurizeVoidTwinNormal(obj.instanceMethodArgMutBorrowTwinNormal());
    await futurizeVoidTwinNormal(obj.instanceMethodReturnOwnTwinNormal());
    await futurizeVoidTwinNormal(obj.instanceMethodArgOwnTwinNormal());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinNormal.newWithResultTwinNormal();
    await futurizeVoidTwinNormal(obj.instanceMethodArgBorrowTwinNormal());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinNormal.newTwinNormal();
    expect(await obj.instanceMethodGetterTwinNormal, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinNormal();
    expect(obj.good, 'hello');
    await futurizeVoidTwinNormal(
        rustAutoOpaqueArgBorrowTwinNormal(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinNormal(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinNormal(arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinNormal());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinNormal());

    await futurizeVoidTwinNormal(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinNormal(arg: good));
    await futurizeVoidTwinNormal(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinNormal(arg: opaque));

    await futurizeVoidTwinNormal(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinNormal(
            arg: EnumWithGoodAndOpaqueTwinNormal.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinNormal();
    await futurizeVoidTwinNormal(
        rustAutoOpaqueEnumArgBorrowTwinNormal(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinNormal();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinNormal(
        rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinNormal();

    expect(vec.length, 2);
    await futurizeVoidTwinNormal(
        rustAutoOpaqueArgBorrowTwinNormal(arg: vec[0], expect: 10));
    await futurizeVoidTwinNormal(
        rustAutoOpaqueArgBorrowTwinNormal(arg: vec[1], expect: 20));

    await futurizeVoidTwinNormal(
        rustAutoOpaqueArgVecOwnTwinNormal(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinNormal(initial: 100);
      await futurizeVoidTwinNormal(
          rustAutoOpaqueExplicitArgTwinNormal(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinNormal(initial: 100);
      await futurizeVoidTwinNormal(rustAutoOpaqueExplicitStructTwinNormal(
          arg: StructWithExplicitAutoOpaqueFieldTwinNormal(
              autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinNormal();
      await futurizeVoidTwinNormal(
          rustAutoOpaqueExplicitStructTwinNormal(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj = await rustAutoOpaqueExplicitReturnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(
            rustAutoOpaqueExplicitArgTwinNormal(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinNormal(
            borrow: obj, mutBorrow: obj),
        'TwinNormal',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinNormal(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinNormal(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinNormal(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);
      expect(await rustAutoOpaqueSleepTwinNormal(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinNormal(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinNormal(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinNormal(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinNormal(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinNormal(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
