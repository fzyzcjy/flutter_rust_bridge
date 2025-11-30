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

  // Oxidized Result tests - async
  test('async Result - success', () async {
    final result = await fallibleDivide(a: 10, b: 2);
    expect(result.isOk(), true);
    expect(result.unwrap(), 5);
  });

  test('async Result - error', () async {
    final result = await fallibleDivide(a: 10, b: 0);
    expect(result.isErr(), true);
    expect(result.unwrapErr().message, 'division by zero');
  });

  // Oxidized Result tests - sync
  test('sync Result - success', () {
    final result = fallibleDivideSync(a: 10, b: 2);
    expect(result.isOk(), true);
    expect(result.unwrap(), 5);
  });

  test('sync Result - error', () {
    final result = fallibleDivideSync(a: 10, b: 0);
    expect(result.isErr(), true);
    expect(result.unwrapErr().message, 'division by zero');
  });
  print('Action: Configure tests (end)');
}
