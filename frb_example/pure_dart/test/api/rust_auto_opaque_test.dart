import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  throw Exception('TODO');
}
