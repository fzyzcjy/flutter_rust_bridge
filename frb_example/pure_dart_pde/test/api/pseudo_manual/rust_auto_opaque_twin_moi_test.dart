// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/rust_auto_opaque_twin_moi.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('simple functions', () {
    test('return opaque', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      expect(obj.isDisposed, false);
      obj.dispose();
    });

    group('arg owned', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgOwnTwinMoi(arg: obj, expect: 100));
      });

      test('after call, the object cannot be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgOwnTwinMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, true);

        await expectLater(
            () => rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100),
            throwsA(isA<DroppableDisposedException>()));
      });
    });

    group('arg ref', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100));
      });
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgMutBorrowTwinMoi(arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
      });

      test('after call, the object can still be used again', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgMutBorrowTwinMoi(arg: obj, expect: 100, adder: 1));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinMoi(rustAutoOpaqueArgMutBorrowTwinMoi(
            arg: obj, expect: 101, adder: 10));

        expect(obj.isDisposed, false);

        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 111));
      });

      test('does change the internal data', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);

        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100));

        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgMutBorrowTwinMoi(arg: obj, expect: 100, adder: 1));

        // expect internal data to change
        await futurizeVoidTwinMoi(
            rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100 + 1));
      });
    });

    group('concurrent calls', () {
      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
        await Future.wait([
          futurizeVoidTwinMoi(
              rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100)),
          futurizeVoidTwinMoi(
              rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100)),
        ]);
      });

      // Not test yet, since this requires one function to acquire the Rust RwLock
      // before the other releases it, thus require some timing.
      //
      // test('cannot call multiple `T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinMoi(
      //           rustAutoOpaqueArgOwnTwinMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinMoi(
      //           rustAutoOpaqueArgOwnTwinMoi(arg: obj, expect: 100)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call multiple `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinMoi(rustAutoOpaqueArgMutBorrowTwinMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //       futurizeVoidTwinMoi(rustAutoOpaqueArgMutBorrowTwinMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
      //
      // test('cannot call one `&T` and one `&mut T` concurrently', () async {
      //   final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      //   await expectLater(() async {
      //     return Future.wait([
      //       futurizeVoidTwinMoi(
      //           rustAutoOpaqueArgBorrowTwinMoi(arg: obj, expect: 100)),
      //       futurizeVoidTwinMoi(rustAutoOpaqueArgMutBorrowTwinMoi(
      //           arg: obj, expect: 100, adder: 1)),
      //     ]);
      //   }, throwsA(isA<DroppableDisposedException>()));
      // });
    });
  });

  group('with other args', () {
    test('call rustAutoOpaqueArgOwnAndReturnOwnTwinMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinMoi(initial: 42);

      final b = await rustAutoOpaqueArgOwnAndReturnOwnTwinMoi(arg: a);

      await futurizeVoidTwinMoi(
          rustAutoOpaqueArgOwnTwinMoi(arg: b, expect: 42));
    });

    test('call rustAutoOpaqueTwoArgsTwinMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinMoi(initial: 10);
      final b = await rustAutoOpaqueReturnOwnTwinMoi(initial: 20);

      await futurizeVoidTwinMoi(rustAutoOpaqueTwoArgsTwinMoi(a: a, b: b));
    });

    test('call rustAutoOpaqueNormalAndOpaqueArgTwinMoi', () async {
      final a = await rustAutoOpaqueReturnOwnTwinMoi(initial: 42);

      await futurizeVoidTwinMoi(
          rustAutoOpaqueNormalAndOpaqueArgTwinMoi(a: a, b: 'hello'));
    });
  });

  group('complex type signatures', () {
    test('plus sign', () async {
      final obj = await rustAutoOpaquePlusSignReturnTwinMoi();
      await futurizeVoidTwinMoi(rustAutoOpaquePlusSignArgTwinMoi(arg: obj));
    });

    test('callable', () async {
      final obj = await rustAutoOpaqueCallableReturnTwinMoi();
      await futurizeVoidTwinMoi(rustAutoOpaqueCallableArgTwinMoi(arg: obj));
    });
  });

  group('trait object', () {
    Future<void> _body(BoxHelloTraitTwinMoi obj, String expect) async {
      await futurizeVoidTwinMoi(
          rustAutoOpaqueTraitObjectArgBorrowTwinMoi(arg: obj, expect: expect));
      await futurizeVoidTwinMoi(rustAutoOpaqueTraitObjectArgMutBorrowTwinMoi(
          arg: obj, expect: expect));
      await futurizeVoidTwinMoi(
          rustAutoOpaqueTraitObjectArgOwnTwinMoi(arg: obj, expect: expect));
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinMoi(), 'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinMoi(), 'B'));
  });

  test('static method', () async {
    final obj = await NonCloneSimpleTwinMoi.staticMethodReturnOwnTwinMoi();
    await futurizeVoidTwinMoi(
        NonCloneSimpleTwinMoi.staticMethodArgBorrowTwinMoi(arg: obj));
    await futurizeVoidTwinMoi(
        NonCloneSimpleTwinMoi.staticMethodArgMutBorrowTwinMoi(arg: obj));
    await futurizeVoidTwinMoi(
        NonCloneSimpleTwinMoi.staticMethodArgOwnTwinMoi(arg: obj));
  });

  test('instance method', () async {
    final obj = await NonCloneSimpleTwinMoi.newTwinMoi();
    await futurizeVoidTwinMoi(obj.instanceMethodArgBorrowTwinMoi());
    await futurizeVoidTwinMoi(obj.instanceMethodArgMutBorrowTwinMoi());
    await futurizeVoidTwinMoi(obj.instanceMethodReturnOwnTwinMoi());
    await futurizeVoidTwinMoi(obj.instanceMethodArgOwnTwinMoi());
  });
  test('instance newWithResult', () async {
    final obj = await NonCloneSimpleTwinMoi.newWithResultTwinMoi();
    await futurizeVoidTwinMoi(obj.instanceMethodArgBorrowTwinMoi());
  });

  test('getter', () async {
    final obj = await NonCloneSimpleTwinMoi.newTwinMoi();
    expect(await obj.instanceMethodGetterTwinMoi, 42);
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinMoi();
    await futurizeVoidTwinMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinMoi(arg: obj));
    await futurizeVoidTwinMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinMoi(
            arg: obj));
    await futurizeVoidTwinMoi(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinMoi(arg: obj));
  });

  group('borrow + mut borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      await expectRustPanic(
        () async => rustAutoOpaqueBorrowAndMutBorrowTwinMoi(
            borrow: obj, mutBorrow: obj),
        'TwinMoi',
        messageMatcherOnNative: matches(RegExp('Fail to.*borrow object')),
      );
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinMoi(initial: 200);
      expect(
          await rustAutoOpaqueBorrowAndMutBorrowTwinMoi(
              borrow: a, mutBorrow: b),
          300);
    });
  });

  group('borrow + borrow', () {
    test('when same object', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinMoi(a: obj, b: obj), 200);
    });

    test('when different object', () async {
      final a = await rustAutoOpaqueReturnOwnTwinMoi(initial: 100);
      final b = await rustAutoOpaqueReturnOwnTwinMoi(initial: 200);
      expect(await rustAutoOpaqueBorrowAndBorrowTwinMoi(a: a, b: b), 300);
    });
  });
}
