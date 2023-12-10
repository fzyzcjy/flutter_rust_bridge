import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  // print('Action: Configure tests (before)');
  // test('dart call minimalAdder', () async {
  //   print('Action: Call rust (before)');
  //   expect(await minimalAdder(a: 100, b: 200), 300);
  //   print('Action: Call rust (after)');
  // });
  // print('Action: Configure tests (end)');

  test('hi', () async {
    print('hi dart before rustCallDartSimple');
    await rustCallDartSimple(callback: (x, y) {
      print('hi Dart callback is called: $x, $y');
      return 'this is string returned from dart callback';
    });
    print('hi dart after rustCallDartSimple');
  });
}
