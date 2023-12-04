import 'package:frb_example_pure_dart/src/rust/api/rust_auto_opaque.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('simple functions', () async {
    final obj = await rustAutoOpaqueReturnOwnTwinNormal();
    await futurizeVoidTwinNormal(rustAutoOpaqueArgOwnTwinNormal(arg: obj));
    await futurizeVoidTwinNormal(rustAutoOpaqueArgBorrowTwinNormal(arg: obj));
    await futurizeVoidTwinNormal(
        rustAutoOpaqueArgMutBorrowTwinNormal(arg: obj));
  });

  // TODO after `T` call, should auto dispose
  // TODO after `&T`, `&mut T` call, should NOT auto dispose
  // TODO can call multiple `&T` concurrently
  // TODO can NOT call `&T` + `&mut T` concurrently
  // TODO can NOT call multiple `&mut T` concurrently
  // TODO all apis
}
