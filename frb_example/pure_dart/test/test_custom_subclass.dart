import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await WhatName.init(TODO_custom_subclass);

  test('can use custom subclasses', () async {
    expect(await simpleAdder(a: 1, b: 2), 3);
    expect(logsFromCustomSubclass, TODO);
  });
}
