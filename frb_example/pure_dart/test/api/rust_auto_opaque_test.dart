import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  // TODO after `T` call, should auto dispose
  // TODO after `&T`, `&mut T` call, should NOT auto dispose
  // TODO can call multiple `&T` concurrently
  // TODO can NOT call `&T` + `&mut T` concurrently
  // TODO can NOT call multiple `&mut T` concurrently
  // TODO all apis
}
