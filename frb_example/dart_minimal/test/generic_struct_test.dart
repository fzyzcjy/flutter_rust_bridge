import 'dart:async';

import 'package:frb_example_dart_minimal/src/rust/api/generic_struct.dart';
import 'package:frb_example_dart_minimal/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await RustLib.init();

  test('StringContainer round-trip', () async {
    final input = StringContainer(value: 'hello', label: 'greeting');
    final output = await funcStringContainer(arg: input);
    expect(output, input);
  });

  test('IntContainer round-trip', () async {
    final input = IntContainer(value: 42, label: 'answer');
    final output = await funcIntContainer(arg: input);
    expect(output, input);
  });

  test('VisibleStringWrapper round-trip', () async {
    final input = VisibleStringWrapper(inner: 'wrapped');
    final output = await funcVisibleStringWrapper(arg: input);
    expect(output, input);
  });
}
