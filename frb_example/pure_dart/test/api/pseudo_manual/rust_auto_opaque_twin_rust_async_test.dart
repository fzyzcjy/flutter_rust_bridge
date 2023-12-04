// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_auto_opaque_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:flutter_rust_bridge/src/droppable/droppable.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/rust_auto_opaque_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
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

  group('trait object', () {
    Future<void> _body(
        RwLockBoxHelloTraitTwinRustAsync obj, String expect) async {
      await rustAutoOpaqueTraitObjectArgBorrowTwinRustAsync(
          arg: obj, expect: expect);
      await rustAutoOpaqueTraitObjectArgMutBorrowTwinRustAsync(
          arg: obj, expect: expect);
      await rustAutoOpaqueTraitObjectArgOwnTwinRustAsync(
          arg: obj, expect: expect);
    }

    test(
        'case one',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnOneTwinRustAsync(),
            'hello'));
    test(
        'case two',
        () async => await _body(
            await rustAutoOpaqueTraitObjectReturnOwnTwoTwinRustAsync(), 'B'));
  });

  test('static method', () async {
    final obj = await RwLockNonCloneSimpleTwinRustAsync
        .staticMethodReturnOwnTwinRustAsync();
    await futurizeVoidTwinRustAsync(
        RwLockNonCloneSimpleTwinRustAsync.staticMethodArgBorrowTwinRustAsync(
            arg: obj));
    await futurizeVoidTwinRustAsync(
        RwLockNonCloneSimpleTwinRustAsync.staticMethodArgMutBorrowTwinRustAsync(
            arg: obj));
    await futurizeVoidTwinRustAsync(
        RwLockNonCloneSimpleTwinRustAsync.staticMethodArgOwnTwinRustAsync(
            arg: obj));
  });

  test('instance method', () async {
    final obj = await RwLockNonCloneSimpleTwinRustAsync.newTwinRustAsync();
    await futurizeVoidTwinRustAsync(obj.instanceMethodArgBorrowTwinRustAsync());
    await futurizeVoidTwinRustAsync(
        obj.instanceMethodArgMutBorrowTwinRustAsync());
    await futurizeVoidTwinRustAsync(obj.instanceMethodReturnOwnTwinRustAsync());
    await futurizeVoidTwinRustAsync(obj.instanceMethodArgOwnTwinRustAsync());
  });

  test('types with both encodable and opaque fields', () async {
    final obj =
        await rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinRustAsync();
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgBorrowTwinRustAsync(
            arg: obj));
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgMutBorrowTwinRustAsync(
            arg: obj));
    await futurizeVoidTwinRustAsync(
        rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinRustAsync(
            arg: obj));
  });
}
