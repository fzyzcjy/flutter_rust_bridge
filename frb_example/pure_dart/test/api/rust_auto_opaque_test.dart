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

    test('arg owned', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      await futurizeVoidTwinNormal(
          rustAutoOpaqueArgOwnTwinNormal(arg: obj, expect: 100));
      expect(obj.isDisposed, true);
    });

    test('arg ref', () async {
      final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
      await futurizeVoidTwinNormal(
          rustAutoOpaqueArgBorrowTwinNormal(arg: obj, expect: 100));
      expect(obj.isDisposed, false);
    });

    group('arg ref mut', () {
      test('can be called', () async {
        final obj = await rustAutoOpaqueReturnOwnTwinNormal(initial: 100);
        await futurizeVoidTwinNormal(rustAutoOpaqueArgMutBorrowTwinNormal(
            arg: obj, expect: 100, adder: 1));
        expect(obj.isDisposed, false);
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
  });

  // TODO after `T` call, should auto dispose
  // TODO after `&T`, `&mut T` call, should NOT auto dispose
  // TODO can call multiple `&T` concurrently
  // TODO can NOT call `&T` + `&mut T` concurrently
  // TODO can NOT call multiple `&mut T` concurrently
  // TODO all apis
}
