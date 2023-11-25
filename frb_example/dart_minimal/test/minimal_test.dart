import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call minimalAdder', () async {
    expect(await minimalAdder(a: 100, b: 200), 300);
  });
}
