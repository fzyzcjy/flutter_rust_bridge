// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgOwnTwinSseMoi(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgOwnTwinSseMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);

        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100));

        await futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await Future.wait([
          futurizeVoidTwinSseMoi(
              rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100)),
          futurizeVoidTwinSseMoi(
              rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSseMoi(
      //           rustAutoOpaqueArgOwnTwinSseMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinSseMoi(
      //           rustAutoOpaqueArgOwnTwinSseMoi(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSseMoi(
      //           rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinSseMoi(rustAutoOpaqueArgMutBorrowTwinSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinSseMoi(arg: a);

      await futurizeVoidTwinSseMoi(
          rustAutoOpaqueArgOwnTwinSseMoi(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 20);

      await futurizeVoidTwinSseMoi(rustAutoOpaqueTwoArgsTwinSseMoi(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 42);

      await futurizeVoidTwinSseMoi(
          rustAutoOpaqueNormalAndOpaqueArgTwinSseMoi(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinSseMoi();
      await futurizeVoidTwinSseMoi(
          rustAutoOpaquePlusSignArgTwinSseMoi(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinSseMoi();
      await futurizeVoidTwinSseMoi(
          rustAutoOpaqueCallableArgTwinSseMoi(arg: obj));
    });
  });

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinSseMoi obj, String expect) async {
  //     await futurizeVoidTwinSseMoi(rustAutoOpaqueTraitObjectArgBorrowTwinSseMoi(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinSseMoi(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinSseMoi(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinSseMoi(
  //         rustAutoOpaqueTraitObjectArgOwnTwinSseMoi(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinSseMoi(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSseMoi(), 'B'));
  // });

  test('static method', () async {
    final obj =
        await NonCloneSimpleTwinSseMoi.staticMethodReturnOwnTwinSseMoi();
    await futurizeVoidTwinSseMoi(
        NonCloneSimpleTwinSseMoi.staticMethodArgBorrowTwinSseMoi(arg: obj));
    await futurizeVoidTwinSseMoi(
        NonCloneSimpleTwinSseMoi.staticMethodArgMutBorrowTwinSseMoi(arg: obj));
    await futurizeVoidTwinSseMoi(
        NonCloneSimpleTwinSseMoi.staticMethodArgOwnTwinSseMoi(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinSseMoi.newTwinSseMoi();
    await futurizeVoidTwinSseMoi(obj.instanceMethodArgBorrowTwinSseMoi());
    await futurizeVoidTwinSseMoi(obj.instanceMethodArgMutBorrowTwinSseMoi());
    await futurizeVoidTwinSseMoi(obj.instanceMethodReturnOwnTwinSseMoi());
    await futurizeVoidTwinSseMoi(obj.instanceMethodArgOwnTwinSseMoi());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinSseMoi.newWithResultTwinSseMoi();
    await futurizeVoidTwinSseMoi(obj.instanceMethodArgBorrowTwinSseMoi());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinSseMoi.newTwinSseMoi();
    expect(await obj.instanceMethodGetterTwinSseMoi, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSseMoi();
    expect(obj.good, 'hello');
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSseMoi(arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSseMoi());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSseMoi());

    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSseMoi(arg: good));
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSseMoi(arg: opaque));

    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSseMoi(
            arg: EnumWithGoodAndOpaqueTwinSseMoi.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinSseMoi();
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueEnumArgBorrowTwinSseMoi(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinSseMoi();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueArgBorrowTwinSseMoi(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinSseMoi();

    expect(vec.length, 2);
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueArgBorrowTwinSseMoi(arg: vec[0], expect: 10));
    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueArgBorrowTwinSseMoi(arg: vec[1], expect: 20));

    await futurizeVoidTwinSseMoi(
        rustAutoOpaqueArgVecOwnTwinSseMoi(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSseMoi(initial: 100);
      await futurizeVoidTwinSseMoi(
          rustAutoOpaqueExplicitArgTwinSseMoi(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSseMoi(initial: 100);
      await futurizeVoidTwinSseMoi(rustAutoOpaqueExplicitStructTwinSseMoi(
          arg: StructWithExplicitAutoOpaqueFieldTwinSseMoi(
              autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinSseMoi();
      await futurizeVoidTwinSseMoi(
          rustAutoOpaqueExplicitStructTwinSseMoi(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj = await rustAutoOpaqueExplicitReturnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueArgOwnTwinSseMoi(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
        await futurizeVoidTwinSseMoi(
            rustAutoOpaqueExplicitArgTwinSseMoi(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinSseMoi(
            borrow: obj, mutBorrow: obj),
        'TwinSseMoi',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinSseMoi(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinSseMoi(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSseMoi(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 200);
      expect(await rustAutoOpaqueSleepTwinSseMoi(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSseMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSseMoi(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSseMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSseMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSseMoi(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
