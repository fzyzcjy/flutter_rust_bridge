import 'package:frb_example_pure_dart/src/rust/api/dart_fn.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('TODO', () async {
    fail('TODO');
  });
}
