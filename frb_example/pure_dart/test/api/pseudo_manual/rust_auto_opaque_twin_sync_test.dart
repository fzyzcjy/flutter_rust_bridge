// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

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

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinSync obj, String expect) async {
  //     await futurizeVoidTwinSync(rustAutoOpaqueTraitObjectArgBorrowTwinSync(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinSync(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinSync(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinSync(
  //         rustAutoOpaqueTraitObjectArgOwnTwinSync(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinSync(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSync(), 'B'));
  // });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinSync.staticMethodReturnOwnTwinSync();
    await futurizeVoidTwinSync(
        NonCloneSimpleTwinSync.staticMethodArgBorrowTwinSync(arg: obj));
    await futurizeVoidTwinSync(
        NonCloneSimpleTwinSync.staticMethodArgMutBorrowTwinSync(arg: obj));
    await futurizeVoidTwinSync(
        NonCloneSimpleTwinSync.staticMethodArgOwnTwinSync(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinSync.newTwinSync();
    await futurizeVoidTwinSync(obj.instanceMethodArgBorrowTwinSync());
    await futurizeVoidTwinSync(obj.instanceMethodArgMutBorrowTwinSync());
    await futurizeVoidTwinSync(obj.instanceMethodReturnOwnTwinSync());
    await futurizeVoidTwinSync(obj.instanceMethodArgOwnTwinSync());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinSync.newWithResultTwinSync();
    await futurizeVoidTwinSync(obj.instanceMethodArgBorrowTwinSync());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinSync.newTwinSync();
    expect(await obj.instanceMethodGetterTwinSync, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSync();
    expect(obj.good, 'hello');
    await futurizeVoidTwinSync(
        rustAutoOpaqueArgBorrowTwinSync(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinSync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSync(arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSync());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSync());

    await futurizeVoidTwinSync(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSync(arg: good));
    await futurizeVoidTwinSync(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSync(arg: opaque));

    await futurizeVoidTwinSync(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSync(
            arg: EnumWithGoodAndOpaqueTwinSync.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinSync();
    await futurizeVoidTwinSync(rustAutoOpaqueEnumArgBorrowTwinSync(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinSync();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinSync(
        rustAutoOpaqueArgBorrowTwinSync(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinSync();

    expect(vec.length, 2);
    await futurizeVoidTwinSync(
        rustAutoOpaqueArgBorrowTwinSync(arg: vec[0], expect: 10));
    await futurizeVoidTwinSync(
        rustAutoOpaqueArgBorrowTwinSync(arg: vec[1], expect: 20));

    await futurizeVoidTwinSync(
        rustAutoOpaqueArgVecOwnTwinSync(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSync(initial: 100);
      await futurizeVoidTwinSync(
          rustAutoOpaqueExplicitArgTwinSync(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSync(initial: 100);
      await futurizeVoidTwinSync(rustAutoOpaqueExplicitStructTwinSync(
          arg: StructWithExplicitAutoOpaqueFieldTwinSync(
              autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinSync();
      await futurizeVoidTwinSync(
          rustAutoOpaqueExplicitStructTwinSync(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj = await rustAutoOpaqueExplicitReturnTwinSync(initial: 100);
        await futurizeVoidTwinSync(
            rustAutoOpaqueArgOwnTwinSync(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
        await futurizeVoidTwinSync(
            rustAutoOpaqueExplicitArgTwinSync(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinSync(
            borrow: obj, mutBorrow: obj),
        'TwinSync',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSync(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinSync(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSync(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSync(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSync(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSync(initial: 200);
      expect(await rustAutoOpaqueSleepTwinSync(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSync(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSync(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSync(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSync(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSync(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSync(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
