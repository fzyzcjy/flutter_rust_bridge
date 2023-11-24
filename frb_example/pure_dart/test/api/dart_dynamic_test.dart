import 'package:frb_example_pure_dart/src/rust/api/dart_dynamic.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('dart call return_dart_dynamic', () async {
    final data = await returnDartDynamic();
    expect(data, ['foo']);
  });
}
