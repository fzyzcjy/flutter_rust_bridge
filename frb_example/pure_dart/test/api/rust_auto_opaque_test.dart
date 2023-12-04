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
      test('cannot call multiple `T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await expectLater(() async {
          return Future.wait([
            futurizeVoidTwinNormal(
                rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100)),
            futurizeVoidTwinNormal(
                rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100)),
          ]);
        }, throwsA(isA<DroppableDisposedException>()));
      });

      test('can call multiple `&T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await Future.wait([
          futurizeVoidTwinNormal(
              rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
          futurizeVoidTwinNormal(
              rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
        ]);
      });

      test('cannot call multiple `&mut T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await expectLater(() async {
          return Future.wait([
            futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
                arg: obj, expect: 100, adder: 1)),
            futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
                arg: obj, expect: 100, adder: 1)),
          ]);
        }, throwsA(isA<DroppableDisposedException>()));
      });

      test('cannot call one `&T` and one `&mut T` concurrently', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await expectLater(() async {
          return Future.wait([
            futurizeVoidTwinNormal(
                rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100)),
            futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
                arg: obj, expect: 100, adder: 1)),
          ]);
        }, throwsA(isA<DroppableDisposedException>()));
      });
    });
  });

  // TODO other apis
}
