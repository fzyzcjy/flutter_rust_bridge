import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();
  tearDownAll(() {
    // This is the main thing to test in this file
    RustLib.dispose();
  });
  test('dummy (the main thing is not this test, but the dispose)', () {});
}
