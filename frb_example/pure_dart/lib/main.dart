import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:test/test.dart';

Future<void> main() async {
  await WhatName.init();

  test('dart call simpleAdder', () async {
    expect(await simpleAdder(a: 42, b: 100), 142);
  });

  // TODO
}
