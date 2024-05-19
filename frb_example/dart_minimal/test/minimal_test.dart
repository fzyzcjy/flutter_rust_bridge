import 'package:frb_example_dart_minimal/src/rust/api/minimal.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  print('Action: Init rust (before)');
  await RustLib.init();
  print('Action: Init rust (after)');

  print('Action: Configure tests (before)');
  test('dart call minimalAdder', () async {
    print('Action: Call rust (before)');
    expect(await minimalAdder(a: 100, b: 200), 300);
    print('Action: Call rust (after)');
  });
  print('Action: Configure tests (end)');

  group('rustCallDartReturnResultTwinNormal', () {
    test('when normal', () async {
      await rustCallDartReturnResultTwinNormal(
          callback: (s) => s * 2, expectOutput: "hihi");
    });

    test('when error', () async {
      await rustCallDartReturnResultTwinNormal(
          callback: (s) => throw Exception('dummy exception'),
          expectOutput: null);
    });
  });
}
