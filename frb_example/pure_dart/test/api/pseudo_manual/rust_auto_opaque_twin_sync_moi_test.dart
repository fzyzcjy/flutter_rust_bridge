// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_sync_moi.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgOwnTwinSyncMoi(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgOwnTwinSyncMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);

        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100));

        await futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await Future.wait([
          futurizeVoidTwinSyncMoi(
              rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100)),
          futurizeVoidTwinSyncMoi(
              rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncMoi(
      //           rustAutoOpaqueArgOwnTwinSyncMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinSyncMoi(
      //           rustAutoOpaqueArgOwnTwinSyncMoi(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinSyncMoi(
      //           rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinSyncMoi(rustAutoOpaqueArgMutBorrowTwinSyncMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinSyncMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinSyncMoi(arg: a);

      await futurizeVoidTwinSyncMoi(
          rustAutoOpaqueArgOwnTwinSyncMoi(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinSyncMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 20);

      await futurizeVoidTwinSyncMoi(
          rustAutoOpaqueTwoArgsTwinSyncMoi(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinSyncMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 42);

      await futurizeVoidTwinSyncMoi(
          rustAutoOpaqueNormalAndOpaqueArgTwinSyncMoi(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinSyncMoi();
      await futurizeVoidTwinSyncMoi(
          rustAutoOpaquePlusSignArgTwinSyncMoi(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinSyncMoi();
      await futurizeVoidTwinSyncMoi(
          rustAutoOpaqueCallableArgTwinSyncMoi(arg: obj));
    });
  });

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinSyncMoi obj, String expect) async {
  //     await futurizeVoidTwinSyncMoi(rustAutoOpaqueTraitObjectArgBorrowTwinSyncMoi(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinSyncMoi(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinSyncMoi(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinSyncMoi(
  //         rustAutoOpaqueTraitObjectArgOwnTwinSyncMoi(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinSyncMoi(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinSyncMoi(), 'B'));
  // });

  test('static method', () async {
    final obj =
        await NonCloneSimpleTwinSyncMoi.staticMethodReturnOwnTwinSyncMoi();
    await futurizeVoidTwinSyncMoi(
        NonCloneSimpleTwinSyncMoi.staticMethodArgBorrowTwinSyncMoi(arg: obj));
    await futurizeVoidTwinSyncMoi(
        NonCloneSimpleTwinSyncMoi.staticMethodArgMutBorrowTwinSyncMoi(
            arg: obj));
    await futurizeVoidTwinSyncMoi(
        NonCloneSimpleTwinSyncMoi.staticMethodArgOwnTwinSyncMoi(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinSyncMoi.newTwinSyncMoi();
    await futurizeVoidTwinSyncMoi(obj.instanceMethodArgBorrowTwinSyncMoi());
    await futurizeVoidTwinSyncMoi(obj.instanceMethodArgMutBorrowTwinSyncMoi());
    await futurizeVoidTwinSyncMoi(obj.instanceMethodReturnOwnTwinSyncMoi());
    await futurizeVoidTwinSyncMoi(obj.instanceMethodArgOwnTwinSyncMoi());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinSyncMoi.newWithResultTwinSyncMoi();
    await futurizeVoidTwinSyncMoi(obj.instanceMethodArgBorrowTwinSyncMoi());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinSyncMoi.newTwinSyncMoi();
    expect(await obj.instanceMethodGetterTwinSyncMoi, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinSyncMoi();
    expect(obj.good, 'hello');
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinSyncMoi(arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinSyncMoi());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinSyncMoi());

    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSyncMoi(arg: good));
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSyncMoi(arg: opaque));

    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinSyncMoi(
            arg: EnumWithGoodAndOpaqueTwinSyncMoi.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinSyncMoi();
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueEnumArgBorrowTwinSyncMoi(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinSyncMoi();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueArgBorrowTwinSyncMoi(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinSyncMoi();

    expect(vec.length, 2);
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueArgBorrowTwinSyncMoi(arg: vec[0], expect: 10));
    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueArgBorrowTwinSyncMoi(arg: vec[1], expect: 20));

    await futurizeVoidTwinSyncMoi(
        rustAutoOpaqueArgVecOwnTwinSyncMoi(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSyncMoi(initial: 100);
      await futurizeVoidTwinSyncMoi(
          rustAutoOpaqueExplicitArgTwinSyncMoi(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinSyncMoi(initial: 100);
      await futurizeVoidTwinSyncMoi(rustAutoOpaqueExplicitStructTwinSyncMoi(
          arg: StructWithExplicitAutoOpaqueFieldTwinSyncMoi(
              autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinSyncMoi();
      await futurizeVoidTwinSyncMoi(
          rustAutoOpaqueExplicitStructTwinSyncMoi(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj = await rustAutoOpaqueExplicitReturnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueArgOwnTwinSyncMoi(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
        await futurizeVoidTwinSyncMoi(
            rustAutoOpaqueExplicitArgTwinSyncMoi(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinSyncMoi(
            borrow: obj, mutBorrow: obj),
        'TwinSyncMoi',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinSyncMoi(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      expect(
          await rustAutoOpaqueBorrowAndBorrowTwinSyncMoi(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinSyncMoi(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 200);
      expect(await rustAutoOpaqueSleepTwinSyncMoi(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSyncMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSyncMoi(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinSyncMoi(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinSyncMoi(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinSyncMoi(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
