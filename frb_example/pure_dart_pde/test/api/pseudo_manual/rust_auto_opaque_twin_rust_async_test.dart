// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/rust_auto_opaque_twin_rust_async.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgOwnTwinRustAsync(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgOwnTwinRustAsync(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
            arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);

        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100));

        await futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
            arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await Future.wait([
          futurizeVoidTwinRustAsync(
              rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100)),
          futurizeVoidTwinRustAsync(
              rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsync(
      //           rustAutoOpaqueArgOwnTwinRustAsync(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsync(
      //           rustAutoOpaqueArgOwnTwinRustAsync(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinRustAsync(
      //           rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 100)),
      //       futurizeVoidTwinRustAsync(rustAutoOpaqueArgMutBorrowTwinRustAsync(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsync', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinRustAsync(arg: a);

      await futurizeVoidTwinRustAsync(
          rustAutoOpaqueArgOwnTwinRustAsync(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinRustAsync', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 20);

      await futurizeVoidTwinRustAsync(
          rustAutoOpaqueTwoArgsTwinRustAsync(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinRustAsync', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 42);

      await futurizeVoidTwinRustAsync(
          rustAutoOpaqueNormalAndOpaqueArgTwinRustAsync(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinRustAsync();
      await futurizeVoidTwinRustAsync(
          rustAutoOpaquePlusSignArgTwinRustAsync(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinRustAsync();
      await futurizeVoidTwinRustAsync(
          rustAutoOpaqueCallableArgTwinRustAsync(arg: obj));
    });
  });

  // group('trait object', () {
  //   Future<void> _body(BoxHelloTraitTwinRustAsync obj, String expect) async {
  //     await futurizeVoidTwinRustAsync(rustAutoOpaqueTraitObjectArgBorrowTwinRustAsync(
  //         arg: obj, expect: expect));
  //     await futurizeVoidTwinRustAsync(
  //         rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsync(
  //             arg: obj, expect: expect));
  //     await futurizeVoidTwinRustAsync(
  //         rustAutoOpaqueTraitObjectArgOwnTwinRustAsync(arg: obj, expect: expect));
  //   }
  //
  //   test(
  //       'case one',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsync(), 'hello'));
  //   test(
  //       'case two',
  //       () async => await _body(
  //           await rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsync(), 'B'));
  // });

  test('static method', () async {
    final obj =
        await NonCloneSimpleTwinRustAsync.staticMethodReturnOwnTwinRustAsync();
    await futurizeVoidTwinRustAsync(
        NonCloneSimpleTwinRustAsync.staticMethodArgBorrowTwinRustAsync(
            arg: obj));
    await futurizeVoidTwinRustAsync(
        NonCloneSimpleTwinRustAsync.staticMethodArgMutBorrowTwinRustAsync(
            arg: obj));
    await futurizeVoidTwinRustAsync(
        NonCloneSimpleTwinRustAsync.staticMethodArgOwnTwinRustAsync(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinRustAsync.newTwinRustAsync();
    await futurizeVoidTwinRustAsync(obj.instanceMethodArgBorrowTwinRustAsync());
    await futurizeVoidTwinRustAsync(
        obj.instanceMethodArgMutBorrowTwinRustAsync());
    await futurizeVoidTwinRustAsync(obj.instanceMethodReturnOwnTwinRustAsync());
    await futurizeVoidTwinRustAsync(obj.instanceMethodArgOwnTwinRustAsync());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinRustAsync.newWithResultTwinRustAsync();
    await futurizeVoidTwinRustAsync(obj.instanceMethodArgBorrowTwinRustAsync());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinRustAsync.newTwinRustAsync();
    expect(await obj.instanceMethodGetterTwinRustAsync, 42);
  });

  test('structs with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsync();
    expect(obj.good, 'hello');
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj.opaque, expect: 42));
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsync(
            arg: obj));
  });

  test('enums with both encodable and opaque', () async {
    final good =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinRustAsync());
    final opaque =
        (await rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinRustAsync());

    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsync(arg: good));
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsync(arg: opaque));

    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinRustAsync(
            arg: EnumWithGoodAndOpaqueTwinRustAsync.good('hello')));
  });

  test('enum opaque type', () async {
    final obj = await rustAutoOpaqueEnumReturnOwnTwinRustAsync();
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueEnumArgBorrowTwinRustAsync(arg: obj));
  });

  test('stream sink', () async {
    final stream = rustAutoOpaqueStreamSinkTwinRustAsync();
    final obj = (await stream.toList()).single;
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueArgBorrowTwinRustAsync(arg: obj, expect: 42));
  });

  test('vec of opaque', () async {
    final vec = await rustAutoOpaqueReturnVecOwnTwinRustAsync();

    expect(vec.length, 2);
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueArgBorrowTwinRustAsync(arg: vec[0], expect: 10));
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueArgBorrowTwinRustAsync(arg: vec[1], expect: 20));

    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueArgVecOwnTwinRustAsync(arg: vec, expect: [10, 20]));
  });

  group('Explicit rust-auto-opaque types', () {
    test('it can be created and used', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinRustAsync(initial: 100);
      await futurizeVoidTwinRustAsync(
          rustAutoOpaqueExplicitArgTwinRustAsync(arg: obj, expect: 100));
    });

    test('it can be inside a struct used as argument', () async {
      final obj = await rustAutoOpaqueExplicitReturnTwinRustAsync(initial: 100);
      await futurizeVoidTwinRustAsync(rustAutoOpaqueExplicitStructTwinRustAsync(
          arg: StructWithExplicitAutoOpaqueFieldTwinRustAsync(
              autoOpaque: obj, normal: 100)));
    });

    test('it can be inside a struct used as return type', () async {
      final obj = await rustAutoOpaqueExplicitReturnStructTwinRustAsync();
      await futurizeVoidTwinRustAsync(
          rustAutoOpaqueExplicitStructTwinRustAsync(arg: obj));
    });

    group('it can be used with automatic (implicit) ones', () {
      test('create by explicit, use by implicit', () async {
        final obj =
            await rustAutoOpaqueExplicitReturnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueArgOwnTwinRustAsync(arg: obj, expect: 100));
      });

      test('create by implicit, use by explicit', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
        await futurizeVoidTwinRustAsync(
            rustAutoOpaqueExplicitArgTwinRustAsync(arg: obj, expect: 100));
      });
    });
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinRustAsync(
            borrow: obj, mutBorrow: obj),
        'TwinRustAsync',
        messageMatcherOnNative: matches(RegExp('Cannot.*borrow.*object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinRustAsync(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinRustAsync(a: obj, b: obj),
          200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinRustAsync(a: a, b: b), 300);
    });
  });

  group('deadlock', () {
    test('simple call', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 200);
      expect(await rustAutoOpaqueSleepTwinRustAsync(apple: a, orange: b), 300);
    });

    test('call both with same order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinRustAsync(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinRustAsync(apple: a, orange: b);

      expect(await future1, 300);
      expect(await future2, 300);
    });

    test('call both with reversed order', () async {
      final a = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinRustAsync(initial: 200);

      final future1 = rustAutoOpaqueSleepTwinRustAsync(apple: a, orange: b);
      final future2 = rustAutoOpaqueSleepTwinRustAsync(apple: b, orange: a);

      expect(await future1, 300);
      expect(await future2, 300);
    });
  });
}
