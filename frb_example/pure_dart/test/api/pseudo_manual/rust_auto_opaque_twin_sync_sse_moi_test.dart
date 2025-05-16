// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_sync_sse_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgOwnTwinSyncSseMoi(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgOwnTwinSyncSseMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
                arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
                arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
                arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);

        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100));

        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
                arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await Future.wait([
          futurizeVoidTwinSyncSseMoi(
              rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100)),
          futurizeVoidTwinSyncSseMoi(
              rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncSseMoi(
      //           rustAutoOpaqueArgOwnTwinSyncSseMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinSyncSseMoi(
      //           rustAutoOpaqueArgOwnTwinSyncSseMoi(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncSseMoi(rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinSyncSseMoi(rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncSseMoi(
      //           rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinSyncSseMoi(rustAutoOpaqueArgMutBorrowTwinSyncSseMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinSyncSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinSyncSseMoi(arg: a);

      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueArgOwnTwinSyncSseMoi(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinSyncSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 20);

      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueTwoArgsTwinSyncSseMoi(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinSyncSseMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 42);

      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueNormalAndOpaqueArgTwinSyncSseMoi(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinSyncSseMoi();
      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaquePlusSignArgTwinSyncSseMoi(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinSyncSseMoi();
      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueCallableArgTwinSyncSseMoi(arg: obj));
    });
  });

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinSyncSseMoi obj, String expect) async {
  //     await futurizeVoidTwinSyncSseMoi(rustAutoOpaqueTraitObjectArgBorrowTwinSyncSseMoi(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinSyncSseMoi(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinSyncSseMoi(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinSyncSseMoi(
  //         rustAutoOpaqueTraitObjectArgOwnTwinSyncSseMoi(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinSyncSseMoi(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSyncSseMoi(), 'B'));
  // });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinSyncSseMoi
        .staticMethodReturnOwnTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(
        NonCloneSimpleTwinSyncSseMoi.staticMethodArgBorrowTwinSyncSseMoi(
            arg: obj));
    await futurizeVoidTwinSyncSseMoi(
        NonCloneSimpleTwinSyncSseMoi.staticMethodArgMutBorrowTwinSyncSseMoi(
            arg: obj));
    await futurizeVoidTwinSyncSseMoi(
        NonCloneSimpleTwinSyncSseMoi.staticMethodArgOwnTwinSyncSseMoi(
            arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinSyncSseMoi.newTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(
        obj.instanceMethodArgBorrowTwinSyncSseMoi());
    await futurizeVoidTwinSyncSseMoi(
        obj.instanceMethodArgMutBorrowTwinSyncSseMoi());
    await futurizeVoidTwinSyncSseMoi(
        obj.instanceMethodReturnOwnTwinSyncSseMoi());
    await futurizeVoidTwinSyncSseMoi(obj.instanceMethodArgOwnTwinSyncSseMoi());
  });
  test('instance newWithResult', () async {
    final obj =
        await NonCloneSimpleTwinSyncSseMoi.newWithResultTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(
        obj.instanceMethodArgBorrowTwinSyncSseMoi());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinSyncSseMoi.newTwinSyncSseMoi();
    expect(await obj.instanceMethodGetterTwinSyncSseMoi, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSyncSseMoi();
    expect(obj.good, 'hello');
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSyncSseMoi(
            arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSyncSseMoi());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSyncSseMoi());

    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSyncSseMoi(arg: good));
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSyncSseMoi(arg: opaque));

    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSyncSseMoi(
            arg: EnumWithGoodAndOpaqueTwinSyncSseMoi.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinSyncSseMoi();
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueEnumArgBorrowTwinSyncSseMoi(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinSyncSseMoi();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinSyncSseMoi();

    expect(vec.length, 2);
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: vec[0], expect: 10));
    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueArgBorrowTwinSyncSseMoi(arg: vec[1], expect: 20));

    await futurizeVoidTwinSyncSseMoi(
        rustAutoOpaqueArgVecOwnTwinSyncSseMoi(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj =
          await rustAutoOpaqueExplicitReturnTwinSyncSseMoi(initial: 100);
      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueExplicitArgTwinSyncSseMoi(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj =
          await rustAutoOpaqueExplicitReturnTwinSyncSseMoi(initial: 100);
      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueExplicitStructTwinSyncSseMoi(
              arg: StructWithExplicitAutoOpaqueFieldTwinSyncSseMoi(
                  autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinSyncSseMoi();
      await futurizeVoidTwinSyncSseMoi(
          rustAutoOpaqueExplicitStructTwinSyncSseMoi(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj =
            await rustAutoOpaqueExplicitReturnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueArgOwnTwinSyncSseMoi(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
        await futurizeVoidTwinSyncSseMoi(
            rustAutoOpaqueExplicitArgTwinSyncSseMoi(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinSyncSseMoi(
            borrow: obj, mutBorrow: obj),
        'TwinSyncSseMoi',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinSyncSseMoi(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSyncSseMoi(a: obj, b: obj),
          200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinSyncSseMoi(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 200);
      expect(await rustAutoOpaqueSleepTwinSyncSseMoi(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSyncSseMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSyncSseMoi(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncSseMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSyncSseMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSyncSseMoi(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
