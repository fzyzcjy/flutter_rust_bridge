import 'package:frb_example_pure_dart/src/rust/api/sync.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call simpleAdderSync', () {
    expect(simpleAdderSync(a: 42, b: 100), 142);
  });
}
