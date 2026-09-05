import 'dart:async';

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
  test('new worker streams deliver the complete initial burst and close', () async {
    for (var iteration = 0; iteration < 1000; iteration++) {
      final values = await immediateStream().toList().timeout(const Duration(seconds: 2));
      expect(values, [0, 1], reason: 'iteration=$iteration');
    }
  });
}
